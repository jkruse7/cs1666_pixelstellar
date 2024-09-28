use bevy::prelude::*;

use crate::LEVEL_H;
use crate::LEVEL_W;
use crate::WIN_W;
use crate::WIN_H;

use crate::core::engine::gravity::GravityForce;

const TILE_SIZE: u32 = 100;

const PLAYER_SPEED: f32 = 250.;
const ACCEL_RATE: f32 = 5000.;

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
        GravityForce::new(),
        Player,
    ));
}
pub fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut Sprite, &mut GravityForce), (With<Player>)>,
) {
    let (mut pt, mut pv, mut ps, mut pg) = player.single_mut();

    let mut deltav = Vec2::splat(0.);

    if input.pressed(KeyCode::KeyA) {
        deltav.x -= 1.;
        ps.flip_x = true;
    }

    if input.pressed(KeyCode::KeyD) {
        deltav.x += 1.; 
        ps.flip_x = false;
    }
    if input.pressed(KeyCode::KeyW) {
        //deltav.y = *base force here...*
    }
    else {
        pg.update_force();
        deltav.y -= pg.get_force();
    }


    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;

    /*pv.velocity = if deltav.length() > 0. {
        (pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if pv.velocity.length() > acc {
        pv.velocity + (pv.velocity.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };*/
    pv.velocity = (pv.velocity + (deltav * deltat));
    let change = pv.velocity * deltat;
    info!("Current force: {}, Current y velocity: {}, velocity: {}\n", pg.get_force(), deltav.y, pv.velocity.y);


    // Check if player is within the X bounds
    let new_pos = pt.translation + Vec3::new(change.x, 0., 0.);
    if new_pos.x >= -(LEVEL_W / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.x <= -(LEVEL_W / 2.) + (TILE_SIZE as f32) / 2.
    {
        pt.translation = new_pos;
        //info!("player coords: {}/{}", pt.translation.x, pt.translation.y);
    }

    // Check if player is within the Y bounds
    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    if new_pos.y >= -(LEVEL_H / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.y <= LEVEL_H / 2. - (TILE_SIZE as f32) / 2.
    {
        pt.translation = new_pos;
    }
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
    if v.velocity.cmpne(Vec2::ZERO).any() {
        timer.tick(time.delta());

        if timer.just_finished() {
        texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
         }
    }
}