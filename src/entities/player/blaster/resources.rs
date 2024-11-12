use bevy::prelude::*;
use crate::{
    entities::player::blaster::components::*,
};


pub const BLASTER_POWER: f32 = 10.;

// Higher recharge, slower rate of fire
pub const RECHARGE_RATE: f32 = 0.05;
pub const BLASTER_DISPLACEMENT: f32 = 0.; //Distance from blaster to particle being spawned (made so that we dont detect it for splashing)

#[derive(Component, Resource)]
pub struct BlasterSelection {
    pub selected: BlasterType,
}

impl BlasterSelection {
    pub fn new(selection: BlasterType) -> Self {
        Self {
            selected: selection,
        }
    }
}