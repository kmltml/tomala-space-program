#[macro_use]
extern crate kiss3d;
extern crate nalgebra as na;

mod solver;
mod presets;

use solver::State;
use presets::Preset;

use std::path::Path;
use std::collections::vec_deque::VecDeque;

use na::{Vector3, Point3, Rotation3};
use kiss3d::window::Window;
use kiss3d::event::{Key, WindowEvent, Action};
use kiss3d::light::Light;
use kiss3d::resource::{TextureManager};
use kiss3d::camera::{ArcBall, Camera};
use kiss3d::conrod;

fn main() {
    let mut window = Window::new("Tomala Space Program");
    let mut body_spheres = [window.add_sphere(1.0), window.add_sphere(1.0), window.add_sphere(1.0)];
    let mut sky = window.add_sphere(200.0);

    let mut textures = TextureManager::new();

    let mut camera = ArcBall::new(Point3::new(4.0, 4.0, 0.0), Point3::new(0.0, 0.0, 0.0));

    let presets = Preset::default_presets();

    textures.add(Path::new("tex/earth.jpg"), "earth");
    textures.add(Path::new("tex/sun.jpg"), "sun");
    textures.add(Path::new("tex/moon.jpg"), "moon");
    textures.add(Path::new("tex/sky.jpg"), "sky");
    textures.add(Path::new("tex/yellowstar.jpg"), "yellowstar");
    textures.add(Path::new("tex/bluestar.jpg"), "bluestar");

    sky.set_texture(textures.get("sky").unwrap());
    sky.enable_backface_culling(false);
    sky.set_color(5.0, 5.0, 5.0);

    let mut ids = Ids::new(window.conrod_ui_mut().widget_id_generator());
    ids.mass.resize(3, &mut window.conrod_ui_mut().widget_id_generator());
    ids.velocity.resize(3, &mut window.conrod_ui_mut().widget_id_generator());
    ids.body_panel.resize(3, &mut window.conrod_ui_mut().widget_id_generator());
    ids.follow.resize(3, &mut window.conrod_ui_mut().widget_id_generator());
    ids.fix.resize(3, &mut window.conrod_ui_mut().widget_id_generator());
    ids.fix_rot.resize(3, &mut window.conrod_ui_mut().widget_id_generator());
    window.conrod_ui_mut().theme = theme();

    let mut state = presets[0].state();
    let mut masses = presets[0].masses();
    for i in 0..3 {
        let body_data = &presets[0].bodies[i];
        body_spheres[i].set_texture(textures.get(body_data.texture).unwrap());
        body_spheres[i].set_color(body_data.color[0], body_data.color[1], body_data.color[2]);
        body_spheres[i].set_local_scale(body_data.radius, body_data.radius, body_data.radius);
    }

    let mut gui_state = GuiState::new();

    let mut trails: [VecDeque<Point3<f32>>; 3] = [VecDeque::new(), VecDeque::new(), VecDeque::new()];

    while window.render_with_camera(&mut camera) {
        for i in 0..3 {
            body_spheres[i].set_local_translation(state.x[i].map(|x| x as f32).into());
        }
        window.set_light(Light::Absolute(state.x[0].map(|x| x as f32).into()));
        for i in 0..3 {
            trails[i].push_front(state.x[i].map(|x| x as f32).into());
            if trails[i].len() > gui_state.trail_length {
                trails[i].pop_back();
            }
            let color = presets[gui_state.selected_preset].bodies[i].trail_color;
            for (i, (a, b)) in trails[i].iter().zip(trails[i].iter().skip(1)).enumerate().rev() {
                let l = 1.0 - (i as f32) / (gui_state.trail_length as f32);
                window.draw_line(a, b, &(color * l));
            }
        }
        if !gui_state.paused {
            for _ in 0..gui_state.simulation_speed * gui_state.substeps {
                state.step(0.001 / gui_state.substeps as f64, &masses);
            }
        }
        gui(&mut window.conrod_ui_mut().set_widgets(), &ids, &mut masses, &mut gui_state, &mut state, &presets);

        for e in window.events().iter() {
            match e.value {
                WindowEvent::Key(Key::Space, Action::Press, _) =>
                    gui_state.paused = !gui_state.paused,
                _ => ()
            }
        }

        if let Some(f) = gui_state.follow {
            camera.set_at(state.x[f].map(|x| x as f32).into());
        }

        if let FixState::Fix(f, rot) = gui_state.fix {
            let pos = state.x[f];
            for i in 0..3 {
                state.x[i] -= pos;
            }
            for r in rot {
                if let Some(trans) = Rotation3::rotation_between(&state.x[r], &Vector3::new(1.0, 0.0, 0.0)) {
                    for i in 0..3 {
                        state.x[i] = trans * state.x[i];
                        state.v[i] = trans * state.v[i];
                    }
                }
            }
        }

        sky.set_local_translation(camera.eye().coords.into());
        if gui_state.reset || gui_state.preset_changed {
            let preset = &presets[gui_state.selected_preset];
            state = preset.state();
            masses = preset.masses();
            for i in 0..3 {
                let body_data = &preset.bodies[i];
                body_spheres[i].set_texture(textures.get(body_data.texture).unwrap());
                body_spheres[i].set_color(body_data.color[0], body_data.color[1], body_data.color[2]);
                body_spheres[i].set_local_scale(body_data.radius, body_data.radius, body_data.radius);
            }
            gui_state.follow = None;
            gui_state.fix = FixState::None;
        }
        if gui_state.reset || gui_state.clear_trails || gui_state.preset_changed {
            for i in 0..3 {
                trails[i].clear();
            }
        }
    }
}

