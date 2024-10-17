// rust and bevy imports
use bevy::{
    prelude::*,
    window::PresentMode,
    ecs::query,
};

// module declarations and imports
mod engine;
mod gameplay;
mod ui;
mod world;

use ui::{
    button::{
        spawn_custom_button,
        button_interaction_system,
    },
    health::{
        setup_health_bar,
        update_health_bar,
    },
    camera::mouse_coordinates,
};
use world::water::setup_water_tiles;

// constants
const TITLE: &str = "Pixelstellar";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const LEVEL_W: f32 = 6400.;
const LEVEL_H: f32 = 3600.;

use engine::particles::PARTICLE_SIZE;
use world::grid;


fn main() {
    App::new()
        // Set the background color for the window
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.75))))
        .insert_resource(StateArray::initialize(LEVEL_W, LEVEL_H, PARTICLE_SIZE))
        // Add default plugins and configure the primary window
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo, // V-Sync on
                ..default()
            }),
            ..default()
        }))
        // Initialize the state machine with AppState
        // .init_state::<engine::update_state::AppState>() 
        // // Set default state to MainMenu on startup
        // .add_startup_system(engine::update_state::setup_app_state)
       
        // // Systems for entering various game states
        // .add_systems(OnEnter(engine::update_state::AppState::MainMenu), ui::menu::spawn_main_menu)
        // // .add_systems(OnEnter(engine::update_state::AppState::InGame), gameplay::start_game)
        // // .add_systems(OnEnter(engine::update_state::AppState::Paused), ui::pause_menu::spawn_pause_menu)
        // // .add_systems(OnEnter(engine::update_state::AppState::Setting), ui::settings_menu::spawn_settings_menu)
        // // .add_systems(OnEnter(engine::update_state::AppState::DevLab), world::lab::enter_dev_lab)
        
        // // // Systems for exiting various game states
        // .add_systems(OnExit(engine::update_state::AppState::MainMenu), ui::menu::cleanup_main_menu)
        // // .add_systems(OnExit(engine::update_state::AppState::InGame), gameplay::cleanup_game)
        // .add_systems(OnExit(engine::update_state::AppState::Paused), ui::pause_menu::cleanup_pause_menu)
        // .add_systems(OnExit(engine::update_state::AppState::Setting), ui::settings_menu::cleanup_settings_menu)
        // .add_systems(OnExit(engine::update_state::AppState::DevLab), world::lab::exit_dev_lab)
        // // Initialize essential components like camera, background, and player
        .add_systems(Startup, ui::camera::initialize)
        .add_systems(Startup, ui::background::initialize)
        //.add_systems(Startup, ui::health::setup_health_bar)
        .add_systems(Startup, gameplay::player::initialize)
        .add_systems(Startup, gameplay::enemy::initialize)
        //.add_systems(Startup, world::floor::initialize)
        .add_systems(Startup, ui::health::setup_health_bar)
        .add_systems(Startup, engine::particles::test_particle_spawn)
        //.add_systems(Startup,setup_system)
        // Systems for updating game state
        .add_systems(Startup, setup)
        .add_systems(Update, world::water::update_water_tiles)
        .add_systems(Update, ui::camera::mouse_coordinates)
        .add_systems(Update, gameplay::player::move_player)
        .add_systems(Update, gameplay::player::flight.after(gameplay::player::move_player))
        .add_systems(Update, gameplay::player::animate_player.after(gameplay::player::move_player))
        //.add_systems(Update, gameplay::enemy::move_enemy)
        .add_systems(Update, gameplay::enemy::enemy_gravity)
        .add_systems(Update, gameplay::enemy::track_player)
        .add_systems(Update, gameplay::enemy::animate_enemy.after(gameplay::enemy::track_player))
        .add_systems(Update, ui::camera::move_camera.after(gameplay::player::move_player))
        .add_systems(Update, button_interaction_system)
        .add_systems(Update, ui::health::update_health_bar)
        .add_systems(Update, engine::particles::Particle::move_and_handle_collisions.after(gameplay::player::flight))
        // Run game logic only in InGame state
        // .add_systems(Update, gameplay::play_game.run_if(in_state(engine::update_state::AppState::InGame)))
        // // Handle pause/resume using ESC key, applicable only in InGame or Paused states
        // .add_systems(Update, engine::update_state::pause_game.run_if(in_state(engine::update_state::AppState::InGame).or(in_state(engine::update_state::AppState::Paused))))
        .run();
}


use crate::grid::StateArray;
use engine::particles::ELEMENT;
use engine::particles::Particle;
use world::perlin_noise::get_1d_pn_value;
fn setup(
    mut commands: Commands,
    mut state_array: ResMut<StateArray>
) {
    let mut x = -3200;
    while x < 0 {
        info!("X:{}. gives: ({}, {})", x, ((x as f32) / 160.), get_1d_pn_value(((x as f32) / 160.), 30., 0.6));
        let (closest_x, y): (usize, usize) = StateArray::get_closest(x as f32, get_1d_pn_value(((x as f32) / 160.),5., 0.6));
        state_array.spawn(&mut commands, closest_x, y, 0);
        x = x + 4;
    }

}