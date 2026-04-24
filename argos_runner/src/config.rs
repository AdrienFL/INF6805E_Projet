use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub(crate) enum VarU32 {
    Fixed { value: u32 },
    Uniform { min: u32, max: u32 },
    Normal { mean: f64, std_dev: f64 },
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub(crate) enum VarF64 {
    Fixed { value: f64 },
    Uniform { min: f64, max: f64 },
    Normal { mean: f64, std_dev: f64 },
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct Config {
    pub(crate) runner: RunnerConfig,
    pub(crate) experiment: ExperimentConfig,
    pub(crate) arena: ArenaConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct RunnerConfig {
    pub(crate) runs: usize,
    pub(crate) parallel_jobs: usize,
    pub(crate) output_dir: String,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct ExperimentConfig {
    pub(crate) length: u32,
    pub(crate) ticks_per_second: u32,
    pub(crate) robots: VarU32,
    pub(crate) arena_size: VarF64,
    pub(crate) seed: VarU32,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct ArenaConfig {
    pub(crate) arena_type: Vec<String>,
    pub(crate) maze_width: VarU32,
    pub(crate) maze_height: VarU32,
    pub(crate) scatter_obstacles: VarU32,
}

#[derive(Debug, Serialize)]
pub(crate) struct RunConfig {
    pub(crate) length: u32,
    pub(crate) ticks_per_second: u32,
    pub(crate) robots: u32,
    pub(crate) arena_size: f64,
    pub(crate) seed: u32,
    pub(crate) arena_type: String,
    pub(crate) maze_width: usize,
    pub(crate) maze_height: usize,
    pub(crate) scatter_obstacles: usize,
}
