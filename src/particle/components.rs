use bevy::prelude::*;
use rand::Rng;
use crate::{engine::hitbox::Hitbox, LEVEL_H};
use super::resources::PARTICLE_SIZE;


#[derive(Component, Debug)]
pub struct ParticlePosition {
    pub grid_x: i32,
    pub grid_y: i32,
}
impl ParticlePosition {
    fn new(grid_x: i32, grid_y: i32) -> Self {
        Self {
            grid_x: grid_x,
            grid_y: grid_y,
        }
    }
}


#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum ParticleData {
    Air,
    BedRock,
    Water,
    Sand,
    Dirt,
    WetDirt,
    Stone,
}


#[derive(Bundle, Debug)]
pub struct Particle {
    position: ParticlePosition,
    data: ParticleData,
    hitbox: Hitbox,
}
pub trait NewParticle {
    const PARTICLE_DATA: ParticleData;
    fn new(x: i32, y: i32) -> Self;
}


#[derive(Component, Debug)]
pub struct ParticleElementBedRock;
#[derive(Bundle, Debug)]
pub struct BedRockParticle {
    sprite: SpriteBundle,
    particle: Particle,
    element: ParticleElementBedRock,
}
impl NewParticle for BedRockParticle {
    const PARTICLE_DATA: ParticleData = ParticleData::BedRock;
    fn new(x: i32, y: i32) -> Self {
        // random color range
        let mut rng = rand::thread_rng();
        let black = rng.gen_range(0..=50) as u8;
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb_u8(black, black, black),
                    custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.,
                        y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.,
                        0.0,
                    ),
                    ..default()
                },
                ..default()
            },
            particle: Particle {
                position: ParticlePosition::new(x, y),
                data: ParticleData::BedRock,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            element: ParticleElementBedRock,
        }
    }
}


// water particle
#[derive(Component)]
pub struct ParticleElementWater;

#[derive(Bundle)]
pub struct WaterParticle {
    sprite: SpriteBundle,
    particle: Particle,
    element: ParticleElementWater,
}

impl NewParticle for WaterParticle {
    const PARTICLE_DATA: ParticleData = ParticleData::Water;
    fn new(x: i32, y: i32) -> Self {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(15..=30) as u8;
        let g = rng.gen_range(129..=144) as u8;
        let b = rng.gen_range(240..=255) as u8;
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba_u8(r, g, b, 128),
                    custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.,
                        y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.,
                        901.0,
                    ),
                    ..default()
                },
                ..default()
            },
            particle: Particle {
                position: ParticlePosition::new(x, y),
                data: ParticleData::Water,
                // correct hitbox:
                // hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
                // incorrect hitbox: (so that player can walk through)
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(LEVEL_H+10., LEVEL_H+10.))
            },
            element: ParticleElementWater,
        }
    }
}

#[derive(Component)]
struct ParticleTypeSand;

#[derive(Bundle)]
struct SandParticle {

}

impl SandParticle {
    // fn new() -> Self {
        
    // }
}
#[derive(Component)]
pub struct ParticleElementDirt;

#[derive(Bundle)]
pub struct DirtParticle {
    sprite: SpriteBundle, 
    particle: Particle, 
    element: ParticleElementDirt,
}

impl NewParticle for DirtParticle {
    const PARTICLE_DATA: ParticleData = ParticleData::Dirt;

    fn new(x: i32, y: i32) -> Self {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(118..=133) as u8;
        let g = rng.gen_range(85..=90) as u8;
        let b = rng.gen_range(43..=73) as u8;
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb_u8(r, g, b),
                    custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.,
                        y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.,
                        0.0,
                    ),
                    ..default()
                },
                ..default()
            },
            particle: Particle {
                position: ParticlePosition::new(x, y),
                data: ParticleData::Dirt,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            element: ParticleElementDirt,
        }
    }
}

#[derive(Component)]
pub struct ParticleElementWetDirt;

#[derive(Bundle)]
pub struct WetDirtParticle {
    sprite: SpriteBundle, 
    particle: Particle, 
    element: ParticleElementWetDirt,
}

impl NewParticle for WetDirtParticle {
    const PARTICLE_DATA: ParticleData = ParticleData::WetDirt;

    fn new(x: i32, y: i32) -> Self {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(118..=133) as u8 - 80;
        let g = rng.gen_range(85..=90) as u8 - 60;
        let b = rng.gen_range(43..=73) as u8 - 40;
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb_u8(r, g, b),
                    custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.,
                        y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.,
                        0.0,
                    ),
                    ..default()
                },
                ..default()
            },
            particle: Particle {
                position: ParticlePosition::new(x, y),
                data: ParticleData::WetDirt,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            element: ParticleElementWetDirt,
        }
    }
}



// the component for queries
#[derive(Component)]
pub struct ParticleElementStone;

// the bundle that makes it a particle
#[derive(Bundle)]
pub struct StoneParticle {
    sprite: SpriteBundle,
    particle: Particle,
    element: ParticleElementStone,
}

// if you are adding new particles (mainly procedural) follow this implementation scheme:
impl NewParticle for StoneParticle {
    const PARTICLE_DATA: ParticleData = ParticleData::Stone;
    fn new(x: i32, y: i32) -> Self {
        // random color range
        let mut rng = rand::thread_rng();
        let gray = rng.gen_range(113..=128) as u8;
        Self {
            // implement its sprite with:
            // 1. color
            // 2. size
            // 3. translation
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb_u8(gray, gray, gray),
                    custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.,
                        y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.,
                        0.0,
                    ),
                    ..default()
                },
                ..default()
            },
            // give the particle its position and data
            particle: Particle {
                position: ParticlePosition::new(x, y),
                data: ParticleData::Stone,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            // name the particle for queries
            element: ParticleElementStone,
        }
    }
}