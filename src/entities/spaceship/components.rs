use bevy::prelude::*;

#[derive(Component)]
pub struct Spaceship;

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
}

#[derive(Event, Default)]
pub struct FoundSpaceship;

impl Velocity {
    pub fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Self { velocity }
    }
}