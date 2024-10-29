use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationFrameCount(pub usize);

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
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
        info!("Player took damage, current health: {}", self.current);
    }
}


// Capacity is between 0 and 100
#[derive(Component)]
pub struct JetPack {
    pub recharge_rate: f32,
    pub boost_rate: f32,
    pub current: f32,
    pub disabled: bool,
    capacity: f32,
}

impl JetPack {
    pub fn new(recharge_rate: f32, boost_rate: f32) -> Self {
        Self { recharge_rate, boost_rate, disabled: false, current: 0., capacity: 100.}
    }
    pub fn recharge(&mut self){
        self.current = f32::min(self.capacity, self.current + self.recharge_rate);
        if self.current >= self.capacity{
            self.disabled = false;
        }
    }
    pub fn fly(&mut self){
        self.current = f32::max(0., self.current - self.boost_rate);
        if self.current < 0.5{
            self.disabled = true;
        }
    }
}