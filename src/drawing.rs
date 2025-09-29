use std::simd::{LaneCount, Simd, SupportedLaneCount, num::SimdFloat};

use opengl_graphics::GlGraphics;
use piston::{
    Button, ButtonArgs, Input, Key,
    input::{RenderArgs, UpdateArgs},
};

use crate::sim::{Bodies, Body, DT, energy, superstep};

pub struct App<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    pub(crate) gl: GlGraphics,
    pub(crate) sim: Bodies<N>,
    pub(crate) radii: Simd<f64, N>,
    pub(crate) scale: f64,
    pub(crate) speed: f64,
}

const BG: [f32; 4] = [0.1, 0.1, 0.1, 0.0];

impl<const N: usize> App<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::{Transformed, clear, ellipse};

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let s = x.min(y);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BG, gl);

            let xmax = self.sim.x.abs().reduce_max();
            let ymax = self.sim.y.abs().reduce_max();
            let maxdim = xmax.max(ymax) + self.radii.reduce_max();
            let s = s / maxdim;

            let tr = c
                .transform
                .scale(s, -s)
                .trans(x / s, -y / s)
                .scale(self.scale, self.scale);

            for i in 0..N {
                let Body { x, y, .. } = self.sim.body(i);

                let r = self.radii[i];
                ellipse(
                    [0.8, 0.6, 0.3, 0.8],
                    [x - r, y - r, 2.0 * r, 2.0 * r],
                    tr,
                    gl,
                );
            }
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        self.sim = superstep(self.sim, DT(self.speed * args.dt), 10);
    }

    pub fn handle(&mut self, inp: &Input) {
        if let Input::Button(ba) = inp {
            self.handle_button(ba);
        }
    }

    fn handle_button(&mut self, ba: &ButtonArgs) {
        use Button::Keyboard;

        let ButtonArgs {
            state,
            button,
            scancode: _,
        } = ba;

        match (button, state) {
            (Keyboard(Key::Space), piston::ButtonState::Press) => self.speed = 1.0 - self.speed,
            (Keyboard(Key::Return), piston::ButtonState::Press) => self.show(),
            (Keyboard(Key::Plus), piston::ButtonState::Press) => self.scale *= 2.0,
            (Keyboard(Key::Minus), piston::ButtonState::Press) => self.scale /= 2.0,
            _ => (),
        }
    }

    #[allow(clippy::similar_names)]
    fn show(&self) {
        let (x, y) = self.sim.com();
        let e = energy(self.sim);
        let am0 = self.sim.tam(0.0, 0.0);
        let aml = self.sim.tam(-1.0, 0.0);
        let amr = self.sim.tam(1.0, 0.0);
        let amu = self.sim.tam(0.0, 1.0);
        let amd = self.sim.tam(0.0, -1.0);
        println!(
            "E={e:+.8}\t\
            x=({x:+.4},{y:+.4})\t\
            L:0={am0:+.4},\
            l={aml:+.4},\
            r={amr:+.4},\
            u={amu:+.4},\
            d={amd:+.4}"
        );
    }
}
