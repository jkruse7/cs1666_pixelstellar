use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Jump {
    pub is_jumping: bool,
    pub needs_jump: bool,
    pub jumped: bool,
}

impl Jump{
    pub fn new() -> Self {
        Self {
            is_jumping: false,
            needs_jump: false,
            jumped: false,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationFrameCount(usize);

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
}

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

#[derive(Component)]
pub struct EnemyHealth {
    pub hp: f32,
} 

impl EnemyHealth {
    pub fn new(health: f32) -> Self {
        Self {
            hp: health,
        }
    }


    pub fn take_damage(&mut self, amount: f32) {
        self.hp = (self.hp - amount).max(0.0);
    }
}