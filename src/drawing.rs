use std::simd::{LaneCount, Simd, StdFloat, SupportedLaneCount, num::SimdFloat};

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
    pub(crate) colors: [[f32; 4]; N],
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

            let r = (self.sim.x * self.sim.x + self.sim.y * self.sim.y)
                .sqrt()
                .reduce_max();
            let maxdim = r + self.radii.reduce_max();
            let s = s / maxdim;

            let tr = c
                .transform
                .scale(s, -s)
                .trans(x / s, -y / s)
                .scale(self.scale, self.scale);

            for i in 0..N {
                let Body { x, y, .. } = self.sim.body(i);

                let r = self.radii[i];
                ellipse(self.colors[i], [x - r, y - r, 2.0 * r, 2.0 * r], tr, gl);
            }
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        self.sim = superstep(self.sim, DT(self.speed * args.dt), 10);
        for i in 0..N {
            self.colors[i] = hueshift(self.colors[i], ((i + 1) as f32).sqrt() / 10000.0);
        }
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
            (Keyboard(Key::Plus | Key::Equals), piston::ButtonState::Press) => self.scale *= 2.0,
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

type M3 = [[f32; 3]; 3];

fn rot(theta: f32) -> M3 {
    let c = theta.cos();
    let mc = 1. - c;
    let s = theta.sin();
    [
        [
            c + mc / 3.0,
            mc / 3. + s / 3f32.sqrt(),
            mc / 3. - s / 3f32.sqrt(),
        ],
        [
            mc / 3. - s / 3f32.sqrt(),
            c + mc / 3.,
            mc / 3. + s / 3f32.sqrt(),
        ],
        [
            mc / 3. + s / 3f32.sqrt(),
            mc / 3. - s / 3f32.sqrt(),
            c + mc / 3.,
        ],
    ]
}

fn add(x: [f32; 3], y: [f32; 3]) -> [f32; 3] {
    [x[0] + y[0], x[1] + y[1], x[2] + y[2]]
}

fn scale(x: f32, y: [f32; 3]) -> [f32; 3] {
    y.map(|z| x * z)
}

fn app(m: M3, v: [f32; 3]) -> [f32; 3] {
    add(scale(v[0], m[0]), add(scale(v[1], m[1]), scale(v[2], m[2])))
}

fn hueshift(c: [f32; 4], theta: f32) -> [f32; 4] {
    let rgb = [c[0], c[1], c[2]];
    let rgb = app(rot(theta), rgb);
    [
        rgb[0].clamp(0.0, 1.0),
        rgb[1].clamp(0.0, 1.0),
        rgb[2].clamp(0.0, 1.0),
        c[3],
    ]
}
