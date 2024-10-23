use bevy::prelude::*;
use rand::Rng;
use crate::{engine::hitbox::Hitbox, LEVEL_H};
use super::resources::PARTICLE_SIZE;


#[derive(Component, Debug, Clone, Copy)]
pub struct ParticlePosition {
    pub grid_x: i32,
    pub grid_y: i32,
}
impl ParticlePosition {
    pub fn new(grid_x: i32, grid_y: i32) -> Self {
        Self {
            grid_x: grid_x,
            grid_y: grid_y,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct ParticleEntity {
    pub entity_id: Option<Entity>,
}
impl ParticleEntity {
    pub fn new(entity_id: Option<Entity>) -> Self {
        Self {
            entity_id,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum ParticleElement {
    Air,
    BedRock,
    Water,
    Dirt,
    WetDirt,
    Stone,
}


#[derive(Component, Clone, Copy)]
pub struct ParticleElementBedRock;

#[derive(Component, Clone, Copy)]
pub struct ParticleElementWater;

#[derive(Component, Clone, Copy)]
pub struct ParticleElementDirt;

#[derive(Component, Clone, Copy)]
pub struct ParticleElementWetDirt;

#[derive(Component, Clone, Copy)]
pub struct ParticleElementStone;


#[derive(Component, Clone, Copy)]
pub enum ParticleType {
    BedRock(ParticleElementBedRock),
    Water(ParticleElementWater),
    Dirt(ParticleElementDirt),
    WetDirt(ParticleElementWetDirt),
    Stone(ParticleElementStone),
    // Add other particle types as needed
}
impl ParticleType{
    pub fn new(particle: ParticleType) -> Self {
        match particle {
            ParticleType::BedRock(element) => ParticleType::BedRock(element),
            ParticleType::Water(element) => ParticleType::Water(element),
            ParticleType::Dirt(element) => ParticleType::Dirt(element),
            ParticleType::WetDirt(element) => ParticleType::WetDirt(element),
            ParticleType::Stone(element) => ParticleType::Stone(element),
            // Handle other particle types as needed
        }
    }
}
impl ParticleType {
    pub fn from_element(element: ParticleElement) -> Self {
        match element {
            ParticleElement::BedRock => ParticleType::BedRock(ParticleElementBedRock),
            ParticleElement::Water => ParticleType::Water(ParticleElementWater),
            ParticleElement::Dirt => ParticleType::Dirt(ParticleElementDirt),
            ParticleElement::WetDirt => ParticleType::WetDirt(ParticleElementWetDirt),
            ParticleElement::Stone => ParticleType::Stone(ParticleElementStone),
            ParticleElement::Air => panic!("Air does not correspond to a ParticleType"),
        }
    }
}

#[derive(Bundle, Clone, Copy)]
pub struct Particle {
    pub position: ParticlePosition,
    pub entity: ParticleEntity,
    pub element: ParticleElement,
    pub hitbox: Hitbox,
    pub typpe: ParticleType,
}
impl Default for Particle {
    fn default() -> Self {
        Self {
            position: ParticlePosition::new(0, 0),
            entity: ParticleEntity::new(None),
            element: ParticleElement::Air,
            hitbox: Hitbox::new(0., 0., Vec2::splat(0.)), 
            typpe: ParticleType::new(ParticleType::BedRock(ParticleElementBedRock)),
        }
    }
}

pub trait NewParticle {
    const ELEMENT: ParticleElement;
    fn new(x: i32, y: i32) -> Self;
}



#[derive(Bundle)]
pub struct BedRockParticle {
    sprite: SpriteBundle,
    particle: Particle,
    element: ParticleElementBedRock,
}
impl NewParticle for BedRockParticle {
    const ELEMENT: ParticleElement = ParticleElement::BedRock;
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
                element: ParticleElement::BedRock,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.)),
                entity: ParticleEntity::new(None),
                typpe: ParticleType::new(ParticleType::BedRock(ParticleElementBedRock)),
            },
            element: ParticleElementBedRock,
        }
    }
}



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
                element: ParticleElement::Water,
                // correct hitbox:
                // hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
                // incorrect hitbox: (so that player can walk through)
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(LEVEL_H+10., LEVEL_H+10.)),
                entity: ParticleEntity::new(None),
                typpe: ParticleType::new(ParticleType::Water(ParticleElementWater)),
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
}



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
                element: ParticleElement::Dirt,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.)),
                entity: ParticleEntity::new(None),
                typpe: ParticleType::new(ParticleType::Dirt(ParticleElementDirt)),
            },
            element: ParticleElementDirt,
        }
    }

}



#[derive(Bundle)]
pub struct WetDirtParticle {
    sprite: SpriteBundle, 
    particle: Particle, 
    element: ParticleElementWetDirt,
}

impl NewParticle for WetDirtParticle {
    const ELEMENT: ParticleElement = ParticleElement::WetDirt;

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
                element: ParticleElement::WetDirt,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.)),
                entity: ParticleEntity::new(None),
                typpe: ParticleType::new(ParticleType::WetDirt(ParticleElementWetDirt)),
            },
            element: ParticleElementWetDirt,
        }
    }
}



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
                element: ParticleElement::Stone,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.)),
                entity: ParticleEntity::new(None),
                typpe: ParticleType::new(ParticleType::Stone(ParticleElementStone)),
            },
            element: ParticleElementStone,
        }
    }
}