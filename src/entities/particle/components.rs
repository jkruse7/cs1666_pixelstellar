use bevy::prelude::*;
use rand::Rng;
use crate::{common::hitbox::Hitbox, LEVEL_H};
use crate::entities::particle::resources::PARTICLE_SIZE;



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
pub enum ParticleElement {
    Air,
    BedRock,
    Water,
    Dirt,
    Stone,
}


pub trait NewParticle {
    const ELEMENT: ParticleElement;
    fn new(x: i32, y: i32) -> Self;
}


#[derive(Bundle, Debug)]
pub struct Particle {
    position: ParticlePosition,
    data: ParticleElement,
    hitbox: Hitbox,
}






//                       Particle Definitions:







// Bedrock ------------------------------------------------------------------------
#[derive(Component, Debug)]
pub struct ParticleElementBedRock;
#[derive(Bundle, Debug)]
pub struct BedRockParticle {
    sprite: SpriteBundle,
    particle: Particle,
    element: ParticleElementBedRock,
}
impl NewParticle for BedRockParticle {
    const ELEMENT: ParticleElement = ParticleElement::BedRock;
    fn new(x: i32, y: i32) -> Self {
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
                data: ParticleElement::BedRock,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            element: ParticleElementBedRock,
        }
    }
}


// Water ------------------------------------------------------------------------
#[derive(Component)]
pub struct ParticleElementWater;
#[derive(Bundle)]
pub struct WaterParticle {
    sprite: SpriteBundle,
    particle: Particle,
    element: ParticleElementWater,
}
impl NewParticle for WaterParticle {
    const ELEMENT: ParticleElement = ParticleElement::Water;
    fn new(x: i32, y: i32) -> Self {
        let mut rng = rand::thread_rng();
        let (r,g,b) = (rng.gen_range(15..=30) as u8, rng.gen_range(129..=144) as u8, rng.gen_range(240..=255) as u8);
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
                data: ParticleElement::Water,
                // correct hitbox:
                // hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
                // incorrect hitbox: (so that player can walk through)
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(LEVEL_H+10., LEVEL_H+10.))
            },
            element: ParticleElementWater,
        }
    }
}



// Dirt ------------------------------------------------------------------------
#[derive(Component)]
pub struct ParticleElementDirt;
#[derive(Bundle)]
pub struct DirtParticle {
    sprite: SpriteBundle, 
    particle: Particle, 
    element: ParticleElementDirt,
}
impl NewParticle for DirtParticle {
    const ELEMENT: ParticleElement = ParticleElement::Dirt;
    fn new(x: i32, y: i32) -> Self {
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
                position: ParticlePosition::new(x, y),
                data: ParticleElement::Dirt,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            element: ParticleElementDirt,
        }
    }
}




// Stone ------------------------------------------------------------------------
#[derive(Component)]
pub struct ParticleElementStone;
#[derive(Bundle)]
pub struct StoneParticle {
    sprite: SpriteBundle,
    particle: Particle,
    element: ParticleElementStone,
}
impl NewParticle for StoneParticle {
    const ELEMENT: ParticleElement = ParticleElement::Stone;
    fn new(x: i32, y: i32) -> Self {
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
                position: ParticlePosition::new(x, y),
                data: ParticleElement::Stone,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            element: ParticleElementStone,
        }
    }
}