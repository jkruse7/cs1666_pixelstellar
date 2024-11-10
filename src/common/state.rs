use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::ecs::system::SystemId;


#[derive(Resource)]
struct MyItemSystems(HashMap<String, SystemId>);

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    WinScreen,
    EndCredits,
    Loading,

}
use crate::entities::player::systems::initialize as player_initialize;
use crate::entities::enemy::systems::initialize as enemy_initalize;
use crate::entities::player::{components::Player,
    blaster::components::Blaster};
    use crate::entities::enemy::components::Enemy;
    use crate::entities::spaceship::components::Spaceship;
    
    use crate::entities::particle::components::ParticleElement;
    use crate::common::ui::{
        background::Background,
        health_bar::HealthBar};


#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::InGame)]
pub enum GamePhase {
    #[default]
    Level1,
    Level2,
    //Add other levels here
}

pub fn set_next_state(
    state: Res<State<GamePhase>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    mut next_app_state: ResMut<NextState<AppState>>,

){
    match state.get() {
        GamePhase::Level1 => next_phase.set(GamePhase::Level2),
        // add level transitions here
        //LAST LEVEL CHANGES THE APP STATE
        GamePhase::Level2 => next_app_state.set(AppState::WinScreen),
    }
}

fn clear_level(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Player>, With<Enemy>, With<Background>, With<ParticleElement>, With<HealthBar>, With<Blaster>, With<Spaceship>)>>,

){
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn test(){
    info!("test change state");
}

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GamePhase::Level1),  clear_level);

        let mut my_item_systems = MyItemSystems(HashMap::new());

            my_item_systems.0.insert(
                "player_init".into(),
                app.register_system(player_initialize)
            );
            my_item_systems.0.insert(
                "enemy_init".into(),
                app.register_system(enemy_initalize)
            );
        
            app.insert_resource(my_item_systems);
}
}

