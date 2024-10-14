use bevy::prelude::*;

use crate::{
    engine::{
        hitbox::Hitbox,
        gravity::Gravity,
    },
    gameplay::enemy::Enemy,
    LEVEL_H,
    LEVEL_W,
    WIN_W,
    WIN_H,
};


const TILE_SIZE: u32 = 100;
const MAX_FLIGHT_SPEED: f32 = 250.;
const PLAYER_SPEED: f32 = 250.;
const ACCEL_RATE_X: f32 = 5000.;
const ACCEL_RATE_Y: f32 = 10800.;
const ANIM_TIME: f32 = 0.2;

//Julianne 10/8: These will be used for hitbox sizes.
const SPRITE_HEIGHT: u32 = 50;
const SPRITE_WIDTH: u32 = 30;

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

// #[derive(Component)]
// pub struct Health {
//     hp: i32,
// }

// impl Health {
//     fn new() -> Self {
//         Self {
//             hp: 100,
//         }
//     }
// }

#[derive(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
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
        Health::new(100.0),
        Gravity::new(),
        Hitbox::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32, Vec2::new(0., -210.)),
        Player,
    ));
}

pub fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut Sprite, &mut Hitbox, &mut Health), (With<Player>)>,
    mut hitboxes: Query<(&Hitbox), Without<Player>>,
    mut enemy_hitboxes: Query<(&Hitbox), (With<Enemy>, Without<Player>)>,
) {
    let (mut pt, mut pv, mut ps, mut hb, mut player_health) = player.single_mut();
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
    let new_pos = pt.translation + change.extend(0.);
    let new_hb = Hitbox::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32, new_pos.xy());

    
    if new_hb.player_enemy_collision(&enemy_hitboxes){
        info!("updating!");
        player_health.current -=1.;
    }
    if new_pos.x >= -(WIN_W / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.x <= LEVEL_W - (WIN_W / 2. + (TILE_SIZE as f32) / 2.)
        && !new_hb.all_player_collisions(&hitboxes)
    {
        pt.translation = new_pos;
        *hb = new_hb;
    }

}

pub fn flight(
    time: Res<Time>, 
    input: Res<ButtonInput<KeyCode>>, 
    mut player: Query<(&mut Transform, &mut Velocity, &mut Gravity, &mut Hitbox), With<Player>>, 
    mut hitboxes: Query<(&Hitbox), Without<Player>>
) {
    let (mut pt, mut pv, mut pg, mut hb) = player.single_mut();

    let deltat = time.delta_seconds();
    let acc_y = ACCEL_RATE_Y * deltat;

    if input.pressed(KeyCode::Space) {
        pg.reset_g();
        pv.velocity.y = f32::min(MAX_FLIGHT_SPEED, pv.velocity.y + (1. * acc_y))
    }else {
        pg.update_g(&pv.velocity.y, &deltat);
        pv.velocity.y = pg.get_g();
    }

    let change = pv.velocity * deltat;
    let new_pos = pt.translation + change.extend(0.);
    let new_hb = Hitbox::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32, new_pos.xy());
    //Bound player to within level height

    if new_pos.y >= -(WIN_H / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.y <= WIN_H - (TILE_SIZE as f32) / 2.
        && !new_hb.all_player_collisions(&hitboxes)
    {
        pt.translation = new_pos;
        *hb = new_hb;
    }  
    
    let new_hb = Hitbox::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32, new_pos.xy());
    // Velocity is zero when player hits the ground
    if pt.translation.y <= -(LEVEL_H / 2.) + (TILE_SIZE as f32) ||
        new_hb.all_player_collisions(&hitboxes) 
    {
        pv.velocity.y = 0.;
    }
    //assumes the player is a square and pt.translation is the lower-left corner
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