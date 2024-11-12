#![allow(unused_variables)]


use bevy::{prelude::*, window::PresentMode};
use crate::common::state;
mod common;
mod entities;
mod planets;


// Game constants
const TITLE: &str = "Pixelstellar";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const LEVEL_W: f32 = WIN_W * 3.0;
const LEVEL_H: f32 = WIN_H * 2.0;



fn main() {
    App::new()
        // Resources which will be accessible throughout the game
        .insert_resource(ClearColor(Color::srgb_u8(0, 0, 0)))
        .add_plugins(common::state::StatePlugin)
        .add_plugins(common::death::DeathPlugin)
        // UI Plugins

        .add_plugins(common::ui::menu::MenuPlugin)
        .add_plugins(common::ui::camera::CameraPlugin)
        .add_plugins(common::ui::health_bar::HealthBarPlugin)
        .add_plugins(common::ui::win::WinPlugin)
        .add_plugins(common::ui::end_credits::EndCreditsPlugin)

        // Entity Plugins
        .add_plugins(entities::particle::systems::ParticlePlugin)
        .add_plugins(entities::enemy::systems::EnemyPlugin)
        .add_plugins(entities::player::systems::PlayerPlugin)
        .add_plugins(entities::spaceship::systems::SpaceshipPlugin)

        // Planet Plugins
        .add_plugins(planets::planet_1::Planet1Plugin)

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .init_state::<state::AppState>()
        .add_sub_state::<state::GamePhase>()
        .run();
}
