use bevy::prelude::*;

// Gravitational acceleration: 1px/frame^2 @60Hz == 3600px/s^2
const GRAVITY_FORCE: f32 = 3600.;
const TERMINAL_VELOCITY: f32 = 750.;

#[derive(Component)]
pub struct Gravity {
    current_g: f32, // I think this is current y velocity
}

impl Gravity {
    pub fn new() -> Self {
        Self {
            current_g: 0.,
        }
    }

    pub fn update_g(&mut self, curr_velocity: &f32, deltat: &f32) {
        //self.current_G = GRAVITY;
        self.current_g = f32::max(-TERMINAL_VELOCITY, curr_velocity - GRAVITY_FORCE * deltat);
    }

    pub fn get_g(&mut self) -> f32 {
        self.current_g
    }
    pub fn reset_g(&mut self) {
        self.current_g = 0.;
    }
}