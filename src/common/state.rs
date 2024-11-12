use bevy::prelude::*;
use crate::entities::particle::resources::ParticleMap;
use crate::entities::player::{components::Player,
    blaster::components::Blaster};
    use crate::entities::enemy::components::Enemy;
    use crate::entities::spaceship::components::Spaceship;
    
    use crate::entities::particle::components::ParticleElement;
    use crate::common::ui::{
        background::Background,
        health_bar::HealthBar};


#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum AppState {
        #[default]
        Menu,
        InGame,
        WinScreen,
        EndCredits,
        Loading,
        
    }
        


// src/common/background -> add more

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::InGame)]
pub enum GamePhase {
    #[default]
    Planet1,
    Planet2,
    Planet3,
    //Add other levels here
}

pub fn set_next_state(
    state: Res<State<GamePhase>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    mut next_app_state: ResMut<NextState<AppState>>,

){
    match state.get() {
        GamePhase::Planet1 => next_phase.set(GamePhase::Planet2),
        GamePhase::Planet2 => next_app_state.set(AppState::WinScreen),
        // add level transitions here
        //LAST LEVEL CHANGES THE APP STATE
        GamePhase::Planet3 => next_app_state.set(AppState::WinScreen),
    }
}

fn clear_level(
    mut commands: Commands,
    mut map: ResMut<ParticleMap>,
    query: Query<Entity, Or<(With<Player>, With<Enemy>, With<Background>, With<ParticleElement>, With<HealthBar>, With<Blaster>, With<Spaceship>)>>,

){
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    map.reset();
}

pub struct StatePlugin; 
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GamePhase::Planet1),  clear_level);
        app.add_systems(OnExit(GamePhase::Planet2),  clear_level);
}
}

