use rand::RngExt as _;

use crate::config::WallInfo;

pub fn generate_scatter(n_obstacles: usize, arena_size: f64) -> Vec<WallInfo> {
    let mut rng = rand::rng();
    let half = arena_size / 2.0;
    let margin = 0.5;
    let mut placed: Vec<(f64, f64)> = Vec::new();
    let mut walls = Vec::new();

    let start_min_x = -half;
    let start_max_x = -half + 1.0;
    let start_min_y = half - 1.0;
    let start_max_y = half;

    let target_x = half - 0.5;
    let target_y = -half + 0.5;
    let clearance = 0.5;

    for i in 0..n_obstacles {
        for _ in 0..100 {
            let x = rng.random_range((-half + margin)..(half - margin));
            let y = rng.random_range((-half + margin)..(half - margin));

            if x > (start_min_x - clearance)
                && x < (start_max_x + clearance)
                && y > (start_min_y - clearance)
                && y < (start_max_y + clearance)
            {
                continue;
            }

            if (x - target_x).powi(2) + (y - target_y).powi(2) < 1.0 {
                continue;
            }

            if !placed
                .iter()
                .any(|(px, py)| (x - px).powi(2) + (y - py).powi(2) <= 0.4)
            {
                placed.push((x, y));
                walls.push(WallInfo {
                    id: format!("obs_{}", i),
                    x,
                    y,
                    sx: rng.random_range(0.1..0.4),
                    sy: rng.random_range(0.1..0.4),
                    yaw: rng.random_range(0.0..360.0),
                });
                break;
            }
        }
    }
    walls
}

// pub fn generate_scatter(n_obstacles: usize, arena_size: f64) -> String {
//     let mut rng = rand::rngs::ThreadRng::default();
//     let half = arena_size / 2.0;
//     let margin = 0.5;
//     let mut placed: Vec<(f64, f64)> = Vec::new();
//     let mut xml = Vec::new();

//     for i in 0..n_obstacles {
//         for _ in 0..100 {
//             let x = rng.random_range((-half + margin)..(half - margin));
//             let y = rng.random_range((-half + margin)..(half - margin));

//             if x.abs() < 0.5 && y.abs() < 0.5 {
//                 continue;
//             }

//             let collision = placed
//                 .iter()
//                 .any(|(px, py)| (x - px).powi(2) + (y - py).powi(2) <= 0.4);

//             if !collision {
//                 placed.push((x, y));
//                 let angle = rng.random_range(0.0..360.0);
//                 let w = rng.random_range(0.1..0.4);
//                 let l = rng.random_range(0.1..0.4);
//                 xml.push(format!(
//                     r#"<box id="obs_{}" size="{:.2},{:.2},0.5" movable="false"><body position="{:.3},{:.3},0" orientation="{:.1},0,0"/></box>"#,
//                     i, l, w, x, y, angle
//                 ));
//                 break;
//             }
//         }
//     }
//     xml.join("\n")
// }
