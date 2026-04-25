use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum VarU32 {
    Fixed { value: u32 },
    Uniform { min: u32, max: u32 },
    Normal { mean: f64, std_dev: f64 },
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum VarF64 {
    Fixed { value: f64 },
    Uniform { min: f64, max: f64 },
    Power { min: f64, max: f64, power: f64 },
    Normal { mean: f64, std_dev: f64 },
}

#[derive(Deserialize, Clone)]
pub struct RangeU32 {
    pub min: u32,
    pub max: u32,
    pub step: u32,
}

impl RangeU32 {
    pub fn iter(&self) -> impl Iterator<Item = u32> {
        (self.min..=self.max).step_by(self.step as usize)
    }
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub runner: RunnerConfig,
    pub experiment: ExperimentConfig,
    pub arena: ArenaConfig,
}

#[derive(Deserialize, Clone)]
pub struct RunnerConfig {
    pub runs_per_config: usize,
    pub parallel_jobs: usize,
    pub output_dir: String,
}

#[derive(Deserialize, Clone)]
pub struct ExperimentConfig {
    pub algorithms: Vec<String>,
    pub length: u32,
    pub ticks_per_second: u32,
    pub robots: RangeU32,
    pub arena_size: VarF64,
    pub seed: VarU32,
}

#[derive(Deserialize, Clone)]
pub struct ArenaConfig {
    pub arena_type: Vec<String>,
    pub maze_width: VarU32,
    pub maze_height: VarU32,
    pub scatter_density: VarF64,
    pub scatter_size: VarF64,
}

#[derive(Serialize)]
pub struct ArenaInfo {
    pub walls: Vec<WallInfo>,
    pub target: TargetInfo,
    pub start: StartInfo,
}

#[derive(Serialize, Clone)]
pub struct WallInfo {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub sx: f64,
    pub sy: f64,
    pub yaw: f64,
}

#[derive(Serialize)]
pub struct TargetInfo {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub color: String,
}

#[derive(Serialize)]
pub struct StartInfo {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

#[derive(Serialize)]
pub struct RunConfig {
    pub algorithm: String,
    pub length: u32,
    pub ticks_per_second: u32,
    pub total_ticks: u32,
    pub robots: u32,
    pub arena_size: f64,
    pub seed: u32,
    pub arena_type: String,
    pub maze_width: usize,
    pub maze_height: usize,
    pub scatter_size: f64,
    pub scatter_density: f64,
}
