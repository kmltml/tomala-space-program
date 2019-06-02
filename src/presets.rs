use na::{Vector3, Point3};
use crate::solver::State;

pub struct Preset {
    pub name: &'static str,
    pub bodies: [BodyData; 3]
}

pub struct BodyData {
    pub name: &'static str,
    pub texture: &'static str,
    pub color: [f32; 3],
    pub trail_color: Point3<f32>,
    pub radius: f32,
    pub mass: f64,
    pub x: Vector3<f64>,
    pub v: Vector3<f64>
}

impl Preset {

    pub fn default_presets() -> Vec<Preset> {
        vec!(
            sun_earth_moon(),
            three_stars(),
            figure_eight(),
            lagrange_1(),
            lagrange_4()
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

fn sun_earth_moon() -> Preset {
    Preset {
        name: "Sun-Earth-Moon",
        bodies: [
            BodyData {
                name: "Sol",
                texture: "sun",
                color: [5.0, 5.0, 5.0],
                trail_color: Point3::new(0.92, 0.80, 0.49),
                radius: 2.0,
                mass: 1000.0,
                x: Vector3::new(0.0, 0.0, 0.0),
                v: Vector3::new(0.0, 0.0, 0.0)
            },
            BodyData {
                name: "Earth",
                texture: "earth",
                color: [1.0, 1.0, 1.0],
                trail_color: Point3::new(0.49, 0.72, 0.92),
                radius: 0.5,
                mass: 16.0,
                x: Vector3::new(20.0, 0.0, 0.0),
                v: Vector3::new(0.0, 0.0, 7.07)
            },
            BodyData {
                name: "Luna",
                texture: "moon",
                color: [1.0, 1.0, 1.0],
                trail_color: Point3::new(0.94, 0.94, 0.94),
                radius: 0.25,
                mass: 0.1,
                x: Vector3::new(20.0, 0.0, 1.0),
                v: Vector3::new(0.0, 4.0, 7.07)
            }
        ]
    }
}

fn three_stars() -> Preset {
    let r = 5.0;
    let m = 1000.0;
    let v = (m / (3.0f64.sqrt() * r)).sqrt();
    Preset {
        name: "Three Stars",
        bodies: [
            BodyData {
                name: "Alpha",
                texture: "sun",
                color: [5.0, 5.0, 5.0],
                trail_color: Point3::new(0.92, 0.80, 0.49),
                radius: 2.0,
                mass: m,
                x: Vector3::new(0.0, 0.0, r),
                v: Vector3::new(-v, 0.0, 0.0)
            },
            BodyData {
                name: "Beta",
                texture: "bluestar",
                color: [3.0, 3.0, 3.0],
                trail_color: Point3::new(0.55, 0.83, 1.00),
                radius: 2.0,
                mass: m,
                x: Vector3::new(- r * 3.0f64.sqrt() / 2.0, 0.0, -r / 2.0),
                v: Vector3::new(v / 2.0, 0.0, - v * 3.0f64.sqrt() / 2.0)
            },
            BodyData {
                name: "Gamma",
                texture: "yellowstar",
                color: [3.0, 3.0, 3.0],
                trail_color: Point3::new(0.99, 1.00, 0.55),
                radius: 2.0,
                mass: m,
                x: Vector3::new(r * 3.0f64.sqrt() / 2.0, 0.0, -r / 2.0),
                v: Vector3::new(v / 2.0, 0.0, v * 3.0f64.sqrt() / 2.0)
            }
        ]
    }
}

fn figure_eight() -> Preset {
    let x1 = Vector3::new(0.97000435669734, 0.0, -0.24308753153583) * 10.0;
    let v3 = Vector3::new(-0.93240737144104, 0.0, -0.86473146092102) * 10.0;
    let m = 1000.0;
    Preset {
        name: "Figure Eight",
        bodies: [
            BodyData {
                name: "Alpha",
                texture: "sun",
                color: [5.0, 5.0, 5.0],
                trail_color: Point3::new(0.92, 0.80, 0.49),
                radius: 2.0,
                mass: m,
                x: x1,
                v: -v3 / 2.0
            },
            BodyData {
                name: "Beta",
                texture: "bluestar",
                color: [5.0, 5.0, 5.0],
                trail_color: Point3::new(0.55, 0.83, 1.00),
                radius: 2.0,
                mass: m,
                x: -x1,
                v: -v3 / 2.0
            },
            BodyData {
                name: "Gamma",
                texture: "yellowstar",
                color: [5.0, 5.0, 5.0],
                trail_color: Point3::new(0.99, 1.00, 0.55),
                radius: 2.0,
                mass: m,
                x: Vector3::zeros(),
                v: v3
            }
        ]
    }
}

fn lagrange_1() -> Preset {
    let R: f64 = 40.0;
    let m2: f64 = 10.0;
    let m1 = 10000.0;
    let r = 2.7076404184011786;
    let v = (m1 / R).sqrt();
    Preset {
        name: "L1",
        bodies: [
            BodyData {
                name: "Sol",
                texture: "sun",
                color: [5.0, 5.0, 5.0],
                trail_color: Point3::new(0.92, 0.80, 0.49),
                radius: 2.0,
                mass: m1,
                x: Vector3::new(0.0, 0.0, 0.0),
                v: Vector3::new(0.0, 0.0, 0.0)
            },
            BodyData {
                name: "Earth",
                texture: "earth",
                color: [1.0, 1.0, 1.0],
                trail_color: Point3::new(0.49, 0.72, 0.92),
                radius: 0.5,
                mass: m2,
                x: Vector3::new(R, 0.0, 0.0),
                v: Vector3::new(0.0, 0.0, v)
            },
            BodyData {
                name: "L1",
                texture: "moon",
                color: [1.0, 1.0, 1.0],
                trail_color: Point3::new(0.94, 0.94, 0.94),
                radius: 0.25,
                mass: 0.1,
                x: Vector3::new(R - r, 0.0, 0.0),
                v: Vector3::new(0.0, 0.0, v * (R - r) / R)
            }
        ]
    }
}

fn lagrange_4() -> Preset {
    let r: f64 = 20.0;
    let m1 = 1000.0;
    let m2 = 10.0;
    let v = (m1 / r).sqrt();
    let x = (m1 - m2) / (m1 + m2) / 2.0;
    let y = (3.0f64).sqrt() / 2.0;
    Preset {
        name: "L4",
        bodies: [
            BodyData {
                name: "Sol",
                texture: "sun",
                color: [5.0, 5.0, 5.0],
                trail_color: Point3::new(0.92, 0.80, 0.49),
                radius: 2.0,
                mass: 1000.0,
                x: Vector3::new(0.0, 0.0, 0.0),
                v: Vector3::new(0.0, 0.0, 0.0)
            },
            BodyData {
                name: "Earth",
                texture: "earth",
                color: [1.0, 1.0, 1.0],
                trail_color: Point3::new(0.49, 0.72, 0.92),
                radius: 0.5,
                mass: m2,
                x: Vector3::new(r, 0.0, 0.0),
                v: Vector3::new(0.0, 0.0, v)
            },
            BodyData {
                name: "Trojan",
                texture: "moon",
                color: [1.0, 1.0, 1.0],
                trail_color: Point3::new(0.94, 0.94, 0.94),
                radius: 0.25,
                mass: 0.1,
                x: Vector3::new(r * x, 0.0, r * y),
                v: Vector3::new(-v * y, 0.0, v * x)
            }
        ]
    }
}
