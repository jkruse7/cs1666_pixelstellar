use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::{particle::components::*, LEVEL_W, LEVEL_H};

pub const PARTICLE_SIZE: f32 = 4.;
pub const MIN_X: i32 = ((-LEVEL_W / 2.) / PARTICLE_SIZE) as i32;
pub const MAX_X: i32 = ((LEVEL_W / 2.) / PARTICLE_SIZE) as i32;
pub const MIN_Y: i32 = ((-LEVEL_H / 2.) / PARTICLE_SIZE) as i32;
pub const MAX_Y: i32 = ((LEVEL_H / 2.) / PARTICLE_SIZE) as i32;

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

    pub fn remove(&mut self, x: i32, y: i32) {
        self.particle_map.remove(&(x, y));
    }

    pub fn insert(&mut self, x: i32, y: i32, particle_type: ParticleData) {
        self.particle_map.insert((x, y), particle_type);
    }

    pub fn move_data(&mut self, old: (i32, i32), new: (i32, i32), data: &ParticleData) {
        self.particle_map.remove(&old);
        self.particle_map.insert(new, *data);
    }

    // Converts bevy coordinates to particle coordinates
    pub fn convert_to_grid_position(&self, x: f32, y: f32) -> (i32, i32) {
        let x = (x / PARTICLE_SIZE).round() as i32;
        let y = (y / PARTICLE_SIZE).round() as i32;
        (x, y)
    }
}

