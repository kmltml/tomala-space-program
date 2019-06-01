use na::Vector3;
use crate::solver::State;

pub struct Preset {
    pub name: &'static str,
    pub bodies: [BodyData; 3]
}

pub struct BodyData {
    name: &'static str,
    radius: f64,
    mass: f64,
    x: Vector3<f64>,
    v: Vector3<f64>
}

impl Preset {

    pub fn default_presets() -> Vec<Preset> {
        vec!(
            Preset {
                name: "Sun-Earth-Moon",
                bodies: [
                    BodyData {
                        name: "sol",
                        radius: 2.0,
                        mass: 1000.0,
                        x: Vector3::new(0.0, 0.0, 0.0),
                        v: Vector3::new(0.0, 0.0, 0.0)
                    },
                    BodyData {
                        name: "earth",
                        radius: 0.5,
                        mass: 16.0,
                        x: Vector3::new(20.0, 0.0, 0.0),
                        v: Vector3::new(0.0, 0.0, 7.07)
                    },
                    BodyData {
                        name: "luna",
                        radius: 0.25,
                        mass: 0.1,
                        x: Vector3::new(20.0, 0.0, 1.0),
                        v: Vector3::new(0.0, 4.0, 7.07)
                    }
                ]
            },
            Preset {
                name: "Equilateral stars",
                bodies: [
                    BodyData {
                        name: "sol",
                        radius: 1.0,
                        mass: 1000.0,
                        x: Vector3::new(0.0, 0.0, 0.0),
                        v: Vector3::new(0.0, 0.0, 0.0)
                    },
                    BodyData {
                        name: "earth",
                        radius: 1.0,
                        mass: 1000.0,
                        x: Vector3::new(20.0, 0.0, 0.0),
                        v: Vector3::new(0.0, 0.0, 7.07)
                    },
                    BodyData {
                        name: "luna",
                        radius: 1.0,
                        mass: 1000.0,
                        x: Vector3::new(0.0, 0.0, 20.0),
                        v: Vector3::new(0.0, 4.0, 7.07)
                    }
                ]
            }
        )
    }

    pub fn masses(&self) -> [f64; 3] {
        let mut ret = [0.0; 3];
        for i in 0..3 {
            ret[i] = self.bodies[i].mass;
        }
        ret
    }

    pub fn x(&self) -> [Vector3<f64>; 3] {
        let mut ret = [Vector3::zeros(); 3];
        for i in 0..3 {
            ret[i] = self.bodies[i].x;
        }
        ret
    }

    pub fn v(&self) -> [Vector3<f64>; 3] {
        let mut ret = [Vector3::zeros(); 3];
        for i in 0..3 {
            ret[i] = self.bodies[i].v;
        }
        ret
    }

    pub fn state(&self) -> State {
        State { x: self.x(), v: self.v() }
    }

}
