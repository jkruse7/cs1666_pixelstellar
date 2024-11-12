use bevy::prelude::*;
use crate::common::state::GamePhase;


pub struct Planet1Plugin;
impl Plugin for Planet1Plugin {
    fn build(&self, app: &mut App) {
        // Startup placements
        app.add_systems(OnEnter(GamePhase::Planet1), crate::common::ui::background::initialize_background);
        app.add_systems(OnEnter(GamePhase::Planet1), crate::common::ui::background::initialize_background);
    }
} 
