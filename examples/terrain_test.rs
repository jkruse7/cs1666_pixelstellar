use bevy::{
    prelude::*,
    window::PresentMode
};
use rand::Rng;

const TITLE: &str = "Test";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const PARTICLE_SIZE: f32 = 4.;

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct HitBox;

#[derive(Component)]
struct Velocity {
    v: Vec2,
}

impl Velocity {
    fn new(x: f32, y: f32) -> Self {
        Self {
            v: Vec2::new(x, y),
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(0, 0, 0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        //.add_systems(Startup, draw_floor)
        .add_systems(Startup, draw_screen)
        .add_systems(Update, move_particles)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw_particle(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb_u8(240, 140, 100),
                custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Particle)
        .insert(HitBox)
        .insert(Velocity::new(rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0)));
}

fn draw_floor(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let mut i: f32 = -WIN_W / 2. + PARTICLE_SIZE / 2.;
    while i <= WIN_W / 2. - PARTICLE_SIZE / 2. {
        commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb_u8(240, 140, 100),
                custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(i, 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Particle)
        .insert(HitBox)
        .insert(Velocity::new(rng.gen_range(-5..5) as f32, rng.gen_range(-5..5) as f32));

        i += PARTICLE_SIZE;
    }
}

fn draw_screen(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let mut i: f32 = -WIN_W / 2. + PARTICLE_SIZE / 2.;
    while i <= WIN_W / 2. - PARTICLE_SIZE / 2. {
        let mut j: f32 = -WIN_H / 2. + PARTICLE_SIZE / 2.;
        while j <= WIN_H / 2. - PARTICLE_SIZE / 2. {
            commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb_u8(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)),
                    custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(i, j, 0.),
                    ..default()
                },
                ..default()
            })
            .insert(Particle)
            .insert(HitBox)
            .insert(Velocity::new(rng.gen_range(-3.0..3.0), rng.gen_range(-3.0..3.0)));
            j += PARTICLE_SIZE;
        }
        i += PARTICLE_SIZE;
    }
}

fn move_particles(
    input: Res<ButtonInput<KeyCode>>,
    mut particles: Query<(&mut Transform, &Velocity), With<Particle>>
) {
    if input.pressed(KeyCode::Space) {
        for (mut transform, velocity) in &mut particles {
            transform.translation.x += velocity.v.x;
            transform.translation.y += velocity.v.y;
        }
    }
}