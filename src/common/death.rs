use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::ecs::system::SystemId;


use crate::entities::player::systems::initialize as player_initialize;
use crate::entities::player::components::Player;
use crate::entities::enemy::components::Enemy;
use crate::entities::enemy::systems::initialize as enemy_initalize;

#[derive(Event, Default)]
pub struct Death;

#[derive(Resource)]
struct MyItemSystems(HashMap<String, SystemId>);

fn death_event_listener(
    mut death_event: EventReader<Death>,
    //mut next_state: ResMut<NextState<GameState>>,
    query: Query<(Entity), Or<(With<Player>, With<Enemy>)>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    systems: Res<MyItemSystems>,


) {
    if !death_event.is_empty() {
        info!("player died");
        // We need to reset the level. Re-initialize Player, Enemies, and Camera. The particles will stay the same
        for (entity) in query.iter() {
            commands.entity(entity).despawn();
        }
        commands.run_system(systems.0["player_init"]);
        commands.run_system(systems.0["enemy_init"]);
        death_event.clear();
    }
}

pub struct DeathPlugin;
impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, death_event_listener)
            .add_event::<Death>();
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