#[macro_use]
extern crate kiss3d;
extern crate nalgebra as na;

mod solver;

use solver::State;

use std::path::Path;
use std::collections::vec_deque::VecDeque;

use na::{Vector3, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::resource::{TextureManager};
use kiss3d::camera::ArcBall;
use kiss3d::conrod;

fn main() {
    let mut window = Window::new("Tomala Space Program");
    let mut earth = window.add_sphere(0.5);
    let mut sol = window.add_sphere(2.0);
    let mut luna = window.add_sphere(0.25);
    let mut sky = window.add_sphere(200.0);

    let mut textures = TextureManager::new();

    let mut camera = ArcBall::new(Point3::new(4.0, 4.0, 0.0), Point3::new(0.0, 0.0, 0.0));

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
    ids.follow.resize(3, &mut window.conrod_ui_mut().widget_id_generator());
    window.conrod_ui_mut().theme = theme();

    let mut state = init_state();
    let mut masses = init_masses();

    let mut gui_state = GuiState::new();

    let trail_colors: [Point3<f32>; 3] = [
        Point3::new(0.92, 0.80, 0.49),
        Point3::new(0.49, 0.72, 0.92),
        Point3::new(0.94, 0.94, 0.94)
    ];
    let mut trails: [VecDeque<Point3<f32>>; 3] = [VecDeque::new(), VecDeque::new(), VecDeque::new()];

    while window.render_with_camera(&mut camera) {
        sol.set_local_translation(state.x[0].map(|x| x as f32).into());
        earth.set_local_translation(state.x[1].map(|x| x as f32).into());
        luna.set_local_translation(state.x[2].map(|x| x as f32).into());
        window.set_light(Light::Absolute(state.x[0].map(|x| x as f32).into()));
        for i in 0..3 {
            trails[i].push_front(state.x[i].map(|x| x as f32).into());
            if(trails[i].len() > gui_state.trail_length) {
                trails[i].pop_back();
            }
            let color = trail_colors[i];
            for (i, (a, b)) in trails[i].iter().zip(trails[i].iter().skip(1)).enumerate().rev() {
                let l = 1.0 - (i as f32) / (gui_state.trail_length as f32);
                window.draw_line(a, b, &(color * l));
            }
        }
        if !gui_state.paused {
            state.step(0.01, &masses);
        }
        gui(&mut window.conrod_ui_mut().set_widgets(), &ids, &mut masses, &mut gui_state, &mut state);
        if let Some(i) = gui_state.follow {
            camera.set_at(state.x[i].map(|x| x as f32).into());
        }
        if gui_state.reset {
            state = init_state();
            masses = init_masses();
            for i in 0..3 {
                trails[i].clear();
            }
        }
    }
}

fn init_state() -> State {
    State {
        x: [Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(20.0, 0.0, 0.0),
            Vector3::new(20.0, 0.0, 1.0)],
        v: [Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 7.07),
            Vector3::new(0.0, 4.0, 7.07)],
    }
}

fn init_masses() -> [f64; 3] {
    [1000.0, 16.0, 0.001]
}

widget_ids! {
    pub struct Ids {
        general,
        momentum,
        energy,
        trail_length,
        pause_play_button,
        momentum_zero,
        reset,
        body_panel[],
        mass[],
        velocity[],
        follow[]
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
    paused: bool,
    reset: bool,
    trail_length: usize,
    follow: Option<usize>
}

impl GuiState {
    fn new() -> GuiState {
        GuiState {
            general_open: true,
            body_panel_open: [false; 3],
            paused: false,
            reset: false,
            trail_length: 500,
            follow: None
        }
    }
}

const MARGIN: conrod::Scalar = 10.0;

fn gui(
    ui: &mut conrod::UiCell,
    ids: &Ids,
    masses: &mut [f64; 3],
    state: &mut GuiState,
    body_state: &mut State
) {
    use conrod::{widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};

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
            for j in (i + 1)..3 {
                let r = (body_state.x[i] - body_state.x[j]).norm();
                energy -= masses[i] * masses[j] / r;
            }
        }

        widget::Text::new(&*format!("Total energy: {:.4}", energy))
            .font_size(12)
            .w(WIDTH)
            .parent(area.id)
            .set(ids.energy, ui);

        for len in widget::NumberDialer::new(state.trail_length as f64, 0.0, 9999.0, 0)
            .parent(area.id)
            .label("trail length")
            .border(0.0)
            .align_left()
            .w(area.width - 2.0 * MARGIN)
            .h(30.0)
            .label_font_size(12)
            .set(ids.trail_length, ui)
        {
            state.trail_length = len as usize;
        }

        if widget::Button::new()
            .parent(area.id)
            .h(30.0)
            .w((area.width - 2.0 * MARGIN) / 3.0)
            .label(if state.paused { "Play" } else { "Pause" })
            .label_font_size(12)
            .set(ids.pause_play_button, ui)
            .was_clicked()
        {
            state.paused = !state.paused;
        }

        if widget::Button::new()
            .parent(area.id)
            .h(30.0)
            .w((area.width - 2.0 * MARGIN) / 3.0)
            .right(0.0)
            .y_relative(0.0)
            .label("p 0")
            .label_font_size(12)
            .set(ids.momentum_zero, ui)
            .was_clicked()
        {
            let m: f64 = masses.iter().sum();
            let dv = momentum / m;
            for i in 0..3 {
                body_state.v[i] -= dv;
            }
        }

        state.reset = widget::Button::new()
            .parent(area.id)
            .h(30.0)
            .w((area.width - 2.0 * MARGIN) / 3.0)
            .right(0.0)
            .y_relative(0.0)
            .label("Reset")
            .label_font_size(12)
            .set(ids.reset, ui)
            .was_clicked();
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
            .h(90.0)
            .pad(MARGIN);
        area.set(canvas, ui);
        for m in widget::NumberDialer::new(*mass, 0.0, 9999.0, 1)
            .parent(area.id)
            .label("mass")
            .border(0.0)
            .align_top()
            .align_middle_x()
            .w(area.width - 2.0 * MARGIN)
            .h(30.0)
            .label_font_size(12)
            .set(ids.mass[i], ui)
        {
            *mass = m;
        }
        let v = body_state.v[i].norm();
        for nv in widget::NumberDialer::new(v, 1.0, 9999.0, 1)
            .parent(area.id)
            .label("velocity")
            .border(0.0)
            .align_left()
            .down(0.0)
            .h(30.0)
            .w(area.width - 2.0 * MARGIN)
            .label_font_size(12)
            .set(ids.velocity[i], ui)
        {
            body_state.v[i] = body_state.v[i].normalize() * nv;
        }
        for s in widget::Toggle::new(state.follow == Some(i))
            .parent(area.id)
            .label("follow")
            .align_left()
            .down(0.0)
            .h(30.0)
            .w(area.width - 2.0 * MARGIN)
            .label_font_size(12)
            .set(ids.follow[i], ui)
        {
            state.follow = if s { Some(i) } else { None };
        }
    }
    match a {
        Some(area) => area.id,
        None => ids.body_panel[i]
    }
}
