use bevy::prelude::*;
use crate::{
    //common::hitbox::Hitbox, 
    //particle::resources::*,
    entities::enemy::components::Enemy, 
    entities::player::components::Player,
    entities::particle::{components::*, resources::*},
    WIN_H
};
use super::{components::*, resources::*};


pub fn initialize(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    player: Query<&Transform, With<Player>>,
){ 
    let blaster_sheet_handle = asset_server.load("blasters.png");
    let blaster_layout = TextureAtlasLayout::from_grid(UVec2::new(19, 11), 4, 1, None, None);
    let blaster_layout_handle = texture_atlases.add(blaster_layout);
    let pt = player.single();


    commands.insert_resource(BlasterSelection {
        selected: BlasterType::Water,
    });

    
    commands.spawn((
        SpriteBundle {
            texture: blaster_sheet_handle,
            transform: Transform {
                translation: Vec3::new(pt.translation.x, pt.translation.y, 901.),
                ..default()
            },
            sprite: Sprite {
                // Flip the logo to the left
                flip_x: false,
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: blaster_layout_handle,
            ..Default::default()
        },
        BlasterVector::new(),
        BlasterLastFiredTime::new(),
        Blaster,)
    );
}


pub fn update_blaster_aim( //this gets window cursor position, not world position (https://bevy-cheatbook.github.io/cookbook/cursor2world.html)
    q_camera: Query<(&Camera, &GlobalTransform), With<crate::common::ui::camera::MainCamera>>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    q_player: Query<&mut Transform, With<Player>>, 
    mut q_blaster_transform: Query<(&mut Transform, &mut BlasterVector, &mut Sprite), (With<Blaster>, Without<Enemy>, Without<Player>)>,

) {

    let (mut blaster_transform, mut blaster_vector, mut blaster_sprite) = q_blaster_transform.single_mut();
    let player_transform = q_player.single();
    let mut cursor_pos = Vec2::new(0., 0.);
    let update_aim_vec = get_game_coords(&mut cursor_pos, q_windows, q_camera);
    //info!("Cursor world position: {:?}", cursor_pos);
    // info! ("Cursor pos: {}/{}", cursor_pos.x, cursor_pos.y);
    if update_aim_vec {
        let player_pos = player_transform.translation;
        let aim_vec = cursor_pos - player_pos.truncate();
        blaster_vector.vector = aim_vec.normalize();
        blaster_transform.rotation = Quat::from_rotation_z(aim_vec.y.atan2(aim_vec.x));
        //inverts the blaster if the blaster is facing left
        if blaster_vector.vector.x < 0. {
            blaster_sprite.flip_y = true;
        } else {
            blaster_sprite.flip_y = false;
        }
    }
}

pub fn shoot_blaster(
    mut map: ResMut<ParticleMap>,
    time: Res<Time>,
    mut commands: Commands,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut q_blaster: Query<(&Transform, &BlasterVector, &mut BlasterLastFiredTime), (With<Blaster>, Without<Enemy>, Without<Player>)>,
    blaster_selection: Res<BlasterSelection>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    let (blaster_transform, blaster_vector, mut blaster_last_fired_time) = q_blaster.single_mut();
    let time_since_last_fired = (time.elapsed_seconds_f64() - blaster_last_fired_time.last_fired) as f32;
    
    if buttons.pressed(MouseButton::Left){
        match blaster_selection.selected {
            BlasterType::Water => {
                if let Some(world_position) = 
                    window.cursor_position()
                        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                        .map(|ray| ray.origin.truncate())
                    {
                    let size = 1.;
                    let mut y: f32 = -size * PARTICLE_SIZE;
    
                if time_since_last_fired > RECHARGE_RATE{
                    blaster_last_fired_time.last_fired = time.elapsed_seconds_f64();
                    if let Some(cursor_position) = window.cursor_position() {
                        if let Some(world_position) = camera.viewport_to_world(camera_transform, cursor_position) {
                            let mut direction = (world_position.origin.truncate() - blaster_transform.translation.truncate()).normalize() * BLASTER_POWER;
                            
                            let position = (convert_to_grid_position(blaster_transform.translation.x + blaster_vector.vector.normalize().x*BLASTER_DISPLACEMENT, blaster_transform.translation.y + blaster_vector.vector.normalize().y*BLASTER_DISPLACEMENT)); //shoot slightly in the direction of the cursor
                            map.insert_at::<WaterParticle>(&mut commands, position, ListType::OnlyAir);
                            map.give_velocity(&mut commands, position, direction);  
                        }
                    }
                }
                }
            }
            BlasterType::Deleter => {
                
                if let Some(world_position) = 
                    window.cursor_position()
                        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                        .map(|ray| ray.origin.truncate())
                {
                    let size = 2.;
                    let mut y: f32 = -size * PARTICLE_SIZE;
                    while y < size * PARTICLE_SIZE + 0.1{
                        let mut x: f32 = -size * PARTICLE_SIZE;
                        while x < size * PARTICLE_SIZE + 0.1{
                            let position = (((world_position.x+x) / PARTICLE_SIZE) as i32, ((world_position.y+y) / PARTICLE_SIZE) as i32);
                            map.delete_at(&mut commands, (((world_position.x+x) / PARTICLE_SIZE) as i32, ((world_position.y+y) / PARTICLE_SIZE) as i32));
                            x += PARTICLE_SIZE;
                        }
                        y += PARTICLE_SIZE;
                    }
                }
                
            }
            BlasterType::Gas => {
                if let Some(world_position) = 
                        window.cursor_position()
                            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                            .map(|ray| ray.origin.truncate())
                {
                    let size = 1.;
                    let mut y: f32 = -size * PARTICLE_SIZE;
                    while y < size * PARTICLE_SIZE + 0.1{
                        let mut x: f32 = -size * PARTICLE_SIZE;
                        while x < size * PARTICLE_SIZE + 0.1{
                            let position = (((world_position.x+x) / PARTICLE_SIZE) as i32, ((world_position.y+y) / PARTICLE_SIZE) as i32);
                            map.insert_at::<GasParticle>(&mut commands, (position.0, position.1), ListType::OnlyAir);
                            x += PARTICLE_SIZE;
                        }
                        y += PARTICLE_SIZE;
                    }
                }
            }
            BlasterType::Lava => {
                if let Some(world_position) = 
                window.cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    let size = 1.;
                    let mut y: f32 = -size * PARTICLE_SIZE;

                    if time_since_last_fired > RECHARGE_RATE{
                        blaster_last_fired_time.last_fired = time.elapsed_seconds_f64();
                        if let Some(cursor_position) = window.cursor_position() {
                            if let Some(world_position) = camera.viewport_to_world(camera_transform, cursor_position) {
                                let mut direction = (world_position.origin.truncate() - blaster_transform.translation.truncate()).normalize() * BLASTER_POWER;
                                
                                let position = (convert_to_grid_position(blaster_transform.translation.x + blaster_vector.vector.normalize().x*BLASTER_DISPLACEMENT, blaster_transform.translation.y + blaster_vector.vector.normalize().y*BLASTER_DISPLACEMENT)); //shoot slightly in the direction of the cursor
                                map.insert_at::<LavaParticle>(&mut commands, position, ListType::OnlyAir);
                                map.give_velocity(&mut commands, position, direction);  
                            }
                        }
                    }
                }
            }
        }
       
    }
}

