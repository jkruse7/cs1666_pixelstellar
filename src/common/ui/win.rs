use bevy::prelude::*;
use crate::common::state::AppState;
use crate::entities::player::{components::Player,
blaster::components::Blaster};
use crate::entities::enemy::components::Enemy;
use crate::entities::spaceship::components::Spaceship;

use crate::entities::particle::components::ParticleElement;
use crate::common::ui::{
    background::Background,
    health_bar::HealthBar};

#[derive(Component)]
pub struct WinScreen;

fn setup_win(
    mut commands: Commands, asset_server: Res<AssetServer>) 
{
    commands.spawn((SpriteBundle {
        texture: asset_server.load("win_screen.png"),
        ..default()
    },
WinScreen,));
}

fn clear_level(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Player>, With<Enemy>, With<Background>, With<ParticleElement>, With<HealthBar>, With<Blaster>, With<Spaceship>)>>,
){
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn despawn_win(
    mut commands: Commands,
query: Query<(Entity), With<WinScreen>>,
){
//TODO: Check if collided with blaster particle 
for (entity) in query.iter() {
        commands.entity(entity).despawn();
    
}
}

pub struct WinPlugin;
impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::WinScreen), clear_level)
        .add_systems(OnEnter(AppState::WinScreen), setup_win)
        .add_systems(
            OnExit(AppState::WinScreen), despawn_win);
    }
}