use bevy::prelude::*;



//blaster or gun
#[derive(Component)]
pub struct Blaster;

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