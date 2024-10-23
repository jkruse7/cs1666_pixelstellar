use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::{particle::components::*, LEVEL_W, LEVEL_H};

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
pub const RAIN_INTENSITY: i32 = 10;

#[derive(Resource)]
pub struct ParticleMap {
    particle_map: HashMap<(i32, i32), ParticleData>,
}

impl ParticleMap {
    pub fn new() -> Self {
        Self {
            particle_map: HashMap::new(),
        }
    }

    pub fn get(&self, x: i32, y: i32) -> ParticleData {
        *self.particle_map.get(&(x, y)).unwrap_or(&ParticleData::Air)
    }

    pub fn remove(&mut self, commands: &mut Commands, entity: Entity, x: i32, y: i32) {
        commands.entity(entity).despawn();
        self.particle_map.remove(&(x, y));
    }

    pub fn insert<P: NewParticle + Bundle>(&mut self, commands: &mut Commands, x: i32, y: i32) {

        commands.spawn(P::new(x, y));
        self.particle_map.insert((x, y), P::PARTICLE_DATA);
    }

    pub fn move_data(&mut self, old: (i32, i32), new: (i32, i32), data: &ParticleData) {
        self.particle_map.remove(&old);
        self.particle_map.insert(new, *data);
    }
}

