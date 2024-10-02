use bevy::ecs::query;
use bevy::{prelude::*, window::PresentMode};

mod core;

use crate::core::ui::button::{spawn_custom_button, button_interaction_system};
use crate::core::ui::camera::{mouse_coordinates};

const TITLE: &str = "Pixelstellar";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const LEVEL_W: f32 = 6300.;
const LEVEL_H: f32 = 3600.;

fn main() {
    App::new()
        // Set the background color for the window
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.75))))
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
        // .init_state::<core::engine::update_state::AppState>() 
        // // Set default state to MainMenu on startup
        // .add_startup_system(core::engine::update_state::setup_app_state)
       
        // // Systems for entering various game states
        // .add_systems(OnEnter(core::engine::update_state::AppState::MainMenu), core::ui::menu::spawn_main_menu)
        // // .add_systems(OnEnter(core::engine::update_state::AppState::InGame), core::gameplay::start_game)
        // // .add_systems(OnEnter(core::engine::update_state::AppState::Paused), core::ui::pause_menu::spawn_pause_menu)
        // // .add_systems(OnEnter(core::engine::update_state::AppState::Setting), core::ui::settings_menu::spawn_settings_menu)
        // // .add_systems(OnEnter(core::engine::update_state::AppState::DevLab), core::world::lab::enter_dev_lab)
        
        // // // Systems for exiting various game states
        // .add_systems(OnExit(core::engine::update_state::AppState::MainMenu), core::ui::menu::cleanup_main_menu)
        // // .add_systems(OnExit(core::engine::update_state::AppState::InGame), core::gameplay::cleanup_game)
        // .add_systems(OnExit(core::engine::update_state::AppState::Paused), core::ui::pause_menu::cleanup_pause_menu)
        // .add_systems(OnExit(core::engine::update_state::AppState::Setting), core::ui::settings_menu::cleanup_settings_menu)
        // .add_systems(OnExit(core::engine::update_state::AppState::DevLab), core::world::lab::exit_dev_lab)
        // // Initialize essential components like camera, background, and player
        .add_systems(Startup, core::ui::camera::initialize)
        .add_systems(Startup, core::ui::background::initialize)
        .add_systems(Startup, core::gameplay::player::initialize)
        .add_systems(Startup, core::world::floor::initialize)
        //.add_systems(Startup,setup_system)
        // Systems for updating game state
        .add_systems(Update, core::ui::camera::mouse_coordinates)
        .add_systems(Update, core::gameplay::player::move_player)
        .add_systems(Update, core::gameplay::player::flight)
        .add_systems(Update, core::gameplay::player::animate_player.after(core::gameplay::player::move_player))
        .add_systems(Update, core::ui::camera::move_camera.after(core::gameplay::player::move_player))
        .add_systems(Update, button_interaction_system)
        // Run game logic only in InGame state
        // .add_systems(Update, core::gameplay::play_game.run_if(in_state(core::engine::update_state::AppState::InGame)))
        // // Handle pause/resume using ESC key, applicable only in InGame or Paused states
        // .add_systems(Update, core::engine::update_state::pause_game.run_if(in_state(core::engine::update_state::AppState::InGame).or(in_state(core::engine::update_state::AppState::Paused))))
        .run();
}


//just for test, will be removed
fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    let font = asset_server.load("fonts/Silkscreen-Bold.ttf");

    spawn_custom_button(
        &mut commands,         
        "Start Game",           
        Vec2::new(160.0, 40.0),
        Some(Vec2::new(0.,0.)),
        font,                   
        Some(Color::srgba(0.15, 0.15, 0.15, 1.0)), // 默认颜色
        Some(Color::srgba(0.25, 0.25, 0.25, 1.0)), // 悬停颜色
        Some(Color::srgba(0.35, 0.15, 0.35, 1.0)), // 按下颜色
        None,              
    );
}






