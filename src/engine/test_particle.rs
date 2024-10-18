use bevy::prelude::*;
use rand::{seq::index, Rng};
use crate::{LEVEL_W, LEVEL_H, world::grid::*,};

pub const PARTICLE_SIZE: f32 = 4.;

#[derive(Component)]
pub struct Particle;

#[derive(Component, Copy, Clone, PartialEq)]
pub enum ParticleType {
    Air,
    BedRock,
    Water,
}

impl ParticleType {
    fn get_color(&self) -> Color {
        match self {
            Self::Air => Color::srgba_u8(0, 0, 0,0),
            Self::BedRock => Color::srgba_u8(128, 128, 128, 255),
            Self::Water => Color::srgba_u8(0, 0, 255, 128),
        }
    }
}

pub fn setup_particles(
    grid: Res<Grid>,
    mut commands: Commands,
) {
    for i in 0..grid.w {
        for j in 0..grid.h {
            let index = Index::new(i, j);
            let particle_type = grid.get(index);

            commands
                .spawn(Particle)
                .insert(index)
                .insert(particle_type)
                .insert(SpriteBundle {
                    sprite: Sprite {
                        color: particle_type.get_color(),
                        custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            PARTICLE_SIZE * i as f32 - LEVEL_W / 2. + PARTICLE_SIZE / 2.,
                            PARTICLE_SIZE * j as f32 - LEVEL_H / 2. + PARTICLE_SIZE / 2.,
                            0.,
                        ),
                        ..default()
                    },
                    ..default()
                });
        }
    }
}

pub fn update_particles(
    mut grid: ResMut<Grid>,
    mut particles: Query<(&Index, &mut ParticleType, &mut Sprite)>,
) {
    // generate random water particles on the screen
    let mut rng = rand::thread_rng();
    let i = Index::new(rng.gen_range(0..grid.w), rng.gen_range((grid.h / 2)..grid.h));
    if grid.get(i) == ParticleType::Air {
        grid.set(i, ParticleType::Water);
    }

    for (index, mut block, mut sprite) in &mut particles {
        match *block {
            ParticleType::Air => {},
            ParticleType::BedRock => {},
            // insert the update function for the type of particle
            ParticleType::Water => update_water(&mut grid, *index),
        }

        *block = grid.get(*index);
        sprite.color = block.get_color();
    }
}

// sample water automata method for physics simulation
pub fn update_water(grid: &mut ResMut<Grid>, index: Index) {
    // 1. check the next particle downward
    // 2. move based on particle type
    // SIDE NOTE:   we can add more components to the particles like hitboxes and velocity
    //              and manipulate them around this area
    let mut rng = rand::thread_rng();
    match grid.get(index.down()) {
        ParticleType::Air => {
            // swap current particle with particle below
            grid.set(index, ParticleType::Air);
            grid.set(index.down(), ParticleType::Water);
        },
        ParticleType::BedRock => {
            let next_index = if rng.gen_bool(0.5) {
                index.left().left()
            } else {
                index.right().right()
            };
            match grid.get(next_index) {
                ParticleType::Air => {
                    grid.set(index, ParticleType::Air);
                    grid.set(next_index, ParticleType::Water);
                },
                ParticleType::BedRock => {},
                ParticleType::Water => {},
            }
        },
        ParticleType::Water => {
            let next_index = if rng.gen_bool(0.5) {
                index.left()
            } else {
                index.right()
            };
            match grid.get(next_index) {
                ParticleType::Air => {
                    grid.set(index, ParticleType::Air);
                    grid.set(next_index, ParticleType::Water);
                },
                ParticleType::BedRock => {},
                ParticleType::Water => {},
            }
        },
    }
}