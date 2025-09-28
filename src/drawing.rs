use std::simd::{LaneCount, Simd, SupportedLaneCount, num::SimdFloat};

use opengl_graphics::GlGraphics;
use piston::{
    ButtonArgs, Input, Key,
    input::{RenderArgs, UpdateArgs},
};

use crate::{
    rk4::{rk4, rk4ntimes},
    sim::{Bodies, Body, DT, derivative, energy, superstep},
};

pub struct App<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    pub(crate) gl: GlGraphics,
    pub(crate) sim: Bodies<N>,
    pub(crate) radii: Simd<f64, N>,
    pub(crate) scale: f64,
}

const BG: [f32; 4] = [0.1, 0.1, 0.1, 0.0];

impl<const N: usize> App<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let s = x.min(y);
        let square = rectangle::square(-0.5, -0.5, 0.5);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BG, gl);

            let xmax = self.sim.x.abs().reduce_max();
            let ymax = self.sim.y.abs().reduce_max();
            let maxdim = xmax.max(ymax) + self.radii.reduce_max();
            let s = s / maxdim;

            let tr = c.transform.scale(s, -s).trans(x / s, -y / s);

            for i in 0..N {
                let Body { x, y, vx, vy, m } = self.sim.body(i);

                let r = self.radii[i];
                ellipse(
                    [0.8, 0.6, 0.3, 0.8],
                    [x - r, y - r, 2.0 * r, 2.0 * r],
                    tr,
                    gl,
                )
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.sim = superstep(self.sim, DT(args.dt), 10);
        let (x, y) = self.sim.com();
    }

    pub fn handle(&mut self, inp: Input) {
        match inp {
            Input::Button(ba) => self.handle_button(ba),
            _ => (),
        }
    }

    fn handle_button(&mut self, ba: ButtonArgs) {
        let ButtonArgs {
            state,
            button,
            scancode,
        } = ba;
        match (button, state) {
            (piston::Button::Keyboard(Key::Space), piston::ButtonState::Press) => self.show(),
            // angular momentum L = ωmr² = vmr = m(v cross r)
            _ => (),
        }
    }

    fn show(&self) {
        let (x, y) = self.sim.com();
        let e = energy(self.sim);
        let am0 = self.sim.tam(0.0, 0.0);
        let aml = self.sim.tam(-1.0, 0.0);
        let amr = self.sim.tam(1.0, 0.0);
        let amu = self.sim.tam(0.0, 1.0);
        let amd = self.sim.tam(0.0, -1.0);
        println!(
            "E={:+.8}\tx=({:+.4},{:+.4})\tL:0={:+.4}, l={:+.4}, r={:+.4}, u={:+.4}, d={:+.4}",
            e, x, y, am0, aml, amr, amu, amd
        );
    }
}
