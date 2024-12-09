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
    Obsidian,
    Hellstone,
    Lava,
    AcidicDirt,
    ToxicGas,
    Snow,
    Healing_Spring,
    Sand,
    QuickSand,
<<<<<<< HEAD
    Ice,
=======
    Slime,
>>>>>>> 514c8f097b63156f8f8a96daeff44dc29f5f0c12
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

// Obsidian ------------------------------------------------------------------------
#[derive(Component, Debug)]
pub struct ParticleTagObsidian;
#[derive(Bundle, Debug)]
pub struct ObsidianParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagObsidian,
}
impl NewParticle for ObsidianParticle {
    const ELEMENT: ParticleElement = ParticleElement::Obsidian;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let red = rng.gen_range(30..=50) as u8;
        let green = rng.gen_range(0..=20) as u8;
        let blue = rng.gen_range(75..=100) as u8;
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb_u8(red, green, blue),
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
                data: ParticleElement::Obsidian,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            tag: ParticleTagObsidian,
        }
    }
}

// Hellstone ------------------------------------------------------------------------
#[derive(Component, Debug)]
pub struct ParticleTagHellstone;
#[derive(Bundle, Debug)]
pub struct HellstoneParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagHellstone,
}
impl NewParticle for HellstoneParticle {
    const ELEMENT: ParticleElement = ParticleElement::Hellstone;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let red = rng.gen_range(90..=120) as u8;
        let green = rng.gen_range(10..=30) as u8;
        let blue = rng.gen_range(5..=15) as u8;
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb_u8(red, green, blue),
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
                data: ParticleElement::Hellstone,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            tag: ParticleTagHellstone,
        }
    }
}

// Lava ------------------------------------------------------------------------
#[derive(Component)]
pub struct ParticleTagLava;
#[derive(Bundle)]
pub struct LavaParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagLava,
}
impl NewParticle for LavaParticle {
    const ELEMENT: ParticleElement = ParticleElement::Lava;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let (r,g,b) = (rng.gen_range(200..=255) as u8, rng.gen_range(80..=120) as u8, rng.gen_range(43..=73) as u8);
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba_u8(r, g, b, 220),
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
                data: ParticleElement::Lava,
                // correct hitbox:
                // hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
                // incorrect hitbox: (so that player can walk through)
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(LEVEL_H+10., LEVEL_H+10.))
            },
            tag: ParticleTagLava,
        }
    }
}


// Healing_Spring ------------------------------------------------------------------------
#[derive(Component)]
pub struct ParticleTagHealing_Spring;
#[derive(Bundle)]
pub struct Healing_SpringParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagHealing_Spring,
}
impl NewParticle for Healing_SpringParticle {
    const ELEMENT: ParticleElement = ParticleElement::Healing_Spring;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let (r,g,b) = (rng.gen_range(118..=133) as u8, rng.gen_range(220..=230) as u8, rng.gen_range(10..=40) as u8);
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba_u8(r, g, b, 220),
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
                data: ParticleElement::Healing_Spring,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(LEVEL_H+10., LEVEL_H+10.))
            },
            tag: ParticleTagHealing_Spring,
        }
    }
}


// Sand ------------------------------------------------------------------------
#[derive(Component, Debug)]
pub struct ParticleTagSand;
#[derive(Bundle, Debug)]
pub struct SandParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagSand,
}
impl NewParticle for SandParticle {
    const ELEMENT: ParticleElement = ParticleElement::Sand;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let red = rng.gen_range(210..=240) as u8;
        let green = rng.gen_range(190..=210) as u8;
        let blue = rng.gen_range(80..=110) as u8;
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb_u8(red, green, blue),
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
                data: ParticleElement::Sand,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            tag: ParticleTagSand,
        }
    }
}

// Quick Sand ------------------------------------------------------------------------
#[derive(Component, Debug)]
pub struct ParticleTagQuickSand;
#[derive(Bundle, Debug)]
pub struct QuickSandParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagQuickSand,
}
impl NewParticle for QuickSandParticle {
    const ELEMENT: ParticleElement = ParticleElement::QuickSand;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let red = rng.gen_range(240..=250) as u8;
        let green = rng.gen_range(180..=200) as u8;
        let blue = rng.gen_range(80..=110) as u8;
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb_u8(red, green, blue),
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
                data: ParticleElement::QuickSand,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(LEVEL_H+10., LEVEL_H+10.))
                        },
            tag: ParticleTagQuickSand,
        }
    }
}

