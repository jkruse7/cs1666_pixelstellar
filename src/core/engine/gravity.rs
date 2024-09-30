use bevy::prelude::*;

// Gravitational acceleration: 1px/frame^2 @60Hz == 3600px/s^2
const GRAVITY: f32 = 3600.;

#[derive(Component)]
pub struct Gravity {
    current_G: f32,
}

impl Gravity {
    pub fn new() -> Self {
        Self {
            current_G: 0.,
        }
    }

    pub fn update_G(&mut self) {
        self.current_G = GRAVITY;
    }

    pub fn get_G(&mut self) -> f32 {
        self.current_G
    }

    /* this function should update force based on time elaspsed 
    pub fn update_force() -> void {
            //current_force: current_force + 0.098; // be mut???
    }*/
}