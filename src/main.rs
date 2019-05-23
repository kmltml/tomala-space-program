extern crate kiss3d;
extern crate nalgebra as na;

mod solver;

use solver::State;

use std::path::Path;
use na::{Vector3, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::resource::{TextureManager};

fn main() {
    let mut window = Window::new("Tomala Space Program");
    let mut earth = window.add_sphere(0.5);
    let mut sol = window.add_sphere(2.0);
    let mut luna = window.add_sphere(0.25);
    let mut sky = window.add_sphere(200.0);

    let mut textures = TextureManager::new();

    textures.add(Path::new("tex/earth.jpg"), "earth");
    textures.add(Path::new("tex/sun.jpg"), "sun");
    textures.add(Path::new("tex/moon.jpg"), "moon");
    textures.add(Path::new("tex/sky.jpg"), "sky");

    earth.set_texture(textures.get("earth").unwrap());
    luna.set_texture(textures.get("moon").unwrap());
    sol.set_texture(textures.get("sun").unwrap());
    sky.set_texture(textures.get("sky").unwrap());
    sky.enable_backface_culling(false);
    sky.set_color(5.0, 5.0, 5.0);
    sol.set_color(6.0, 6.0, 6.0);

    let mut state = State {
        x: [Vector3::new(0.0, 0.0, 0.0), Vector3::new(8.0, 0.0, 0.0), Vector3::new(8.0, 0.0, 1.0)],
        v: [Vector3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 50.0), Vector3::new(10.0, 0.0, 50.0)],
    };
    let masses = [10.0, 1.0, 0.001];

    while window.render() {
        sol.set_local_translation(state.x[0].map(|x| x as f32).into());
        earth.set_local_translation(state.x[1].map(|x| x as f32).into());
        luna.set_local_translation(state.x[2].map(|x| x as f32).into());
        window.set_light(Light::Absolute(state.x[0].map(|x| x as f32).into()));
        state.step(0.001, &masses);
    }
}