widget_ids! {
    pub struct Ids {
        general,
        momentum,
        energy,
        preset,
        speed,
        substeps,
        trail_length,
        pause_play_button,
        momentum_zero,
        reset,
        clear_trails,
        body_panel[],
        mass[],
        velocity[],
        follow[],
        fix[],
        fix_rot[]
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
    selected_preset: usize,
    preset_changed: bool,
    paused: bool,
    reset: bool,
    clear_trails: bool,
    trail_length: usize,
    simulation_speed: usize,
    substeps: usize,
    follow: Option<usize>,
    fix: FixState
}

impl GuiState {
    fn new() -> GuiState {
        GuiState {
            general_open: true,
            body_panel_open: [false; 3],
            selected_preset: 0,
            preset_changed: true,
            paused: false,
            reset: false,
            clear_trails: false,
            trail_length: 500,
            simulation_speed: 10,
            substeps: 10,
            follow: None,
            fix: FixState::None
        }
    }
}

#[derive(PartialEq, Debug)]
enum FixState {
    Fix(usize, Option<usize>),
    None
}

impl FixState {

    fn fix_center(&self) -> Option<usize> {
        match *self {
            FixState::Fix(f, _) => Some(f),
            _ => None
        }
    }

    fn fix_rot(&self) -> Option<usize> {
        match *self {
            FixState::Fix(_, f) => f,
            _ => None
        }
    }

}

const MARGIN: conrod::Scalar = 10.0;

fn gui(
    ui: &mut conrod::UiCell,
    ids: &Ids,
    masses: &mut [f64; 3],
    state: &mut GuiState,
    body_state: &mut State,
    presets: &Vec<Preset>
) {
    use conrod::{widget, Borderable, Labelable, Positionable, Sizeable, Widget};

    const WIDTH: conrod::Scalar = 200.0;

    let preset = &presets[state.selected_preset];

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
            .h(370.0)
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

        state.preset_changed = false;

        let preset_names: Vec<&str> = presets.iter().map(|p| p.name).collect();
        for i in widget::DropDownList::new(&preset_names, Some(state.selected_preset))
            .parent(area.id)
            .align_left()
            .w(area.width - 2.0 * MARGIN)
            .h(30.0)
            .label_font_size(12)
            .set(ids.preset, ui)
        {
            state.selected_preset = i;
            state.preset_changed = true;
        }

        for s in widget::Slider::new(state.simulation_speed as f64, 1.0, 100.0)
            .skew(2.0)
            .parent(area.id)
            .align_left()
            .w(area.width - 2.0 * MARGIN)
            .h(30.0)
            .label(&format!("speed: {}", state.simulation_speed))
            .label_font_size(12)
            .set(ids.speed, ui)
        {
            state.simulation_speed = s as usize;
        }

        for s in widget::Slider::new(state.substeps as f64, 1.0, 1000.0)
            .skew(2.0)
            .parent(area.id)
            .align_left()
            .down(0.0)
            .w(area.width - 2.0 * MARGIN)
            .h(30.0)
            .label(&format!("substeps: {}", state.substeps))
            .label_font_size(12)
            .set(ids.substeps, ui)
        {
            state.substeps = s as usize;
        }



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

        state.clear_trails = widget::Button::new()
            .parent(area.id)
            .h(30.0)
            .w((area.width - 2.0 * MARGIN) / 3.0)
            .down(0.0)
            .align_left_of(ids.pause_play_button)
            .label("Clear\ntrails")
            .label_font_size(12)
            .set(ids.clear_trails, ui)
            .was_clicked()
    }

    let mut prev = match gen {
        Some(area) => area.id,
        None => ids.general
    };
    for i in 0..3 {
        prev = body_panel(i, preset.bodies[i].name, &mut masses[i], body_state, state, prev, ui, ids);
    }
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
    use conrod::{widget, Borderable, Labelable, Positionable, Sizeable, Widget};
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
            .w((area.width - 2.0 * MARGIN) / 3.0)
            .label_font_size(12)
            .set(ids.follow[i], ui)
        {
            state.follow = if s { Some(i) } else { None };
        }
        for s in widget::Toggle::new(state.fix.fix_center() == Some(i))
            .parent(area.id)
            .label("fix")
            .right(0.0)
            .y_relative(0.0)
            .h(30.0)
            .w((area.width - 2.0 * MARGIN) / 3.0)
            .label_font_size(12)
            .set(ids.fix[i], ui)
        {
            state.fix = if s { FixState::Fix(i, None) } else { FixState::None };
        }
        for s in widget::Toggle::new(state.fix.fix_rot() == Some(i))
            .parent(area.id)
            .label("fix rot")
            .enabled(state.fix.fix_center() != None && state.fix.fix_center() != Some(i))
            .right(0.0)
            .y_relative(0.0)
            .h(30.0)
            .w((area.width - 2.0 * MARGIN) / 3.0)
            .label_font_size(12)
            .set(ids.fix_rot[i], ui)
        {
            state.fix = if s {
                FixState::Fix(state.fix.fix_center().unwrap(), Some(i))
            } else {
                FixState::Fix(state.fix.fix_center().unwrap(), None)
            };
        }
    }
    match a {
        Some(area) => area.id,
        None => ids.body_panel[i]
    }
}
