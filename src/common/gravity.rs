use bevy::prelude::*;

// Gravitational acceleration: 1px/frame^2 @60Hz == 3600px/s^2
// const GRAVITY_FORCE: f32 = 3600.;
// const TERMINAL_VELOCITY: f32 = 750.;

#[derive(Resource)]
pub struct GravityResource {
    pub gravity_force: f32,
    pub terminal_velocity: f32,
}

impl GravityResource {
    pub fn new(g_force: f32, t_v: f32 ) -> Self {
        Self {
            gravity_force: g_force,
            terminal_velocity: t_v,
        }
    }
}

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

    pub fn update_g(
        &mut self,
        curr_velocity: &f32,
        deltat: &f32,
        grav_res: &ResMut<GravityResource>,
    ) {
        //self.current_G = GRAVITY;
        let (gravity_force, terminal_velocity) = (grav_res.gravity_force, grav_res.terminal_velocity);
        self.current_g = f32::max(-terminal_velocity, curr_velocity - gravity_force * deltat);
    }
    pub fn update_gravity(
        curr_velocity: &f32,
        deltat: &f32,
        grav_res: &ResMut<GravityResource>,
    ) -> f32{
        let (gravity_force, terminal_velocity) = (grav_res.gravity_force, grav_res.terminal_velocity);
        f32::max(-terminal_velocity, curr_velocity - (gravity_force/70.) * deltat)
    }

    pub fn get_g(&mut self) -> f32 {
        self.current_g
    }
    pub fn reset_g(&mut self) {
        self.current_g = 0.;
    }
}

pub fn initialize( //initializes resource
   commands: &mut Commands,
) {
    (*commands).insert_resource(GravityResource::new(3600., 750.));
}

pub fn change_gravity( //changes gravity
    mut grav_res: ResMut<GravityResource>,
    new_gravity: f32,
    new_terminal_velocity: f32,
) {
    grav_res.gravity_force = new_gravity;
    grav_res.terminal_velocity = new_terminal_velocity;
}
