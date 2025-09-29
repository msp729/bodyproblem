use std::{
    array,
    simd::{LaneCount, Simd, StdFloat, SupportedLaneCount, num::SimdFloat},
};

use clap::{ArgAction, Parser};

use crate::runner::run;

use crate::sim::{Bodies, Body};

macro_rules! arms {
    ($ex:expr, $bodies:expr, $g:expr, $d:expr, $($n:literal),+) => {
        match $ex {
            $($n => run(to_sim::<$n>(&$bodies, $g, $d)),)+
            _ => ()
        }
    };
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Parameters describing the bodies' initial configuration.
    /// Given as: position (x,y), velocity (x,y), and mass.
    #[arg(action = ArgAction::Append, allow_hyphen_values=true)]
    body_params: Vec<f64>,
    #[arg(long = "gravity", short = 'G', default_value = "6.6743e-11")]
    gravity: f64,
    #[arg(long = "density", short = 'd', default_value = "1")]
    density: f64,
}

impl Args {
    pub fn simulate(self) {
        let bodies: Vec<Body> = self.body_params.chunks(5).map(Body::fromparams).collect();
        let x = bodies.len();
        arms!(
            x,
            bodies,
            self.gravity,
            self.density,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10
        );
        match x {
            0 => eprintln!("Simulation with no bodies"),
            1 => eprintln!("One-body simulation"),
            _ => (),
        }
    }
}

fn to_sim<const N: usize>(bodies: &[Body], gravity: f64, density: f64) -> (Bodies<N>, Simd<f64, N>)
where
    LaneCount<N>: SupportedLaneCount,
{
    let x = Simd::from(array::from_fn(|i| bodies[i].x));
    let y = Simd::from(array::from_fn(|i| bodies[i].y));
    let vx = Simd::from(array::from_fn(|i| bodies[i].vx));
    let vy = Simd::from(array::from_fn(|i| bodies[i].vy));
    let m = Simd::from(array::from_fn(|i| bodies[i].m));

    let sim = Bodies {
        x,
        y,
        vx,
        vy,
        m,
        big_g: gravity,
    };
    let radii = cbrt(m / Simd::splat(density));
    (sim, radii)
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
