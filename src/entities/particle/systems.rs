use bevy::prelude::*;
use rand::Rng;
use crate::GameState;
use super::{components::*, resources::*};
use crate::common::
        perlin_noise::{
            generate_permutation_array, get_1d_octaves, get_2d_octaves,
        };

// for both physics and procedural
// if you want to place a particle on the screen either on startup or from a key press or whatever,
// follow these examples:

// example for a Startup system
fn draw_solid(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
) {
    let perm1 = generate_permutation_array();
    let perm2 = generate_permutation_array();
    let perm3 = generate_permutation_array();
    // loop from left side of the screen to right side of the screen
    for x in MIN_X..=MAX_X {
        let mut noise = get_1d_octaves(x as f32, 0.05, 3, 0.5, 1.2, 0., 180., &perm1);
        noise = noise.floor();

        let mut noise_dirt = get_1d_octaves(x as f32, 0.003, 1, 0.5, 1.2, 0., 20., &perm2);
        noise_dirt = noise_dirt.floor();

        let mut noise_stone = get_1d_octaves(x as f32, 0.015, 2, 0.5, 1.2, 30., 40., &perm2);
        noise_stone = noise_stone.floor();

        for y in MIN_Y..=(-90 + noise as i32) {
            let noise_threshold_min = 0.45;
            let noise_threshold_max = 0.55;
            let noise_cave = get_2d_octaves(x as f32, y as f32, 0.03, 3, 0.5, 1.2, 0., 1., &perm3);
            if (y as f32) >= -50. && (y as f32) <= 90. &&
                noise_cave >= noise_threshold_min && noise_cave >= noise_threshold_max {
                    continue;
                }

            let current_particle: ParticleData = select_particle((y + 90) as f32, noise, noise_dirt, noise_stone);
            if current_particle == ParticleData::BedRock {
                // place data in map
                map.insert(x, y, ParticleData::BedRock);
                // place particle on screen
                commands.spawn(BedRockParticle::new(x, y));
            } else if current_particle == ParticleData::Dirt {
                map.insert(x, y, ParticleData::Dirt);
                commands.spawn(DirtParticle::new(x, y));
            } else if current_particle == ParticleData::Stone {
                map.insert(x, y, ParticleData::Stone);
                commands.spawn(StoneParticle::new(x, y));
            }
        }
    }
}

// another example for an Update system
fn draw_rain(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
) {
    for _ in 0..5{
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-50..=50);
        let y = rng.gen_range(100..200);
        if map.get(x, y) == ParticleData::Air {
            map.insert(x, y, ParticleData::Water);
            commands.spawn(WaterParticle::new(x, y));
        }
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
        if map.get(position.grid_x, position.grid_y - 1) == ParticleData::Air {
            // move the particle data on the map down 1
            map.move_data((position.grid_x, position.grid_y), (position.grid_x, position.grid_y - 1), data);
            // update the particle's position
            position.grid_y -= 1;
            // move the particle on the screen
            transform.translation.x = position.grid_x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
            transform.translation.y = position.grid_y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
            // check particle to bottom left
        } else if map.get(position.grid_x - 1, position.grid_y - 1) == ParticleData::Air {
            map.move_data((position.grid_x, position.grid_y), (position.grid_x - 1, position.grid_y - 1), data);
            position.grid_x -= 1;
            position.grid_y -= 1;
            transform.translation.x = position.grid_x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
            transform.translation.y = position.grid_y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
            // check particle to bottom right
        } else if map.get(position.grid_x + 1, position.grid_y - 1) == ParticleData::Air {
            map.move_data((position.grid_x, position.grid_y), (position.grid_x + 1, position.grid_y - 1), data);
            position.grid_x += 1;
            position.grid_y -= 1;
            transform.translation.x = position.grid_x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
            transform.translation.y = position.grid_y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
        }
        // check to the left as far as can go
        else if map.get(position.grid_x - 1, position.grid_y) == ParticleData::Air {
            map.move_data((position.grid_x, position.grid_y), (position.grid_x - 1, position.grid_y), data);
            position.grid_x -= 1;
            transform.translation.x = position.grid_x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
            transform.translation.y = position.grid_y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
        }
        else if map.get(position.grid_x + 1, position.grid_y) == ParticleData::Air {
            map.move_data((position.grid_x, position.grid_y), (position.grid_x + 1, position.grid_y), data);
            position.grid_x += 1;
            transform.translation.x = position.grid_x as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
            transform.translation.y = position.grid_y as f32 * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
        }
    }
}



pub fn convert_to_grid_position(x: f32, y: f32) -> (i32, i32) {
    let x = (x / PARTICLE_SIZE).round() as i32;
    let y = (y / PARTICLE_SIZE).round() as i32;
    (x, y)
}

/*pub fn add_water(x: i32, y: i32, map: &mut ResMut<ParticleMap>, commands: &mut Commands) {
    if map.get(x, y) == ParticleData::Air {
        map.insert(x, y, ParticleData::Water);
        commands.spawn(WaterParticle::new(x, y));
    }
}*/

fn select_particle(y: f32, noise: f32, dirt_height: f32, stone_height: f32) -> ParticleData {
    if y >= stone_height {
        ParticleData::Stone
    } else if y >= dirt_height{
        ParticleData::Dirt
    } else {
        ParticleData::BedRock
    }
}



// once your automata system is complete,
// you can add it to this plugin and you don't have to touch main.rs
pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        // Startup placements
        app.insert_resource(ParticleMap::new());
        app.add_systems(OnEnter(GameState::Level1), draw_solid);

        // Updates i.e. all automata goes here
        app.add_systems(Update, draw_rain.run_if(in_state(GameState::Level1)));
        app.add_systems(Update, update_water.run_if(in_state(GameState::Level1)));
        
        
    }
} 
