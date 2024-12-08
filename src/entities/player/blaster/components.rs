use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum BlasterType {
    Water,
    Deleter,
    Gas,
    Lava,
    Healing_Spring,
}

#[derive(Component)]
pub struct Blaster;

#[derive(Component, Resource, PartialEq)]
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


#[derive(Component)] //direction blaster is facing
pub struct BlasterVector {
    pub vector: Vec2,
}

impl BlasterVector {
    pub fn new() -> Self {
        Self {
            vector: Vec2::splat(0.),
        }
    }
}

#[derive(Component)]
pub struct BlasterLastFiredTime{
    pub last_fired: f64,
}

impl BlasterLastFiredTime {
    pub fn new() -> Self {
        Self {
            last_fired: 0.,
        }
    }
}

#[derive(Event)]
pub struct ChangeBlasterEvent {
    pub new_blaster_type: BlasterType,
}