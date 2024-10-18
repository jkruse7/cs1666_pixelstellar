// rust and bevy imports
use bevy::{prelude::*, window::PresentMode};

// module declarations and imports
mod engine;
mod gameplay;
mod ui;
mod world;

use crate::{
    engine::test_particle::*,
    world::grid::*,
    world::floor::*,
    gameplay::player::*,
    ui::background::*,
    ui::camera::*,
};

// constants
const TITLE: &str = "Pixelstellar";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const LEVEL_W: f32 = WIN_W;
const LEVEL_H: f32 = WIN_H;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(0, 0, 0)))
        .insert_resource(Grid::new())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        //.add_systems(Startup, setup_camera)
        .add_systems(Startup, initialize_camera)
        .add_systems(Startup, initialize_background)
        .add_systems(Startup, initialize_player)
        .add_systems(Startup, setup_particles)
        .add_systems(Startup, generate_floor)

        // Updates
        .add_systems(Update, update_particles)
        .add_systems(Update, move_player)
        .add_systems(Update, flight.after(move_player))
        .add_systems(Update, animate_player.after(move_player))
        .add_systems(Update, move_camera.after(move_player))
        .run();
}