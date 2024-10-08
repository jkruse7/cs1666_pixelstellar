use bevy::prelude::*;
use crate::core::gameplay::player::{Health, Player}; 

#[derive(Component)]
pub struct HealthBar;

//const DEFAULT_GREEN_COLOR: Color = Color::srgba(0.0, 1.0, 0.0, 1.0); 


pub fn setup_health_bar(
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
    ));
}

pub fn update_health_bar(
    mut query: Query<(&mut Style, &mut BackgroundColor, &mut Text), With<HealthBar>>, 
    player_query: Query<&Health, With<Player>>,
) {
    let health = player_query.single();
    info!("Curr Health: {:?}", health.current);
    let percentage = health.current / health.max;

    for (mut style, mut background_color, mut text) in query.iter_mut() {
        // Update width based on health
        style.width = Val::Px(200.0 * percentage);
        // Optionally change the color based on health (e.g., green -> red)
        *background_color = Color::rgb(1.0 - percentage, percentage, 0.0).into();

}
}
