use bevy::prelude::*;

use crate::LEVEL_H;
use crate::LEVEL_W;
use crate::WIN_H;
use crate::WIN_W;

use crate::core::gameplay::player::Player;


pub fn initialize(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

pub fn move_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let pt = player.single();
    let mut ct = camera.single_mut();

    let x_bound = LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = LEVEL_H / 2. - WIN_H / 2.;
    ct.translation.x = pt.translation.x.clamp(-x_bound, x_bound);
    ct.translation.y = pt.translation.y.clamp(-y_bound, y_bound);
}
