use rand::RngExt as _;

pub fn generate_scatter(n_obstacles: usize, arena_size: f64) -> String {
    let mut rng = rand::rngs::ThreadRng::default();
    let half = arena_size / 2.0;
    let margin = 0.5;
    let mut placed: Vec<(f64, f64)> = Vec::new();
    let mut xml = Vec::new();

    for i in 0..n_obstacles {
        for _ in 0..100 {
            let x = rng.random_range((-half + margin)..(half - margin));
            let y = rng.random_range((-half + margin)..(half - margin));

            if x.abs() < 0.5 && y.abs() < 0.5 {
                continue;
            }

            let collision = placed
                .iter()
                .any(|(px, py)| (x - px).powi(2) + (y - py).powi(2) <= 0.4);

            if !collision {
                placed.push((x, y));
                let angle = rng.random_range(0.0..360.0);
                let w = rng.random_range(0.1..0.4);
                let l = rng.random_range(0.1..0.4);
                xml.push(format!(
                    r#"<box id="obs_{}" size="{:.2},{:.2},0.5" movable="false"><body position="{:.3},{:.3},0" orientation="{:.1},0,0"/></box>"#,
                    i, l, w, x, y, angle
                ));
                break;
            }
        }
    }
    xml.join("\n")
}
