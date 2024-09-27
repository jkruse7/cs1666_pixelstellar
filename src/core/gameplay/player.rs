use bevy::prelude::*;

enum PlayerType {
    Bird,
    Plane,
    UFO,
    Helicopter,
}



use crate::LEVEL_H;
use crate::LEVEL_W;

const TILE_SIZE: u32 = 100;

const PLAYER_SPEED: f32 = 1200.;
const ACCEL_RATE: f32 = 5000.;


#[derive(Component)]
pub struct Player;

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



pub fn initialize(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let bird_sheet_handle = asset_server.load("image4.png");
    let bird_layout = TextureAtlasLayout::from_grid(UVec2::splat(10), 2, 2, None, None);
    let bird_layout_handle = texture_atlases.add(bird_layout);
    commands.spawn((
        SpriteBundle {
            texture: bird_sheet_handle,
            transform: Transform {
                translation: Vec3::new(0., 0., 900.),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: bird_layout_handle.clone(),
            index: PlayerType::UFO as usize,
        },
        Velocity::new(),
        Player,
    ));
}
pub fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity), (With<Player>)>,
) {
    let (mut pt, mut pv) = player.single_mut();

    let mut deltav = Vec2::splat(0.);

    if input.pressed(KeyCode::KeyA) {
        deltav.x -= 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        deltav.x += 1.;
    }

    if input.pressed(KeyCode::KeyW) {
        deltav.y += 1.;
    }

    if input.pressed(KeyCode::KeyS) {
        deltav.y -= 1.;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;

    pv.velocity = if deltav.length() > 0. {
        (pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if pv.velocity.length() > acc {
        pv.velocity + (pv.velocity.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };
    let change = pv.velocity * deltat;

    let new_pos = pt.translation + Vec3::new(change.x, 0., 0.);
    if new_pos.x >= -(LEVEL_W / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.x <= LEVEL_W / 2. - (TILE_SIZE as f32) / 2.
    {
        pt.translation = new_pos;
    }

    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    if new_pos.y >= -(LEVEL_H / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.y <= LEVEL_H / 2. - (TILE_SIZE as f32) / 2.
    {
        pt.translation = new_pos;
    }
}