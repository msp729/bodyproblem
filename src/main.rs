#![feature(portable_simd)]
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::simd::num::SimdFloat;
use std::simd::{LaneCount, Simd, StdFloat, SupportedLaneCount};

use drawing::App;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Event, Input};
use sim::Bodies;

mod drawing;
mod rk4;
mod sim;

fn cbrt<const N: usize>(s: Simd<f64, N>) -> Simd<f64, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    let logs = s.abs().ln();
    let cbrtlogs = logs / Simd::splat(3.0);
    let cbrts = cbrtlogs.exp();
    let signedcbrt = cbrts * s / s.abs();
    signedcbrt
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let masses = Simd::from([1.0, 1.0, 1.0]);

    // Create an App
    let mut app = App {
        gl: GlGraphics::new(opengl),
        sim: Bodies {
            x: Simd::from([-1.0, 0.0, 1.0]),
            y: Simd::from([1.0, -2.0, 1.0]),
            vx: Simd::from([-0.1, 0.0, 0.1]),
            vy: Simd::from([1.0, 0.0, -1.0]),
            masses,
            big_g: 1.0,
        },
        radii: cbrt(masses),
        scale: 1.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Event::Input(input, _) = e {
            app.handle(input);
        }
    }
}
