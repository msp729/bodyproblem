use std::ops::{Add, Div, Mul, Neg};

#[inline]
pub fn rk4<State, Time, Derivative, Delta>(s: State, dv: &mut Derivative, dt: Time) -> State
where
    Delta: Copy + Neg<Output = Delta> + Mul<u8, Output = Delta> + Add<Delta, Output = Delta>,
    State: Copy + Add<Delta, Output = State>,
    Time: Copy + Mul<Delta, Output = Delta> + Div<i8, Output = Time>,
    Derivative: FnMut(State) -> Delta,
{
    let k0 = dv(s);
    let d0 = (dt / 3) * k0;
    let k1 = dv(s + d0);
    let d1 = dt * k1;
    let k2 = dv(s + d1 + -d0);
    let k3 = dv(s + dt * k2 + -d1 + d0 * 3);
    let k = k0 + k1 * 3 + k2 * 3 + k3;
    let d = dt / 8 * k;
    s + d
}

#[inline]
pub fn rk4ntimes<State, Time, Derivative, Delta>(
    mut s: State,
    dv: &mut Derivative,
    dt: Time,
    n: usize,
) -> State
where
    Delta: Copy + Neg<Output = Delta> + Mul<u8, Output = Delta> + Add<Delta, Output = Delta>,
    State: Copy + Add<Delta, Output = State>,
    Time: Copy + Mul<Delta, Output = Delta> + Div<i8, Output = Time>,
    Derivative: FnMut(State) -> Delta,
{
    for _ in 0..n {
        s = rk4(s, dv, dt);
    }
    s
}
