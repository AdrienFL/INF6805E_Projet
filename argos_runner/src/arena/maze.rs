use rand::seq::SliceRandom;

#[derive(Clone)]
struct Cell {
    n: bool,
    w: bool,
    visited: bool,
}

pub fn generate_maze(width: usize, height: usize, arena_size: f64) -> String {
    let mut grid = vec![
        vec![
            Cell {
                n: true,
                w: true,
                visited: false
            };
            width
        ];
        height
    ];

    carve_passages(0, 0, &mut grid, width, height);

    let cell_w = arena_size / width as f64;
    let cell_h = arena_size / height as f64;
    let wall_thickness = 0.1;
    let mut xml = Vec::new();
    let mut obs_idx = 0;

    for y in 0..height {
        for x in 0..width {
            let cx = -arena_size / 2.0 + (x as f64 * cell_w) + (cell_w / 2.0);
            let cy = arena_size / 2.0 - (y as f64 * cell_h) - (cell_h / 2.0);

            if grid[y][x].n && y != 0 {
                xml.push(format!(
                    r#"<box id="maze_n_{}" size="{},{},0.5" movable="false"><body position="{},{},0" orientation="0,0,0"/></box>"#,
                    obs_idx,
                    cell_w + wall_thickness,
                    wall_thickness,
                    cx,
                    cy + cell_h / 2.0
                ));
                obs_idx += 1;
            }
            if grid[y][x].w && x != 0 {
                xml.push(format!(
                    r#"<box id="maze_w_{}" size="{},{},0.5" movable="false"><body position="{},{},0" orientation="0,0,0"/></box>"#,
                    obs_idx,
                    wall_thickness,
                    cell_h + wall_thickness,
                    cx - cell_w / 2.0,
                    cy
                ));
                obs_idx += 1;
            }
        }
    }
    xml.join("\n")
}

fn carve_passages(cx: i32, cy: i32, grid: &mut Vec<Vec<Cell>>, width: usize, height: usize) {
    grid[cy as usize][cx as usize].visited = true;
    let mut directions = vec![('N', 0, -1), ('S', 0, 1), ('E', 1, 0), ('W', -1, 0)];
    let mut thread_rng = rand::rngs::ThreadRng::default();
    directions.shuffle(&mut thread_rng);

    for (dir, dx, dy) in directions {
        let nx = cx + dx;
        let ny = cy + dy;

        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
            if !grid[ny as usize][nx as usize].visited {
                match dir {
                    'N' => grid[cy as usize][cx as usize].n = false,
                    'W' => grid[cy as usize][cx as usize].w = false,
                    'S' => grid[ny as usize][nx as usize].n = false,
                    'E' => grid[ny as usize][nx as usize].w = false,
                    _ => {}
                }
                carve_passages(nx, ny, grid, width, height);
            }
        }
    }
}
