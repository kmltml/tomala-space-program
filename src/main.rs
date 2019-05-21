extern crate kiss3d;
extern crate nalgebra as na;

use std::{rc, cell};
use std::path::Path;
use na::{Vector3, UnitQuaternion, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::resource::{TextureManager};

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut earth = window.add_sphere(1.0);

    let mut textures = TextureManager::new();

    textures.add(Path::new("tex/earth.jpg"), "earth");

    earth.set_texture(textures.get("earth").unwrap());

    window.set_light(Light::Absolute(Point3::new(2.0, 2.0, 2.0)));

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        earth.append_to_local_rotation(&rot);
    }
}
