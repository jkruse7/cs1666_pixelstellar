use bevy::prelude::*;
use crate::entities::player::components::*; 
use crate::common::state::{AppState, GamePhase};

#[derive(Component)]
pub struct HealthBar;


pub fn initialize_health_bar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Silkscreen-Bold.ttf");

    commands.spawn((
        TextBundle::from_section(
            "Health: 100", // Initial health text
            TextStyle {
                font,
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            width: Val::Px(200.0),
            height: Val::Px(30.0),
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        HealthBar, 
    ));
}


pub fn update_health_bar(
    mut query: Query<(&mut Style, &mut BackgroundColor), With<HealthBar>>, 
    player_query: Query<&Health, With<Player>>,
) {
    let health = player_query.single();
   // info!("Curr Health: {:?}", health.current);
    let percentage = health.current / health.max;

    for (mut style, mut background_color) in query.iter_mut() {
        // Update width based on health
        style.width = Val::Px(200.0 * percentage);
        // Optionally change the color based on health (e.g., green -> red)
        *background_color = Color::srgba(1.0 - percentage, percentage, 0.0, 1.0).into();

    }
}


pub struct HealthBarPlugin;
impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), initialize_health_bar);
        app.add_systems(OnEnter(GamePhase::Planet2), initialize_health_bar);
        app.add_systems(OnEnter(GamePhase::Planet3), initialize_health_bar);
        app.add_systems(OnEnter(GamePhase::Planet4), initialize_health_bar);
        app.add_systems(OnEnter(GamePhase::Planet5), initialize_health_bar);
        app.add_systems(OnEnter(GamePhase::Planet6), initialize_health_bar);
        app.add_systems(OnEnter(GamePhase::Planet7), initialize_health_bar);

        app.add_systems(Update, update_health_bar.run_if(in_state(AppState::InGame)));
    }
}