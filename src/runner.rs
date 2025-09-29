use std::simd::{LaneCount, Simd, StdFloat, SupportedLaneCount, num::SimdFloat};

use glutin_window::{GlutinWindow as Window, OpenGL};
use opengl_graphics::GlGraphics;
use piston::{Event, EventSettings, Events, RenderEvent, UpdateEvent, WindowSettings};

use crate::{drawing::App, sim::Bodies};

pub fn run<const N: usize>(sim: Bodies<N>)
where
    LaneCount<N>: SupportedLaneCount,
{
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        sim,
        radii: cbrt(sim.m),
        scale: 1.0,
        speed: 1.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(args);
        }

        if let Event::Input(input, _) = e {
            app.handle(&input);
        }
    }
}

fn cbrt<const N: usize>(s: Simd<f64, N>) -> Simd<f64, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    let logs = s.abs().ln();
    let cbrtlogs = logs / Simd::splat(3.0);
    let cbrts = cbrtlogs.exp();
    cbrts * s / s.abs()
}
