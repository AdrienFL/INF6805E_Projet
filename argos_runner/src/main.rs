use log::{error, trace};
use rayon::prelude::*;

mod arena;
mod argos;
mod config;

use regex::Regex;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    sync::Arc,
};

use crate::{argos::generate_argos_xml, config::Config};

fn execute_run(config: &Config, run_idx: usize, ansi_regex: &Regex) {
    let seed = config.runner.base_seed + run_idx as u32;
    let run_dir = PathBuf::from(&config.runner.output_dir).join(format!("run_{}", run_idx));
    fs::create_dir_all(&run_dir).unwrap();

    let argos_file = run_dir.join("projet.argos");
    let xml_content = generate_argos_xml(config, seed);
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
    // for line in err_reader.lines().flatten() {
    //     process_line(line);
    // }

    let res = child.wait().unwrap();
    let mut toto = String::default();
    err_reader.read_to_string(&mut toto).unwrap();
    if !res.success() {
        error!("{}: {}", res, &toto);
    }
}

fn main() {
    env_logger::init();
    let config_str = fs::read_to_string("config.toml").expect("Missing config.toml");
    let config: Arc<Config> = Arc::new(toml::from_str(&config_str).unwrap());

    fs::create_dir_all(&config.runner.output_dir).unwrap();

    rayon::ThreadPoolBuilder::new()
        .num_threads(config.runner.parallel_jobs)
        .build_global()
        .unwrap();

    let ansi_regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();

    (0..config.runner.runs).into_par_iter().for_each(|i| {
        execute_run(&config, i, &ansi_regex);
        trace!("Completed run {}/{}", i + 1, config.runner.runs);
    });
}
