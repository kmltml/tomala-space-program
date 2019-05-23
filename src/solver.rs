use na::Vector3;
use std::ops;

#[derive(Copy, Clone)]
pub struct State {
    pub x: [Vector3<f64>; 3],
    pub v: [Vector3<f64>; 3]
}

fn get_acceleration(s: State, m: &[f64; 3]) -> [Vector3<f64>; 3] {
    let mut ret = [Vector3::new(0.0, 0.0, 0.0); 3];
    for i in 0..3 {
        for j in 0..3 {
            if i != j {
                let r = s.x[j] - s.x[i];
                let rl = r.norm();
                ret[i] += r * m[j] / rl * rl * rl;
            }
        }
    }
    ret
}

impl ops::Add<State> for State {
    type Output = State;
    fn add(self, r: State) -> State {
        let mut s = self.clone();
        for i in 0..3 {
            s.x[i] += r.x[i];
            s.v[i] += r.v[i];
        }
        s
    }
}

impl ops::Mul<f64> for State {
    type Output = State;
    fn mul(self, h: f64) -> State {
        let mut s = self.clone();
        for i in 0..3 {
            s.x[i] *= h;
            s.v[i] *= h;
        }
        s
    }
}

impl State {

    pub fn step(&mut self, h: f64, m: &[f64; 3]) {
        fn deriv(s: State, m: &[f64; 3]) -> State {
            let acc = get_acceleration(s, m);
            State { x: s.v, v: acc }
        }
        let k1 = deriv(*self, m) * h;
        let k2 = deriv(*self + k1 * 0.5, m) * h;
        let k3 = deriv(*self + k2 * 0.5, m) * h;
        let k4 = deriv(*self + k3, m) * h;
        *self = *self + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (1.0 / 6.0)
    }

}
