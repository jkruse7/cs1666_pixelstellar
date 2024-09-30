use bevy::prelude::*;

use crate::core::engine::collision::AABB;
use crate::LEVEL_H;
use crate::LEVEL_W;
use crate::WIN_W;
use crate::WIN_H;

use crate::core::engine::gravity::Gravity;

const TILE_SIZE: u32 = 100;

const MAX_FLIGHT_SPEED: f32 = 500.;
const PLAYER_SPEED: f32 = 250.;
const ACCEL_RATE_X: f32 = 5000.;
const ACCEL_RATE_Y: f32 = 10800.;

const ANIM_TIME: f32 = 0.2;



#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationFrameCount(usize);

#[derive(Component)]
pub struct Velocity {
    velocity: Vec2,
}

impl Velocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Self { velocity }
    }
}

#[derive(Component)]
pub struct Health {
    hp: i32,
}

impl Health {
    fn new() -> Self {
        Self {
            hp: 100,
        }
    }
}


pub fn initialize(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let player_sheet_handle = asset_server.load("walking.png");
    let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 4, 1, None, None);
    let player_layout_len = player_layout.textures.len();
    let player_layout_handle = texture_atlases.add(player_layout);
    commands.spawn((
        SpriteBundle {
            texture: player_sheet_handle,
            transform: Transform {
                translation: Vec3::new(0., -(WIN_H / 2.) + ((TILE_SIZE as f32) * 1.5), 900.),
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
            layout: player_layout_handle,
            index: 0,
        },
        AnimationTimer(Timer::from_seconds(ANIM_TIME, TimerMode::Repeating)),
        AnimationFrameCount(player_layout_len),
        Velocity::new(),
        Health::new(),
        Gravity::new(),
        AABB::new(Vec2::new(0., -(WIN_H / 2.) + ((TILE_SIZE as f32) * 1.5)), Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
        Player,
    ));
}

pub fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut Sprite, &mut Gravity, &mut AABB), (With<Player>)>,
) {
    let (mut pt, mut pv, mut ps, pg, aabb) = player.single_mut();
    let mut deltav_x = 0.;

    if input.pressed(KeyCode::KeyA) {
        deltav_x -= 1.;
        ps.flip_x = true;
    }

    if input.pressed(KeyCode::KeyD) {
        deltav_x += 1.;
        ps.flip_x = false;
    }
    
    let deltat = time.delta_seconds();
    let acc_x = ACCEL_RATE_X * deltat;

    if deltav_x != 0. {
        if pv.velocity.y >= 0. {
            pv.velocity.x = (pv.velocity.x + deltav_x * acc_x).clamp(-PLAYER_SPEED, PLAYER_SPEED);
        }
        else {
            pv.velocity.x = (pv.velocity.x + deltav_x * acc_x).clamp(-PLAYER_SPEED * 0.3, PLAYER_SPEED * 0.3);
        }
    } else if pv.velocity.x.abs() > acc_x {
        pv.velocity.x -= pv.velocity.x.signum() * acc_x;
    } else {
        pv.velocity.x = 0.;
    }

    let change = pv.velocity * deltat;

    // Bound player to within the level width
    pt.translation.x = (pt.translation.x + change.x).clamp(
        -(LEVEL_W / 2.) + (TILE_SIZE as f32) / 2.,
        LEVEL_W / 2. - (TILE_SIZE as f32) / 2.,
    );
}

pub fn flight(
    time: Res<Time>, 
    input: Res<ButtonInput<KeyCode>>, 
    mut player: Query<(&mut Transform, &mut Velocity, &mut Gravity, &mut AABB), With<Player>>, 
) {
    let (mut pt, mut pv, mut pg, mut aabb) = player.single_mut();

    let deltat = time.delta_seconds();
    let acc_y = ACCEL_RATE_Y * deltat;

    if input.pressed(KeyCode::Space) {
        pg.reset_G();
        pv.velocity.y = f32::min(MAX_FLIGHT_SPEED, pv.velocity.y + (1. * acc_y))
    }else {
        pg.update_G(&pv.velocity.y, &deltat);
        pv.velocity.y = pg.get_G();
    }

    let change = pv.velocity * deltat;

    //Bound player to within level height
    pt.translation.y = (pt.translation.y + change.y).clamp(
        -(LEVEL_H / 2.) + (TILE_SIZE as f32)*1.2,
        LEVEL_H / 2. - (TILE_SIZE as f32) / 2.,
    );

    // Velocity is zero when player hits the ground
    if pt.translation.y <= -(LEVEL_H / 2.) + (TILE_SIZE as f32){
        pv.velocity.y = 0.;
    }
    //assumes the player is a square and pt.translation is the lower-left corner
    *aabb = AABB::new(Vec2::new(pt.translation.x , pt.translation.y), Vec2::new(pt.translation.x + TILE_SIZE as f32, pt.translation.y + TILE_SIZE as f32));
}

pub fn animate_player(
    time: Res<Time>,
    mut player: Query<
        (
            &Velocity,
            &mut TextureAtlas,
            &mut AnimationTimer,
            &AnimationFrameCount,
        ),
        With<Player>,
    >,
) {
    let (v, mut texture_atlas, mut timer, frame_count) = player.single_mut();
    let x_vel = Vec2::new(v.velocity.x, 0.);
    //info!(x_vel.x);
    if x_vel.cmpne(Vec2::ZERO).any() {
        timer.tick(time.delta());

        if timer.just_finished() {
        texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
         }
    }
}