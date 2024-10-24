use bevy::prelude::*;
use crate::{
    entities::player::components::*,
    LEVEL_H, LEVEL_W,
    WIN_H, WIN_W,
};

const THRESHOLD_X: f32 = 160.;
const THRESHOLD_Y: f32 = 90.;

#[derive(Component)]
pub struct MainCamera;

pub fn initialize_camera(mut commands: Commands){
    info!("sss");
    commands.spawn((Camera2dBundle::default(), MainCamera));
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

    if x_diff.x > THRESHOLD_X{ ct.translation.x = pt.translation.x.clamp(-x_bound - THRESHOLD_X, x_bound + THRESHOLD_X) - THRESHOLD_X; }
    if x_diff.x < -THRESHOLD_X { ct.translation.x = pt.translation.x.clamp(-x_bound - THRESHOLD_X, x_bound + THRESHOLD_X) + THRESHOLD_X; }
    if x_diff.y > THRESHOLD_Y{ ct.translation.y = pt.translation.y.clamp(-y_bound - THRESHOLD_Y, y_bound + THRESHOLD_Y) - THRESHOLD_Y; }
    if x_diff.y < -THRESHOLD_Y { ct.translation.y = pt.translation.y.clamp(-y_bound - THRESHOLD_Y, y_bound + THRESHOLD_Y) + THRESHOLD_Y; }

}

// Useful for debugging, prints bevy coordinates at the mouse location.
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
        info!("World coords: {}/{}", world_position.x, world_position.y);
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera);
        //app.add_systems(Update, mouse_coordinates);
        app.add_systems(Update, move_camera.after(crate::entities::player::systems::move_player));
    }
}
