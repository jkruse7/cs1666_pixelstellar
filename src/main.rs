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
const LEVEL_W: f32 = WIN_W * 3.0;
const LEVEL_H: f32 = WIN_H * 2.0;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(0, 0, 0)))
        //.add_systems(Startup, setup_camera)
        .add_systems(Startup, ui::camera::initialize)
        .add_systems(Startup, ui::background::initialize)
        .add_systems(Startup, gameplay::player::initialize)
        .add_systems(Startup, gameplay::blaster::initialize)
        .add_systems(Startup, gameplay::enemy::initialize)
        .add_systems(Startup, ui::health::setup_health_bar)

        .add_systems(Update, gameplay::player::move_player)
        .add_systems(Update, gameplay::player::flight.after(gameplay::player::move_player))
        .add_systems(Update, gameplay::player::animate_player.after(gameplay::player::move_player))
        .add_systems(Update, ui::camera::move_camera.after(gameplay::player::move_player))
        .add_systems(Update, gameplay::blaster::update_blaster_aim)
        .add_systems(Update, gameplay::blaster::shoot_blaster.after(gameplay::blaster::update_blaster_aim))
        .add_systems(Update, engine::particles::Particle::move_and_handle_collisions.after(gameplay::player::flight))
        .add_systems(Update, gameplay::enemy::enemy_gravity)
        .add_systems(Update, gameplay::enemy::track_player)
        .add_systems(Update, gameplay::enemy::animate_enemy.after(gameplay::enemy::track_player))
        .add_systems(Update, ui::health::update_health_bar)


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