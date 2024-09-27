use bevy::prelude::*;


#[derive(Component)]
pub struct GravityForce{
    currrent_force: f32,
}

impl GravityForce {
    pub fn new() -> Self {
        Self {
            currrent_force: 0.,
        }
    }
}

pub fn get_force(
    obj: Query<(&mut Transform, &mut Velocity, &mut Sprite), (With<GravityForce>)>,
) {
    let (mut transform, mut pv, mut ps) = player.single_mut();
}