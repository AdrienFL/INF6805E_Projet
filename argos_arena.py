import random
import xml.etree.ElementTree as ET

def generate_argos(n_obstacles=15, n_robots=10, arena_size=5.0, seed=42):
    random.seed(seed)
    
    lines = []
    lines.append('<?xml version="1.0" ?>')
    lines.append('<argos-configuration>')
    
    lines.append('''
  <framework>
    <system threads="0"/>
    <experiment length="0" ticks_per_second="10" random_seed="{}"/>
  </framework>'''.format(seed))

    lines.append('''
  <controllers>
    <buzz_controller_kheperaiv id="projet">
      <actuators>
        <differential_steering implementation="default" />
        <leds                  implementation="default" medium="leds" />
        <range_and_bearing     implementation="default" />
      </actuators>
      <sensors>
        <kheperaiv_proximity implementation="default" show_rays="true" />
        <range_and_bearing   implementation="medium" medium="rab"
                             show_rays="false" noise_std_dev="0" />
        <positioning         implementation="default" />
        <kheperaiv_light      implementation="rot_z_only" show_rays="false" />
      </sensors>
        <params>
        <wheel_turning hard_turn_angle_threshold="90"
                       soft_turn_angle_threshold="70"
                       no_turn_angle_threshold="10"
                       max_speed="10" />
      </params>
    </buzz_controller_kheperaiv>
  </controllers>''')

    half = arena_size / 2
    lines.append(f'''
  <arena size="{arena_size}, {arena_size}, 1" center="0,0,0.5">
    <box id="wall_n" size="{arena_size},0.1,0.5" movable="false">
      <body position="0,{half},0" orientation="0,0,0"/>
    </box>
    <box id="wall_s" size="{arena_size},0.1,0.5" movable="false">
      <body position="0,{-half},0" orientation="0,0,0"/>
    </box>
    <box id="wall_e" size="0.1,{arena_size},0.5" movable="false">
      <body position="{half},0,0" orientation="0,0,0"/>
    </box>
    <box id="wall_w" size="0.1,{arena_size},0.5" movable="false">
      <body position="{-half},0,0" orientation="0,0,0"/>
    </box>''')


    margin = 0.5 
    placed = []
    for i in range(n_obstacles + 1):
        for _ in range(100):  
            x = random.uniform(-half + margin, half - margin)
            y = random.uniform(-half + margin, half - margin)

            if abs(x) < 0.5 and abs(y) < 0.5:
                continue
            if all((x-px)**2 + (y-py)**2 > 0.4 for px,py in placed):
                placed.append((x, y))
                break
        ox, oy = placed[-1]
        if i == n_obstacles:
            lines.append(f'''
            <light id="light_1"
           position="4.5,-4.5,0.3"
           orientation="0,0,0"
           color="yellow"
           intensity="5"
           medium="leds" />''')
        else:
            angle = random.uniform(0, 360)
            w = random.uniform(0.1, 0.4)
            l = random.uniform(0.1, 0.4)
            lines.append(f'''
        <box id="obs_{i}" size="{l:.2f},{w:.2f},0.5" movable="false">
        <body position="{ox:.3f},{oy:.3f},0" orientation="{angle:.1f},0,0"/>
        </box>''')


    lines.append(f'''
       <distribute>
      <position method="uniform" min="2,2,0" max="3,3,0" />
      <orientation method="gaussian" mean="0,0,0" std_dev="360,0,0" />
      <entity quantity="{n_robots}" max_trials="100">
        <kheperaiv id="kiv" rab_data_size="200" rab_range="6">
          <controller config="projet" />
        </kheperaiv>
      </entity>
    </distribute>''')

    lines.append('''
  </arena>''')

    lines.append('''
  <physics_engines>
    <dynamics2d id="dyn2d"/>
  </physics_engines>

    <media>
    <led id="leds"/>
    <range_and_bearing id="rab" />
    </media>

    <visualization>
    <qt-opengl>
      <camera>
        <placements>
          <placement index="0" position="0,0,30" look_at="0,0,0" up="0,1,0" lens_focal_length="80" />
        </placements>
      </camera>
      <user_functions label="buzz_qt" />
    </qt-opengl>
  </visualization>''')

    lines.append('</argos-configuration>')
    return '\n'.join(lines)

if __name__ == '__main__':
    content = generate_argos(n_obstacles=40, n_robots=10, arena_size=10.0)
    with open('projet.argos', 'w') as f:
        f.write(content)
    print("generated argos file")