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
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, generate_floor)
        .add_systems(Startup, setup_particles)
        .add_systems(Update, update_particles)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}