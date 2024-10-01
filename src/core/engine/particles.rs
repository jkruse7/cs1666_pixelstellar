use bevy::prelude::*;

// 10/1 Julianne: base struct for all elements
// This should include all the componenets that will 
// be needed to create and work with different elements

enum MATTER_STATE {
    LIQUID,
    GAS, 
    SOLID,
}

#[derive(Component)]
pub struct Element {
    has_gravity: bool,
    state: MATTER_STATE
    collision: bool,
    iterate_for_collision: bool,
    



}