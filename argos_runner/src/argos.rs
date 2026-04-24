use crate::{
    arena::{maze::generate_maze, scatter::generate_scatter},
    config::{ArenaInfo, RunConfig, StartInfo, TargetInfo, WallInfo},
};

pub fn generate_argos_data(config: &RunConfig) -> (String, ArenaInfo) {
    let half = config.arena_size / 2.0;

    let mut walls = vec![
        WallInfo {
            id: "wall_n".to_string(),
            x: 0.0,
            y: half,
            sx: config.arena_size,
            sy: 0.1,
            yaw: 0.0,
        },
        WallInfo {
            id: "wall_s".to_string(),
            x: 0.0,
            y: -half,
            sx: config.arena_size,
            sy: 0.1,
            yaw: 0.0,
        },
        WallInfo {
            id: "wall_e".to_string(),
            x: half,
            y: 0.0,
            sx: 0.1,
            sy: config.arena_size,
            yaw: 0.0,
        },
        WallInfo {
            id: "wall_w".to_string(),
            x: -half,
            y: 0.0,
            sx: 0.1,
            sy: config.arena_size,
            yaw: 0.0,
        },
    ];

    let inner_walls = match config.arena_type.as_str() {
        "maze" => generate_maze(config.maze_width, config.maze_height, config.arena_size),
        _ => generate_scatter(config.scatter_obstacles, config.arena_size),
    };
    walls.extend(inner_walls);

    let arena_info = ArenaInfo {
        walls: walls.clone(),
        target: TargetInfo {
            id: "light_1".to_string(),
            x: half - 0.5,
            y: -half + 0.5,
            color: "yellow".to_string(),
        },
        start: StartInfo {
            min_x: -half,
            max_x: -half + 1.0,
            min_y: half - 1.0,
            max_y: half,
        },
    };

    let boxes_xml = walls.iter().map(|w| {
            format!(
                r#"<box id="{}" size="{:.3},{:.3},0.5" movable="false"><body position="{:.3},{:.3},0" orientation="{:.1},0,0"/></box>"#,
                w.id, w.sx, w.sy, w.x, w.y, w.yaw
            )
        }).collect::<Vec<_>>().join("\n    ");

    let xml = format!(
        r#"<?xml version="1.0" ?>
    <argos-configuration>
      <framework>
        <system threads="0"/>
        <experiment length="{}" ticks_per_second="{}" random_seed="{}"/>
      </framework>
      <controllers>
        <buzz_controller_kheperaiv id="projet">
          <actuators>
            <differential_steering implementation="default" />
            <leds implementation="default" medium="leds" />
            <range_and_bearing implementation="default" />
          </actuators>
          <sensors>
            <kheperaiv_proximity implementation="default" show_rays="true" />
            <range_and_bearing implementation="medium" medium="rab" show_rays="false" noise_std_dev="0" />
            <positioning implementation="default" />
            <kheperaiv_light implementation="rot_z_only" show_rays="false" />
          </sensors>
          <params bytecode_file="scripts/{algo}/{algo}.bo" debug_file="scripts/{algo}/{algo}.bdb">
            <wheel_turning hard_turn_angle_threshold="90" soft_turn_angle_threshold="70" no_turn_angle_threshold="10" max_speed="10" />
          </params>
        </buzz_controller_kheperaiv>
      </controllers>
      <arena size="{size}, {size}, 1" center="0,0,0.5">
        {boxes}
        <light id="{tid}" position="{tx},{ty},0.7" orientation="0,0,0" color="{tcolor}" intensity="5" medium="leds" />
        <distribute>
          <position method="uniform" min="{sminx},{sminy},0" max="{smaxx},{smaxy},0" />
          <orientation method="gaussian" mean="0,0,0" std_dev="360,0,0" />
          <entity quantity="{robots}" max_trials="100">
            <kheperaiv id="kiv" rab_data_size="200" rab_range="6">
              <controller config="projet" />
            </kheperaiv>
          </entity>
        </distribute>
      </arena>
      <physics_engines><dynamics2d id="dyn2d"/></physics_engines>
      <media><led id="leds"/><range_and_bearing id="rab" /></media>
    </argos-configuration>"#,
        config.length,
        config.ticks_per_second,
        config.seed,
        algo = config.algorithm,
        size = config.arena_size,
        boxes = boxes_xml,
        tid = arena_info.target.id,
        tx = arena_info.target.x,
        ty = arena_info.target.y,
        tcolor = arena_info.target.color,
        sminx = arena_info.start.min_x,
        sminy = arena_info.start.min_y,
        smaxx = arena_info.start.max_x,
        smaxy = arena_info.start.max_y,
        robots = config.robots
    );

    (xml, arena_info)
}
