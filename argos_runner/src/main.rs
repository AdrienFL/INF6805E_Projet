use indicatif::{ProgressBar, ProgressStyle};
use log::error;
use rand::{Rng, seq::IndexedRandom as _};
use rand_distr::{Distribution, Normal, Uniform};
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
    argos::generate_argos_xml,
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
        VarF64::Normal { mean, std_dev } => Normal::new(*mean, *std_dev).unwrap().sample(rng),
    }
}

fn execute_run(run_config: &RunConfig, run_idx: usize, ansi_regex: &Regex, output_dir: &str) {
    let run_dir = PathBuf::from(output_dir).join(format!("run_{}", run_idx));
    fs::create_dir_all(&run_dir).unwrap();

    let json_file = run_dir.join("experiment_info.json");
    let json_content = serde_json::to_string_pretty(run_config).unwrap();
    fs::write(&json_file, json_content).unwrap();

    let argos_file = run_dir.join("projet.argos");
    let xml_content = generate_argos_xml(run_config);
    fs::write(&argos_file, xml_content).unwrap();

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

    let mut process_line = |line: String| {
        if line.contains("BUZZ:") {
            let clean_line = ansi_regex.replace_all(&line, "");
            if let Some(idx) = clean_line.find("BUZZ:") {
                let content = &clean_line[idx + 5..];
                let no_spaces: String = content.chars().filter(|c| !c.is_whitespace()).collect();
                if no_spaces.starts_with("MAP,") {
                    writeln!(map_file, "{}", no_spaces).unwrap();
                } else if !no_spaces.is_empty() {
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

    let mut run_configs = Vec::with_capacity(config.runner.runs);
    for _ in 0..config.runner.runs {
        run_configs.push(RunConfig {
            length: config.experiment.length,
            ticks_per_second: config.experiment.ticks_per_second,
            robots: sample_u32(&mut rng, &config.experiment.robots),
            arena_size: sample_f64(&mut rng, &config.experiment.arena_size),
            seed: sample_u32(&mut rng, &config.experiment.seed),
            arena_type: config.arena.arena_type.choose(&mut rng).unwrap().clone(),
            maze_width: sample_u32(&mut rng, &config.arena.maze_width) as usize,
            maze_height: sample_u32(&mut rng, &config.arena.maze_height) as usize,
            scatter_obstacles: sample_u32(&mut rng, &config.arena.scatter_obstacles) as usize,
        });
    }

    let pb = ProgressBar::new(config.runner.runs as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} runs ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    run_configs
        .into_par_iter()
        .enumerate()
        .for_each(|(i, run_config)| {
            execute_run(&run_config, i, &ansi_regex, &config.runner.output_dir);
            pb.inc(1);
        });

    pb.finish_with_message("All experiments completed.");
}
