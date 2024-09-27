use bevy::prelude::*;


#[derive(Component)]
pub struct GravityForce{
    pub current_force: f32,
}

impl GravityForce {
    pub fn new() -> Self {
        Self {
            current_force: 0.,
        }
    }

    /* this function should update force based on time elaspsed 
    pub fn update_force() -> void {
            //current_force: current_force + 0.098; // be mut???
    }*/
}

/*pub fn get_force(
    obj: Query<(&mut Transform, &mut Velocity, &mut Sprite), (With<GravityForce>)>,
) {
    let (mut transform, mut pv, mut ps) = player.single_mut();
}*/