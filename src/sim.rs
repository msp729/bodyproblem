use std::{
    ops::{Add, Div, Mul, Neg},
    simd::{LaneCount, Simd, StdFloat, SupportedLaneCount, num::SimdFloat},
};

use crate::rk4::rk4;

#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub m: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Bodies<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    pub x: Simd<f64, N>,
    pub y: Simd<f64, N>,
    pub vx: Simd<f64, N>,
    pub vy: Simd<f64, N>,
    pub m: Simd<f64, N>,
    /// universal gravitational constant, G.
    /// In the real world, it's about 6.67e-11.
    pub big_g: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct BodiesDelta<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    pub x: Simd<f64, N>,
    pub y: Simd<f64, N>,
    pub vx: Simd<f64, N>,
    pub vy: Simd<f64, N>,
}

impl Body {
    pub fn fromparams(params: &[f64]) -> Self {
        Self {
            x: params[0],
            y: params[1],
            vx: params[2],
            vy: params[3],
            m: params[4],
        }
    }
}

impl<const N: usize> Bodies<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    pub fn body(self, i: usize) -> Body {
        Body {
            x: self.x[i],
            y: self.y[i],
            vx: self.vx[i],
            vy: self.vy[i],
            m: self.m[i],
        }
    }

    pub fn com(self) -> (f64, f64) {
        (
            self.x.reduce_sum() / N as f64,
            self.y.reduce_sum() / N as f64,
        )
    }

    pub fn tam(self, x0: f64, y0: f64) -> f64 {
        let dx = self.x - Simd::splat(x0);
        let dy = self.y - Simd::splat(y0);
        // L = m * (r.y * v.x - r.x * v.y);
        let ang_p = self.m * (dx * self.vy - dy * self.vx);
        ang_p.reduce_sum()
    }
}

impl<const N: usize> Neg for BodiesDelta<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            vx: -self.vx,
            vy: -self.vy,
        }
    }
}

impl<const N: usize> Mul<u8> for BodiesDelta<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x * Simd::splat(f64::from(rhs)),
            y: self.y * Simd::splat(f64::from(rhs)),
            vx: self.vx * Simd::splat(f64::from(rhs)),
            vy: self.vy * Simd::splat(f64::from(rhs)),
        }
    }
}

impl<const N: usize> Add for BodiesDelta<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            vx: self.vx + rhs.vx,
            vy: self.vy + rhs.vy,
        }
    }
}

impl<const N: usize> Add<BodiesDelta<N>> for Bodies<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    fn add(self, rhs: BodiesDelta<N>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            vx: self.vx + rhs.vx,
            vy: self.vy + rhs.vy,
            ..self
        }
    }
}

#[derive(Clone, Copy)]
pub struct DT(pub f64);

impl<const N: usize> Mul<BodiesDelta<N>> for DT
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = BodiesDelta<N>;

    fn mul(self, rhs: BodiesDelta<N>) -> Self::Output {
        let splatted = Simd::splat(self.0);
        BodiesDelta {
            x: rhs.x * splatted,
            y: rhs.y * splatted,
            vx: rhs.vx * splatted,
            vy: rhs.vy * splatted,
        }
    }
}

impl Div<i8> for DT {
    type Output = Self;

    fn div(self, rhs: i8) -> Self::Output {
        Self(self.0 / f64::from(rhs))
    }
}

pub fn gravity<const N: usize>(bodies: Bodies<N>) -> ([f64; N], [f64; N])
where
    LaneCount<N>: SupportedLaneCount,
{
    let mut ret = ([0.0; N], [0.0; N]);
    for i in 0..N {
        let dx = bodies.x - Simd::splat(bodies.x[i]);
        let dy = bodies.y - Simd::splat(bodies.y[i]);
        let massprod = bodies.m * Simd::splat(bodies.big_g * bodies.m[i]);
        let r = (dx * dx + dy * dy).sqrt();
        let coe = massprod / (r * r * r);
        let fx = dx * coe;
        let fy = dy * coe;
        for j in 0..i {
            // F_G = (S-F)GMm/(S-F)^3
            ret.0[i] += fx[j];
            ret.0[j] -= fx[j];
            ret.1[i] += fy[j];
            ret.1[j] -= fy[j];
        }
    }
    ret
}

pub fn derivative<const N: usize>(bodies: Bodies<N>) -> BodiesDelta<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    let (vx, vy) = gravity(bodies);
    BodiesDelta {
        x: bodies.vx,
        y: bodies.vy,
        vx: Simd::from(vx) / bodies.m,
        vy: Simd::from(vy) / bodies.m,
    }
}

pub fn energy<const N: usize>(bodies: Bodies<N>) -> f64
where
    LaneCount<N>: SupportedLaneCount,
{
    // E=K+V
    // K=Σmv²/2
    // V=Σ-GMm/r
    let vsq = bodies.vx * bodies.vx + bodies.vy * bodies.vy;
    let kinetics = bodies.m * vsq / Simd::splat(2.0);
    let mut total = kinetics.reduce_sum();
    for i in 0..N {
        let dx = bodies.x - Simd::splat(bodies.x[i]);
        let dy = bodies.y - Simd::splat(bodies.y[i]);
        let massprod = bodies.m * Simd::splat(bodies.big_g * bodies.m[i]);
        let r = (dx * dx + dy * dy).sqrt();
        let potentials = -massprod / r;
        for j in 0..i {
            total += potentials[j];
        }
    }
    total
}

pub fn energy_gradient<const N: usize>(bodies: Bodies<N>) -> BodiesDelta<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    let (f_x, f_y) = gravity(bodies);
    let p_x = bodies.m * bodies.vx;
    let p_y = bodies.m * bodies.vy;
    // dE/dx = F_x
    BodiesDelta {
        x: f_x.into(),
        y: f_y.into(),
        vx: p_x,
        vy: p_y,
    }
}

// s -> s - (E(s) - E₀) ∇E / (∇E)²
/// returns ∇E / (∇E)²
pub fn correct_energy<const N: usize>(bodies: Bodies<N>) -> BodiesDelta<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    let BodiesDelta { x, y, vx, vy } = energy_gradient(bodies);
    let ssq = (x * x + y * y + vx * vx + vy * vy).reduce_sum();
    let splatted = Simd::splat(ssq);
    let x = x / splatted;
    let y = y / splatted;
    let vx = vx / splatted;
    let vy = vy / splatted;
    BodiesDelta { x, y, vx, vy }
}

pub fn superstep<const N: usize>(mut bodies: Bodies<N>, dt: DT, n: u16) -> Bodies<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    let mut oe = energy(bodies);
    for _ in 0..n {
        bodies = rk4(bodies, &mut derivative, DT(dt.0 / f64::from(n)));
        let newe = energy(bodies);
        bodies = bodies + DT(newe - oe) * correct_energy(bodies);
        oe = newe;
    }
    bodies
}
