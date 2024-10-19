// rust and bevy imports
use bevy::{prelude::*, window::PresentMode};

// module declarations and imports
mod engine;
mod gameplay;
mod ui;
mod world;
mod particle;

use crate::particle::resources::*;
use crate::particle::systems::*;

// constants
const TITLE: &str = "Pixelstellar";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const LEVEL_W: f32 = WIN_W;
const LEVEL_H: f32 = WIN_H;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(0, 0, 0)))
        .add_systems(Startup, setup_camera)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SpawnParticles)
        .add_plugins(UpdateParticles)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}