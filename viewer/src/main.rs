use macroquad::prelude::*;
use nbody_scenarios::solar_like_system;
use nbody_simulation::engine::Simulation;

/// Scale factor: how many pixels represent one unit of physical distance.
/// Purely visual, does not affect the simulation itself.
const PIXELS_PER_UNIT: f32 = 60.0;

/// Radius (in pixels) used to draw each body.
/// Fixed for now — could depend on mass later.
const POINT_RADIUS: f32 = 5.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "N-Body Viewer".to_owned(),
        window_width: 900,
        window_height: 700,
        ..Default::default()
    }
}

/// Projects a physical 3D position (f64) to a 2D screen coordinate (f32),
/// ignoring the z axis for now (simple XY projection) and centering the
/// physical origin at the middle of the window.
fn project_to_screen(pos: nbody_simulation::vector3::Vector3) -> (f32, f32) {
    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;
    let x = cx + (pos.x as f32) * PIXELS_PER_UNIT;
    let y = cy + (pos.y as f32) * PIXELS_PER_UNIT;
    (x, y)
}

#[macroquad::main(window_conf)]
async fn main() {
    // Step 3: just a static snapshot for now, no step() yet.
    // The fixed-timestep accumulator (step 4) will replace this single
    // snapshot with a real physics loop.
    let bodies = solar_like_system();
    let sim = Simulation::new(bodies, 1.0, 1e-4);
    let snapshot = sim.snapshot();

    loop {
        clear_background(BLACK);

        for pos in &snapshot.positions {
            let (x, y) = project_to_screen(*pos);
            draw_circle(x, y, POINT_RADIUS, WHITE);
        }

        draw_text(
            &format!("t = {:.2}", snapshot.time),
            20.0,
            30.0,
            24.0,
            GRAY,
        );

        next_frame().await;
    }
}