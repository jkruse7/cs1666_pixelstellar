use bevy::{prelude::*, window::PresentMode};

mod core;

const TITLE: &str = "Pixelstellar";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;


const LEVEL_W: f32 = 6300.;
const LEVEL_H: f32 = 3600.;



fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.75))))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, core::ui::camera::initialize)
        .add_systems(Startup, core::ui::background::initialize)
        .add_systems(Startup, core::gameplay::player::initialize)

        .add_systems(Update, core::gameplay::player::move_player)
        .add_systems(Update, core::ui::camera::move_camera.after(core::gameplay::player::move_player))
        .run();
}




