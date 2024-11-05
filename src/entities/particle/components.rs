use bevy::prelude::*;
use rand::Rng;
use crate::entities::enemy::components::Velocity;
use crate::{common::hitbox::Hitbox, LEVEL_H};
use crate::entities::particle::resources::PARTICLE_SIZE;



#[derive(Component, Debug)]
pub struct ParticlePosVel {
    pub grid_x: i32,
    pub grid_y: i32,
    pub velocity: Vec2,
}
impl ParticlePosVel {
    fn new(grid_x: i32, grid_y: i32, velocity: Vec2) -> Self {
        Self {
            grid_x: grid_x,
            grid_y: grid_y,
            velocity: velocity,
        }
    }
}
#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum ParticleElement {
    Air,
    BedRock,
    Water,
    Gas,
    Dirt,
    Stone,
    Grass,
}


pub trait NewParticle {
    const ELEMENT: ParticleElement;
    fn new(x: i32, y: i32, vel: Vec2) -> Self;
}


#[derive(Bundle, Debug)]
pub struct Particle {
    position: ParticlePosVel,
    data: ParticleElement,
    hitbox: Hitbox,
}






//                       Particle Definitions:







// Bedrock ------------------------------------------------------------------------
#[derive(Component, Debug)]
pub struct ParticleTagBedRock;
#[derive(Bundle, Debug)]
pub struct BedRockParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagBedRock,
}
impl NewParticle for BedRockParticle {
    const ELEMENT: ParticleElement = ParticleElement::BedRock;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
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
                position: ParticlePosVel::new(x, y, vel),
                data: ParticleElement::BedRock,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            tag: ParticleTagBedRock,
        }
    }
}


// Water ------------------------------------------------------------------------
#[derive(Component)]
pub struct ParticleTagWater;
#[derive(Bundle)]
pub struct WaterParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagWater,
}
impl NewParticle for WaterParticle {
    const ELEMENT: ParticleElement = ParticleElement::Water;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let (r,g,b) = (rng.gen_range(15..=30) as u8, rng.gen_range(129..=144) as u8, rng.gen_range(240..=255) as u8);
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba_u8(r, g, b, 150),
                    custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.,
                        y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.,
                        904.0,
                    ),
                    ..default()
                },
                ..default()
            },
            particle: Particle {
                position: ParticlePosVel::new(x, y, vel),
                data: ParticleElement::Water,
                // correct hitbox:
                // hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
                // incorrect hitbox: (so that player can walk through)
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(LEVEL_H+10., LEVEL_H+10.))
            },
            tag: ParticleTagWater,
        }
    }
}


// Gas ------------------------------------------------------------------------
#[derive(Component)]
pub struct ParticleTagGas;
#[derive(Bundle)]
pub struct GasParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagGas,
}
impl NewParticle for GasParticle {
    const ELEMENT: ParticleElement = ParticleElement::Gas;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let (r,g,b) = (rng.gen_range(15..=30) as u8, rng.gen_range(129..=144) as u8, rng.gen_range(15..=30) as u8);
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba_u8(r, g, b, 200),
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
                position: ParticlePosVel::new(x, y, vel),
                data: ParticleElement::Gas,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(LEVEL_H+10., LEVEL_H+10.))
            },
            tag: ParticleTagGas,
        }
    }
}




// Dirt ------------------------------------------------------------------------
#[derive(Component)]
pub struct ParticleTagDirt;
#[derive(Bundle)]
pub struct DirtParticle {
    sprite: SpriteBundle, 
    particle: Particle, 
    tag: ParticleTagDirt,
}
impl NewParticle for DirtParticle {
    const ELEMENT: ParticleElement = ParticleElement::Dirt;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let (r,g, b) = (rng.gen_range(118..=133) as u8, rng.gen_range(85..=90) as u8, rng.gen_range(43..=73) as u8);
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
                position: ParticlePosVel::new(x, y, vel),
                data: ParticleElement::Dirt,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            tag: ParticleTagDirt,
        }
    }
}




// Stone ------------------------------------------------------------------------
#[derive(Component)]
pub struct ParticleTagStone;
#[derive(Bundle)]
pub struct StoneParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagStone,
}
impl NewParticle for StoneParticle {
    const ELEMENT: ParticleElement = ParticleElement::Stone;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        // random color range
        let mut rng = rand::thread_rng();
        let gray = rng.gen_range(113..=128) as u8;
        Self {
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
            particle: Particle {
                position: ParticlePosVel::new(x, y, vel),
                data: ParticleElement::Stone,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            tag: ParticleTagStone,
        }
    }
}




// Grass ------------------------------------------------------------------------
#[derive(Component)]
pub struct ParticleTagGrass;
#[derive(Bundle)]
pub struct GrassParticle {
    sprite: SpriteBundle, 
    particle: Particle, 
    tag: ParticleTagGrass,
}
impl NewParticle for GrassParticle {
    const ELEMENT: ParticleElement = ParticleElement::Grass;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let (r,g, b) = (rng.gen_range(118..=133) as u8, rng.gen_range(220..=230) as u8, rng.gen_range(43..=73) as u8);
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
                position: ParticlePosVel::new(x, y, vel),
                data: ParticleElement::Grass,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            tag: ParticleTagGrass,
        }
    }
}
