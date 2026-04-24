use crate::{
    arena::{maze::generate_maze, scatter::generate_scatter},
    config::RunConfig,
};

pub fn generate_argos_xml(config: &RunConfig) -> String {
    let half = config.arena_size / 2.0;

    let inner_obstacles = match config.arena_type.as_str() {
        "maze" => generate_maze(config.maze_width, config.maze_height, config.arena_size),
        _ => generate_scatter(config.scatter_obstacles, config.arena_size),
    };

    format!(
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
      <params bytecode_file="scripts/projet/projet.bo" debug_file="scripts/projet/projet.bdb">
        <wheel_turning hard_turn_angle_threshold="90" soft_turn_angle_threshold="70" no_turn_angle_threshold="10" max_speed="10" />
      </params>
    </buzz_controller_kheperaiv>
  </controllers>
  <arena size="{}, {}, 1" center="0,0,0.5">
    <box id="wall_n" size="{},0.1,0.5" movable="false"><body position="0,{},0" orientation="0,0,0"/></box>
    <box id="wall_s" size="{},0.1,0.5" movable="false"><body position="0,-{},0" orientation="0,0,0"/></box>
    <box id="wall_e" size="0.1,{},0.5" movable="false"><body position="{},0,0" orientation="0,0,0"/></box>
    <box id="wall_w" size="0.1,{},0.5" movable="false"><body position="-{},0,0" orientation="0,0,0"/></box>
    {}
    <light id="light_1" position="4.5,-4.5,0.7" orientation="0,0,0" color="yellow" intensity="5" medium="leds" />
    <distribute>
      <position method="uniform" min="-5,4,0" max="-4,5,0" />
      <orientation method="gaussian" mean="0,0,0" std_dev="360,0,0" />
      <entity quantity="{}" max_trials="100">
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
        config.arena_size,
        config.arena_size,
        config.arena_size,
        half,
        config.arena_size,
        half,
        config.arena_size,
        half,
        config.arena_size,
        half,
        inner_obstacles,
        config.robots
    )
}
