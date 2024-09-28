use bevy::prelude::*;

const ACCELERATION: f32 = 1.048;


#[derive(Component)]
pub struct GravityForce{
    current_force: f32,
}

impl GravityForce {
    pub fn new() -> Self {
        Self {
            current_force: 0.001,
        }
    }
    pub fn get_force(&mut self) -> f32 {
        return self.current_force;
    }

    pub fn update_force(&mut self){
        self.current_force *= ACCELERATION;
    }
    
    /* this function should update force based on time elaspsed 
    pub fn update_force() -> void {
            //current_force: current_force + 0.098; // be mut???
    }*/
}





