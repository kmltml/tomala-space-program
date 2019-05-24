#[macro_use]
extern crate kiss3d;
extern crate nalgebra as na;

mod solver;

use solver::State;

use std::path::Path;

use na::{Vector3, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::resource::{TextureManager};
use kiss3d::conrod;

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

    let mut ids = Ids::new(window.conrod_ui_mut().widget_id_generator());
    ids.mass.resize(3, &mut window.conrod_ui_mut().widget_id_generator());
    ids.velocity.resize(3, &mut window.conrod_ui_mut().widget_id_generator());
    ids.body_panel.resize(3, &mut window.conrod_ui_mut().widget_id_generator());
    window.conrod_ui_mut().theme = theme();

    let mut state = State {
        x: [Vector3::new(0.0, 0.0, 0.0), Vector3::new(8.0, 0.0, 0.0), Vector3::new(8.0, 0.0, 1.0)],
        v: [Vector3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 50.0), Vector3::new(0.0, 10.0, 50.0)],
    };
    let mut masses = [10.0, 1.0, 0.001];

    let mut gui_state = GuiState::new();

    while window.render() {
        sol.set_local_translation(state.x[0].map(|x| x as f32).into());
        earth.set_local_translation(state.x[1].map(|x| x as f32).into());
        luna.set_local_translation(state.x[2].map(|x| x as f32).into());
        window.set_light(Light::Absolute(state.x[0].map(|x| x as f32).into()));
        if !gui_state.paused {
            state.step(0.001, &masses);
        }
        gui(&mut window.conrod_ui_mut().set_widgets(), &ids, &mut masses, &mut gui_state, &mut state);
    }
}

widget_ids! {
    pub struct Ids {
        canvas,
        test,
        general,
        momentum,
        energy,
        pause_play_button,
        body_panel[],
        mass[],
        velocity[]
    }
}

fn theme() -> conrod::Theme {
    conrod::Theme {
        background_color: conrod::color::rgba(0.0, 0.0, 0.0, 0.4),
        label_color: conrod::color::rgba(1.0, 1.0, 1.0, 0.6),
        shape_color: conrod::color::rgba(0.05, 0.33, 0.51, 0.6),
        ..conrod::Theme::default()
    }
}

struct GuiState {
    general_open: bool,
    body_panel_open: [bool; 3],
    paused: bool
}

impl GuiState {
    fn new() -> GuiState {
        GuiState {
            general_open: true,
            paused: false,
            body_panel_open: [false; 3]
        }
    }
}

fn gui(
    ui: &mut conrod::UiCell,
    ids: &Ids,
    masses: &mut [f64; 3],
    state: &mut GuiState,
    body_state: &mut State
) {
    use conrod::{widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};

    const MARGIN: conrod::Scalar = 10.0;
    const WIDTH: conrod::Scalar = 200.0;

    let (gen, genev) = widget::CollapsibleArea::new(state.general_open, "general")
        .top_right()
        .label_font_size(12)
        .w_h(WIDTH, 20.0)
        .set(ids.general, ui);

    for e in genev {
        state.general_open = e.is_open()
    }
    for area in gen {
        let canvas = widget::Canvas::new()
            .h(200.0)
            .pad(MARGIN);

        area.set(canvas, ui);

        let mut momentum = Vector3::zeros();
        for i in 0..3 {
            momentum += body_state.v[i] * masses[i];
        }
        widget::Text::new(&*format!("Total momentum:\n x: {:.2}\n y: {:.2}\n z: {:.2}",
                                    momentum.x, momentum.y, momentum.z))
            .font_size(12)
            .w(WIDTH)
            .parent(area.id)
            .top_left()
            .set(ids.momentum, ui);

        let mut energy = 0.0;
        for i in 0..3 {
            let v = body_state.v[i].norm();
            energy += masses[i] * v * v / 2.0;
            for j in 0..3 {
                if i != j {
                    let r = (body_state.x[i] - body_state.x[j]).norm();
                    energy -= masses[i] * masses[j] / r;
                }
            }
        }

        widget::Text::new(&*format!("Total energy: {:.4}", energy))
            .font_size(12)
            .w(WIDTH)
            .parent(area.id)
            .set(ids.energy, ui);

        if widget::Button::new()
            .parent(area.id)
            .h(30.0)
            .w(area.width / 3.0)
            .label(if state.paused { "Play" } else { "Pause" })
            .set(ids.pause_play_button, ui)
            .was_clicked()
        {
            state.paused = !state.paused;
        }
    }

    let prev = match gen {
        Some(area) => area.id,
        None => ids.general
    };
    let prev = body_panel(0, "Sol", &mut masses[0], body_state, state, prev, ui, ids);
    let prev = body_panel(1, "Earth", &mut masses[1], body_state, state, prev, ui, ids);
    let prev = body_panel(2, "Luna", &mut masses[2], body_state, state, prev, ui, ids);
}

fn body_panel(
    i: usize,
    title: &'static str,
    mass: &mut f64,
    body_state: &mut State,
    state: &mut GuiState,
    previous: conrod::widget::Id,
    ui: &mut conrod::UiCell,
    ids: &Ids
) -> conrod::widget::Id {
    use conrod::{widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};
    const WIDTH: conrod::Scalar = 200.0;
    let (a, e) = widget::CollapsibleArea::new(state.body_panel_open[i], title)
        .w_h(WIDTH, 20.0)
        .down_from(previous, 0.0)
        .set(ids.body_panel[i], ui);
    for e in e {
        state.body_panel_open[i] = e.is_open();
    }
    for area in a {
        let canvas = widget::Canvas::new()
            .pad(10.0)
            .h(60.0);
        area.set(canvas, ui);
        for m in widget::NumberDialer::new(*mass, 0.0, 9999.0, 1)
            .parent(area.id)
            .label("mass")
            .border(0.0)
            .align_left()
            .align_top()
            .h(30.0)
            .label_font_size(12)
            .set(ids.mass[i], ui)
        {
            *mass = m;
        }
        let v = body_state.v[i].norm();
        for nv in widget::NumberDialer::new(v, 0.0, 9999.0, 1)
            .parent(area.id)
            .label("velocity")
            .border(0.0)
            .align_left()
            .down(0.0)
            .h(30.0)
            .label_font_size(12)
            .set(ids.velocity[i], ui)
        {
            body_state.v[i] *= nv / v;
        }
    }
    match a {
        Some(area) => area.id,
        None => ids.body_panel[i]
    }
}
