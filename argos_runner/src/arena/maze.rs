use rand::seq::SliceRandom;

use crate::config::WallInfo;

#[derive(Clone)]
struct Cell {
    n: bool,
    w: bool,
    visited: bool,
}

pub fn generate_maze(width: usize, height: usize, arena_size: f64) -> Vec<WallInfo> {
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
    let mut walls = Vec::new();
    let mut obs_idx = 0;

    for y in 0..height {
        for x in 0..width {
            let cx = -arena_size / 2.0 + (x as f64 * cell_w) + (cell_w / 2.0);
            let cy = arena_size / 2.0 - (y as f64 * cell_h) - (cell_h / 2.0);

            if grid[y][x].n && y != 0 {
                walls.push(WallInfo {
                    id: format!("maze_n_{}", obs_idx),
                    x: cx,
                    y: cy + cell_h / 2.0,
                    sx: cell_w + wall_thickness,
                    sy: wall_thickness,
                    yaw: 0.0,
                });
                obs_idx += 1;
            }
            if grid[y][x].w && x != 0 {
                walls.push(WallInfo {
                    id: format!("maze_w_{}", obs_idx),
                    x: cx - cell_w / 2.0,
                    y: cy,
                    sx: wall_thickness,
                    sy: cell_h + wall_thickness,
                    yaw: 0.0,
                });
                obs_idx += 1;
            }
        }
    }

    walls
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
