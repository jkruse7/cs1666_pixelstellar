use bevy::prelude::*;
use crate::{LEVEL_W, LEVEL_H, world::grid::*,};

pub const PARTICLE_SIZE: f32 = 4.;

#[derive(Copy, Clone, PartialEq)]
pub enum ParticleType {
    Air,
    BedRock,
    Water,
}

impl ParticleType {
    fn get_color(&self) -> Color {
        match self {
            Self::Air => Color::srgb_u8(0, 0, 0),
            Self::BedRock => Color::srgb_u8(128, 128, 128),
            Self::Water => Color::srgb_u8(0, 0, 255),
        }
    }
}

#[derive(Component)]
pub struct Particle {
    position: Vec3,
    block: ParticleType,
    color: Color,
}

impl Particle {
    pub fn new(position: Vec3, block: ParticleType) -> Self {
        Self {
            position: position,
            block: block,
            color: block.get_color(),
        }
    }

    pub fn change_type(&mut self, block: ParticleType) {
        self.block = block;
        self.color = block.get_color();
    }
}

pub fn setup_particles(
    grid: Res<Grid>,
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

    for i in 0..GRID_W as i32 {
        for j in 0..GRID_H as i32 {
            let particle = Particle::new(
                Vec3::new(
                    PARTICLE_SIZE * i as f32 - LEVEL_W / 2. + PARTICLE_SIZE / 2.,
                    PARTICLE_SIZE * j as f32 - LEVEL_H / 2. + PARTICLE_SIZE / 2.,
                    0.,
                ),
                grid.get(i, j),
            );

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: particle.color,
                        custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                        ..default()
                    },
                    transform: Transform {
                        translation: particle.position,
                        ..default()
                    },
                    ..default()
                })
                .insert(particle)
                .insert(Index::new(i, j));
        }
    }
}

pub fn update_particles(
    grid: Res<Grid>,
    mut query: Query<(&mut Sprite, &mut Particle, &Index)>,
) {
    for (mut sprite, mut particle, index) in &mut query {
        particle.change_type(grid.get(index.i, index.j));
        sprite.color = particle.color;
    }
}