fn get_game_coords( //gets window cursor pos and converts to world position
    coords: &mut Vec2,
    q_window: Query<&Window, With<bevy::window::PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<crate::common::ui::camera::MainCamera>>,
) -> bool {

    // get the camera info and transform
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        *coords = world_position;
        // info!("World coords: {}/{}", coords.x, coords.y);
        return true;
    }
    false
}

fn change_blaster_sprite(
    //mut q_blaster: Query<(&Blaster, &mut TextureAtlas), With<Blaster>>,
    mut texture_atlas: &mut TextureAtlas,
    blaster_selection: &ResMut<BlasterSelection>,
) { 
    let blaster_type = &blaster_selection.selected;
    texture_atlas.index = match blaster_type {
        BlasterType::Water => 0,
        BlasterType::Deleter => 1,
        BlasterType::Gas => 2,
        BlasterType::Lava => 3,
    };    
}

pub fn handle_blaster_change_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<ChangeBlasterEvent>,
    blaster_selection: Res<BlasterSelection>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        event_writer.send(ChangeBlasterEvent { new_blaster_type: BlasterType::Water });
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        event_writer.send(ChangeBlasterEvent { new_blaster_type: BlasterType::Deleter });
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        event_writer.send(ChangeBlasterEvent { new_blaster_type: BlasterType::Gas });
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        event_writer.send(ChangeBlasterEvent { new_blaster_type: BlasterType::Lava });
    }
}

pub fn change_blaster_on_event(
    mut q_blaster: Query<(&Blaster, &mut TextureAtlas), With<Blaster>>,
    mut events: EventReader<ChangeBlasterEvent>,
    mut blaster_selection: ResMut<BlasterSelection>,
    mut q_window: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
) {
    let mut window: Mut<'_, Window> = q_window.single_mut();
    let (_, mut texture_atlas) = q_blaster.single_mut();

    for ev in events.read() {
        blaster_selection.selected = ev.new_blaster_type;
        change_blaster_sprite(&mut texture_atlas, &blaster_selection);

        if blaster_selection.selected == BlasterType::Deleter {
            set_cursor(&mut window, CursorIcon::Cell);
        } else if window.cursor.icon == CursorIcon::Crosshair {
            set_cursor(&mut window, CursorIcon::Crosshair);
        }
    }
}

fn set_cursor(window: &mut Window, cursor_icon: CursorIcon) {
    window.cursor.icon = cursor_icon;
}
