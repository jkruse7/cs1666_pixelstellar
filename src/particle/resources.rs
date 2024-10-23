use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::{engine::hitbox::Hitbox, particle::components::*, LEVEL_W, LEVEL_H};

pub const PARTICLE_SIZE: f32 = 4.;
pub const MIN_X: i32 = ((-LEVEL_W / 2.) / PARTICLE_SIZE) as i32;
pub const MAX_X: i32 = ((LEVEL_W / 2.) / PARTICLE_SIZE) as i32;
pub const MIN_Y: i32 = ((-LEVEL_H / 2.) / PARTICLE_SIZE) as i32;
pub const MAX_Y: i32 = ((LEVEL_H / 2.) / PARTICLE_SIZE) as i32;

pub const RAIN_MIN_X: i32 =    -(LEVEL_W / 2.) as i32;
pub const RAIN_MAX_X: i32 =     (LEVEL_W / 2.) as i32;
pub const RAIN_MIN_Y: i32 =((LEVEL_H / 2. / PARTICLE_SIZE)) as i32;
pub const RAIN_MAX_Y: i32 =     (LEVEL_H / 2. / PARTICLE_SIZE) as i32;
//Ranges from 0 and up. 0 will be no rain, and the higher the value the more rain. Around 50 is like flooding
// If 0, it would be better to disable the rain update system though.
pub const RAIN_INTENSITY: i32 = 4;

#[derive(Resource)]
pub struct ParticleMap {
    particle_map: HashMap<(i32, i32), Particle>,
}

impl ParticleMap {
    pub fn new() -> Self {
        Self {
            particle_map: HashMap::new(),
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Particle {
        match self.particle_map.get(&(x, y)) {
            Some(particle) => particle.clone(),
            None => Particle::default(),
        }
    }

    pub fn remove(&mut self, commands: &mut Commands, x: i32, y: i32) {
        let entity: Option<Entity> = self.particle_map.get(&(x, y)).map(|particle| particle.entity.entity_id).flatten();
        match entity {
            Some(entity) => {commands.entity(entity).despawn(); self.particle_map.remove(&(x, y));},
            None => {},
        }
    }

    pub fn insert<P: NewParticle + Bundle>(&mut self, commands: &mut Commands, x: i32, y: i32) {
        let particle_instance = P::new(x, y);
        let entity = commands.spawn(particle_instance).id();
        let particle_type = ParticleType::from_element(P::ELEMENT);

        self.particle_map.insert((x, y), Particle {
            position: ParticlePosition::new(x, y),
            entity: ParticleEntity::new(Some(entity)),
            element: P::ELEMENT,
            hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE, Vec2::new(x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2., y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.)),
            typpe: particle_type
        });
    }

    pub fn move_data(&mut self, commands: &mut Commands, old: (i32, i32), new: (i32, i32)) {
        if let Some(particle) = self.particle_map.get(&old).cloned() {
            self.remove(commands, old.0, old.1);

            match particle.typpe {
                ParticleType::BedRock(ref p) => self.insert::<BedRockParticle>(commands, new.0, new.1),
                ParticleType::Water(ref p) => self.insert::<WaterParticle>(commands, new.0, new.1),
                ParticleType::Dirt(ref p) => self.insert::<DirtParticle>(commands, new.0, new.1),
                ParticleType::WetDirt(ref p) => self.insert::<WetDirtParticle>(commands, new.0, new.1),
                ParticleType::Stone(ref p) => self.insert::<StoneParticle>(commands, new.0, new.1),
            }
        }
    }
}

