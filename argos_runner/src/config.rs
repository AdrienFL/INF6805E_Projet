use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) runner: RunnerConfig,
    pub(crate) experiment: ExperimentConfig,
    pub(crate) arena: ArenaConfig,
}

#[derive(Deserialize)]
pub(crate) struct RunnerConfig {
    pub(crate) runs: usize,
    pub(crate) parallel_jobs: usize,
    pub(crate) base_seed: u32,
    pub(crate) output_dir: String,
}

#[derive(Deserialize)]
pub struct ExperimentConfig {
    pub(crate) length: u32,
    pub(crate) ticks_per_second: u32,
    pub(crate) robots: u32,
    pub(crate) arena_size: f64,
}

#[derive(Deserialize)]
pub struct ArenaConfig {
    #[serde(rename = "type")]
    pub(crate) arena_type: String,
    pub(crate) maze_width: usize,
    pub(crate) maze_height: usize,
    pub(crate) scatter_obstacles: usize,
}
