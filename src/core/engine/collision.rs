use bevy::ecs::query;
use bevy::{prelude::*, window::PresentMode};
use crate::core::gameplay::player::Player;
use crate::core::ui::button::{spawn_custom_button, button_interaction_system};
use crate::core::ui::camera::{mouse_coordinates};
#[derive(Component)]
#[derive(Debug)]
pub struct AABB { //Axis-Aligned Bounding Box
    min: Vec2,  //The minimum point of the bounding box (lower-left corner)
    max: Vec2,  //The maximum point of the bounding box (upper-right corner)
}

impl AABB { //TODO: Figure out what happens when someone goes past the boundaries
    
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    fn collides_with(&self, other: &AABB) -> bool {     //Function to check if two AABBs are colliding
        //makes sure they aren't the same object
        if std::ptr::eq(self, other) {
            return false;
        }
        //info!("Collision between types {:?} and {:?}", self.type_id(), other.type_id());
        
        self.min.x < other.max.x && self.max.x > other.min.x &&
        self.min.y < other.max.y && self.max.y > other.min.y
    }

}

pub(crate) fn collision_system(
    mut commands: Commands,
    player_query: Query<(Entity, &AABB, &Player)>,
    aabb_query: Query<(Entity, &AABB)>,
) { //currently only checks for collisions between the player and other AABBs, need to change it for all desired entities
    for (player_entity, player_aabb, _) in player_query.iter() {
        for (aabb_entity, aabb_entity_aabb) in aabb_query.iter() {
            if player_aabb.collides_with(aabb_entity_aabb) {
                //info!("Collision detected between player and floor");
                info!("Collision detected between player and floor at {:?} and {:?}", player_aabb, aabb_entity_aabb);
                //commands.entity(player_entity).despawn();
            }
        }
    }
}