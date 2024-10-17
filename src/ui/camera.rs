use bevy::prelude::*;

use crate::{
    gameplay::player::Player,
    LEVEL_H,
    LEVEL_W,
    WIN_H,
    WIN_W,
};

const THRESHOLD_X: f32 = 160.;
const THRESHOLD_Y: f32 = 90.;

pub fn initialize(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

pub fn move_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let pt = player.single();

    let x_bound = LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = LEVEL_H / 2. - WIN_H / 2.;
    

    let mut ct = camera.single_mut();
    let x_diff = pt.translation - ct.translation;

    //info!("player: {}, new: {}, {} (bounds: {}, {})", pt.translation, pt.translation.x.clamp(-x_bound, x_bound) - THRESHOLD_X,  pt.translation.y.clamp(-y_bound, y_bound) - THRESHOLD_Y, x_bound, y_bound);
    if x_diff.x > THRESHOLD_X{ ct.translation.x = pt.translation.x.clamp(-x_bound - THRESHOLD_X, x_bound + THRESHOLD_X) - THRESHOLD_X; }
    if x_diff.x < -THRESHOLD_X { ct.translation.x = pt.translation.x.clamp(-x_bound - THRESHOLD_X, x_bound + THRESHOLD_X) + THRESHOLD_X; }
    if x_diff.y > THRESHOLD_Y{ ct.translation.y = pt.translation.y.clamp(-y_bound - THRESHOLD_Y, y_bound + THRESHOLD_Y) - THRESHOLD_Y; }
    if x_diff.y < -THRESHOLD_Y { ct.translation.y = pt.translation.y.clamp(-y_bound - THRESHOLD_Y, y_bound + THRESHOLD_Y) + THRESHOLD_Y; }

}


pub fn mouse_coordinates(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    if let Some(world_position) = 
        window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
    {
        //info!("World coords: {}/{}", world_position.x, world_position.y);
    }
}
