use macroquad::prelude::*;
use nbody_scenarios::solar_like_system;
use nbody_simulation::engine::Simulation;
use nbody_viewer::accumulator::FixedStepAccumulator;
use nbody_viewer::steppable::Steppable;

const PIXELS_PER_UNIT: f32 = 60.0;
const POINT_RADIUS: f32 = 5.0;
const PHYSICS_DT: f64 = 1e-3;
const SIM_SPEED: f64 = 5.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "N-Body Viewer".to_owned(),
        window_width: 900,
        window_height: 700,
        ..Default::default()
    }
}

fn project_to_screen(pos: nbody_simulation::vector3::Vector3) -> (f32, f32) {
    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;
    let x = cx + (pos.x as f32) * PIXELS_PER_UNIT;
    let y = cy + (pos.y as f32) * PIXELS_PER_UNIT;
    (x, y)
}

#[macroquad::main(window_conf)]
async fn main() {
    let bodies = solar_like_system();
    let mut sim = Simulation::new(bodies, 1.0, 1e-4);
    let mut accumulator = FixedStepAccumulator::new(PHYSICS_DT, SIM_SPEED);

    loop {
        accumulator.advance(&mut sim, get_frame_time() as f64);

        let snapshot = Steppable::snapshot(&sim);

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