use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::error;
use rand::{Rng, seq::SliceRandom as _};
use rand_distr::{Distribution, Normal, Uniform, num_traits::Pow};
use rayon::prelude::*;
use regex::Regex;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

mod arena;
mod argos;
mod config;

use crate::{
    argos::generate_argos_data,
    config::{Config, RunConfig, VarF64, VarU32},
};

fn sample_u32(rng: &mut impl Rng, var: &VarU32) -> u32 {
    match var {
        VarU32::Fixed { value } => *value,
        VarU32::Uniform { min, max } => Uniform::new_inclusive(*min, *max).unwrap().sample(rng),
        VarU32::Normal { mean, std_dev } => Normal::new(*mean, *std_dev)
            .unwrap()
            .sample(rng)
            .round()
            .max(0.0) as u32,
    }
}

fn sample_f64(rng: &mut impl Rng, var: &VarF64) -> f64 {
    match var {
        VarF64::Fixed { value } => *value,
        VarF64::Uniform { min, max } => Uniform::new_inclusive(*min, *max).unwrap().sample(rng),
        VarF64::Power { min, max, power } => Uniform::new_inclusive(*min, *max)
            .unwrap()
            .sample(rng)
            .pow(power),
        VarF64::Normal { mean, std_dev } => Normal::new(*mean, *std_dev).unwrap().sample(rng),
    }
}

fn execute_run(
    run_config: &RunConfig,
    run_idx: usize,
    ansi_regex: &Regex,
    output_dir: &str,
    mp: &MultiProgress,
    main_pb: &ProgressBar,
    _total_runs: usize,
) {
    let run_dir = PathBuf::from(output_dir).join(format!("run_{}", run_idx));
    fs::create_dir_all(&run_dir).unwrap();

    let json_file = run_dir.join("experiment_info.json");
    let json_content = serde_json::to_string_pretty(run_config).unwrap();
    fs::write(&json_file, json_content).unwrap();

    let (xml_content, arena_info) = generate_argos_data(run_config);

    let argos_file = run_dir.join("projet.argos");
    fs::write(&argos_file, xml_content).unwrap();

    let arena_json_file = run_dir.join("arena.json");
    let arena_json_content = serde_json::to_string_pretty(&arena_info).unwrap();
    fs::write(&arena_json_file, arena_json_content).unwrap();

    let mut child = Command::new("argos3")
        .arg("-z")
        .arg("-c")
        .arg(&argos_file)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start argos3");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let mut map_file = File::create(run_dir.join("experiment_map.csv")).unwrap();
    let mut data_file = File::create(run_dir.join("experiment_data.csv")).unwrap();

    let run_pb = mp.add(ProgressBar::new(run_config.total_ticks as u64));
    run_pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>4}/{len:4} - {msg:>}",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    run_pb.set_message(format!(
        "Algorithm {}, {} agents, scatter_density {:.2}, scatter_size {:.2}",
        &run_config.algorithm,
        run_config.robots,
        run_config.scatter_density,
        run_config.scatter_size
    ));

    let mut last_tick: u64 = 0;
    let mut local_tick_accum: u64 = 0;

    let mut process_line = |line: String| {
        if line.contains("BUZZ:") {
            let clean_line = ansi_regex.replace_all(&line, "");
            if let Some(idx) = clean_line.find("BUZZ:") {
                let content = &clean_line[idx + 5..];
                let no_spaces: String = content.chars().filter(|c| !c.is_whitespace()).collect();

                if no_spaces.starts_with("MAP,") {
                    writeln!(map_file, "{}", no_spaces).unwrap();
                } else if !no_spaces.is_empty() {
                    let parts: Vec<&str> = no_spaces.split(',').collect();
                    if parts.len() >= 2 {
                        if let Ok(tick) = parts[1].parse::<u64>() {
                            if tick > last_tick && tick <= run_config.total_ticks as u64 {
                                local_tick_accum += tick - last_tick;
                                last_tick = tick;

                                if local_tick_accum >= 50 {
                                    run_pb.inc(local_tick_accum);
                                    main_pb.inc(local_tick_accum);
                                    local_tick_accum = 0;
                                }
                            }
                        }
                    }
                    writeln!(data_file, "{}", no_spaces).unwrap();
                }
            }
        }
    };

    let out_reader = BufReader::new(stdout);
    for line in out_reader.lines().flatten() {
        process_line(line);
    }

    let mut err_reader = BufReader::new(stderr);
    let res = child.wait().unwrap();

    let mut err_output = String::new();
    err_reader.read_to_string(&mut err_output).unwrap();
    if !res.success() {
        error!("Run {} failed: {}\n{}", run_idx, res, err_output);
    }

    if local_tick_accum > 0 {
        run_pb.inc(local_tick_accum);
        main_pb.inc(local_tick_accum);
    }

    if (run_config.length as u64) > last_tick {
        let diff = (run_config.length as u64) - last_tick;
        run_pb.inc(diff);
        main_pb.inc(diff);
    }

    run_pb.finish_and_clear();
}

fn main() {
    env_logger::init();
    let config_str = fs::read_to_string("config.toml").expect("Missing config.toml");
    let config: Config = toml::from_str(&config_str).unwrap();

    fs::create_dir_all(&config.runner.output_dir).unwrap();

    rayon::ThreadPoolBuilder::new()
        .num_threads(config.runner.parallel_jobs)
        .build_global()
        .unwrap();

    let ansi_regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    let mut rng = rand::rng();

    let mut run_configs = Vec::new();
    let mut total_global_ticks: u64 = 0;

    for algorithm in &config.experiment.algorithms {
        for arena_type in &config.arena.arena_type {
            for robots in config.experiment.robots.iter() {
                for _ in 0..config.runner.runs_per_config {
                    let length = config.experiment.length;
                    let ticks_per_second = config.experiment.ticks_per_second;
                    let total_ticks = length * ticks_per_second;
                    total_global_ticks += total_ticks as u64;

                    run_configs.push(RunConfig {
                        algorithm: algorithm.clone(),
                        length,
                        ticks_per_second,
                        total_ticks,
                        robots,
                        arena_size: sample_f64(&mut rng, &config.experiment.arena_size),
                        seed: sample_u32(&mut rng, &config.experiment.seed),
                        arena_type: arena_type.clone(),
                        maze_width: sample_u32(&mut rng, &config.arena.maze_width) as usize,
                        maze_height: sample_u32(&mut rng, &config.arena.maze_height) as usize,
                        scatter_size: sample_f64(&mut rng, &config.arena.scatter_size),
                        scatter_density: sample_f64(&mut rng, &config.arena.scatter_density),
                    });
                }
            }
        }
    }

    run_configs.shuffle(&mut rng);

    let mp = MultiProgress::new();
    let main_pb = mp.add(ProgressBar::new(total_global_ticks));
    main_pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] OVERALL [{wide_bar:.magenta/blue}] {pos}/{len} ticks ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    main_pb.tick();

    let total_runs = run_configs.len();

    run_configs
        .into_par_iter()
        .enumerate()
        .for_each(|(i, run_config)| {
            execute_run(
                &run_config,
                i,
                &ansi_regex,
                &config.runner.output_dir,
                &mp,
                &main_pb,
                total_runs,
            );
        });

    main_pb.finish_with_message("All experiments completed.");
}
