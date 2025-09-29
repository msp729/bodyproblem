#![feature(portable_simd)]
#![warn(clippy::pedantic, clippy::perf)]
#![allow(clippy::cast_precision_loss)]
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use args::Args;
use clap::Parser;

mod args;
mod drawing;
mod rk4;
mod runner;
mod sim;

fn main() {
    let args = Args::parse();

    args.simulate();
}
