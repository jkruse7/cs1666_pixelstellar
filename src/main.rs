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
        // NEW PLANETS:
        //  add background in assets/planet_X/background.png (size 100x100)
        //  edit common/ui/background to add the new planet
        //  edit common/state.rs. follow all comments in that file.
        //  add planets/planet_X.rs with your plugin and custom generation
        //  add the plugin here:

        .add_plugins(planets::planet_1::Planet1Plugin)
        .add_plugins(planets::planet_2::Planet2Plugin)
        .add_plugins(planets::planet_3::Planet3Plugin)
        .add_plugins(planets::planet_4::Planet4Plugin)
        .add_plugins(planets::planet_5::Planet5Plugin)
        .add_plugins(planets::planet_6::Planet6Plugin)
        .add_plugins(planets::planet_7::Planet7Plugin)
        .add_plugins(planets::planet_8::Planet8Plugin)

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