<<<<<<< HEAD
// Snow --------------------------------------------------------------------------------------------------------
#[derive(Component, Debug)]
pub struct ParticleTagSnow;
#[derive(Bundle, Debug)]
pub struct SnowParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagSnow,
}
impl NewParticle for SnowParticle {
    const ELEMENT: ParticleElement = ParticleElement::Snow;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let w = rng.gen_range(235..=255) as u8;
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb_u8(w, w, w),
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
                data: ParticleElement::Snow,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(LEVEL_H+10., LEVEL_H+10.))
            },
            tag: ParticleTagSnow,
        }
    }
}


// Ice -----------------------------------------------------------------------------------------------------------
#[derive(Component, Debug)]
pub struct ParticleTagIce;
#[derive(Bundle, Debug)]
pub struct IceParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagIce,
}
impl NewParticle for IceParticle {
    const ELEMENT: ParticleElement = ParticleElement::Ice;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let r0 = 115;
        let g0 = 155;
        let b0 = 208;
        let bound = 10;

        let mut r = rng.gen_range((r0 - bound)..=(r0 + bound));
        let mut g = rng.gen_range((g0 - bound)..=(g0 + bound));
        let mut b = rng.gen_range((b0 - bound)..=(b0 + bound));
        if rng.gen_range(0..=100) <= 25 {
            r += 30;
            g += 30;
            b += 30;
        }
        let w = rng.gen_range(235..=255) as u8;
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
                data: ParticleElement::Ice,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
                        },
            tag: ParticleTagIce,
        }
    }
}
              
              
              
              
              
              
              
// AcidicDirt ------------------------------------------------------------------------
#[derive(Component, Debug)]
pub struct ParticleTagAcidicDirt;
#[derive(Bundle, Debug)]
pub struct AcidicDirtParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagAcidicDirt,
}
impl NewParticle for AcidicDirtParticle {
    const ELEMENT: ParticleElement = ParticleElement::AcidicDirt;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let red = rng.gen_range(10..=30) as u8;
        let green = rng.gen_range(20..=30) as u8;
        let blue = rng.gen_range(5..=15) as u8;
=======
// Slime ------------------------------------------------------------------------
#[derive(Component, Debug)]
pub struct ParticleTagSlime;

#[derive(Bundle, Debug)]
pub struct SlimeParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagSlime,
}

impl NewParticle for SlimeParticle {
    const ELEMENT: ParticleElement = ParticleElement::Slime;

    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let red = rng.gen_range(30..=60) as u8;
        let green = rng.gen_range(180..=240) as u8;
        let blue = rng.gen_range(30..=60) as u8;

>>>>>>> 514c8f097b63156f8f8a96daeff44dc29f5f0c12
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb_u8(red, green, blue),
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
<<<<<<< HEAD
                data: ParticleElement::AcidicDirt,
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
            },
            tag: ParticleTagAcidicDirt,
=======
                data: ParticleElement::Slime,
                hitbox: Hitbox::new(
                    PARTICLE_SIZE, 
                    PARTICLE_SIZE, 
                    Vec2::new(LEVEL_H + 10., LEVEL_H + 10.)
                ),
            },
            tag: ParticleTagSlime,
>>>>>>> 514c8f097b63156f8f8a96daeff44dc29f5f0c12
        }
    }
}

<<<<<<< HEAD
// ToxicGas ------------------------------------------------------------------------
#[derive(Component)]
pub struct ParticleTagToxicGas;
#[derive(Bundle)]
pub struct ToxicGasParticle {
    sprite: SpriteBundle,
    particle: Particle,
    tag: ParticleTagToxicGas,
}
impl NewParticle for ToxicGasParticle {
    const ELEMENT: ParticleElement = ParticleElement::ToxicGas;
    fn new(x: i32, y: i32, vel: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let (r,g,b) = (rng.gen_range(165..=180) as u8, rng.gen_range(249..=255) as u8, rng.gen_range(165..=180) as u8);
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
                data: ParticleElement::ToxicGas,
                // correct hitbox:
                // hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.))
                // incorrect hitbox: (so that player can walk through)
                hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE,Vec2::new(LEVEL_H+10., LEVEL_H+10.))
            },
            tag: ParticleTagToxicGas,
        }
    }
}

          
          
          
=======
>>>>>>> 514c8f097b63156f8f8a96daeff44dc29f5f0c12

