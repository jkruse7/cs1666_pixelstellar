use bevy::prelude::*;
use rand::Rng;
use super::{components::*, resources::*};

// for both physics and procedural
// if you want to place a particle on the screen either on startup or from a key press or whatever,
// follow these examples:

// example for a Startup system
fn draw_floor(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
) {
    // loop from left side of the screen to right side of the screen
    for x in MIN_X..=MAX_X {
        // loop from the bottom of the screen to 1/4 of the screen
        for y in MIN_Y..=MIN_Y / 2 {
            // place data in map
            map.insert(x, y, ParticleData::BedRock);
            // place particle on screen
            commands.spawn(BedRockParticle::new(x, y));
        }
    }
}

// another example for an Update system
fn draw_rain(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(MIN_X..=MAX_X);
    let y = rng.gen_range(MIN_Y / 2..=MAX_Y);
    if map.get(x, y) == ParticleData::Air {
        map.insert(x, y, ParticleData::Water);
        commands.spawn(WaterParticle::new(x, y));
    }
}

// once your particle placement system is complete,
// you can add it to this plugin and you don't have to touch main.rs
pub struct SpawnParticles;

impl Plugin for SpawnParticles {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParticleMap::new());
        app.add_systems(Startup, draw_floor);
        app.add_systems(Update, draw_rain);
    }
}

// use this as an example for cellular automation
fn update_water(
    mut map: ResMut<ParticleMap>,
    mut particles: Query<(&mut Transform, &mut ParticlePosition, &ParticleData), With<ParticleElementWater>>,
) {
    // loop through all of the queried water particles
    for (mut transform, mut position, data) in &mut particles {
        // if there is nothing below, fall down 1
        if map.get(position.x, position.y - 1) == ParticleData::Air {
            // move the particle data on the map down 1
            map.move_data((position.x, position.y), (position.x, position.y - 1), data);
            // update the particle's position
            position.y -= 1;
            // move the particle on the screen
            transform.translation.x = position.x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
            transform.translation.y = position.y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
            // check particle to bottom left
        } else if map.get(position.x - 1, position.y - 1) == ParticleData::Air {
            map.move_data((position.x, position.y), (position.x - 1, position.y - 1), data);
            position.x -= 1;
            position.y -= 1;
            transform.translation.x = position.x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
            transform.translation.y = position.y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
            // check particle to bottom right
        } else if map.get(position.x + 1, position.y - 1) == ParticleData::Air {
            map.move_data((position.x, position.y), (position.x + 1, position.y - 1), data);
            position.x += 1;
            position.y -= 1;
            transform.translation.x = position.x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
            transform.translation.y = position.y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
        }
    }
}

// once your automata system is complete,
// you can add it to this plugin and you don't have to touch main.rs
pub struct UpdateParticles;

impl Plugin for UpdateParticles {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_water);
    }
}

pub fn convert_to_grid_position(x: f32, y: f32) -> (i32, i32) {
    let x = (x / PARTICLE_SIZE).round() as i32;
    let y = (y / PARTICLE_SIZE).round() as i32;
    (x, y)
}