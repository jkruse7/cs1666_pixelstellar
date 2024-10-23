use bevy::prelude::*;
use rand::Rng;
use super::{components::*, resources::*};
use crate::world::
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
        let mut noise = get_1d_octaves((x as f32), 0.05, 3, 0.5, 1.2, 0., 180., &perm1);
        noise = noise.floor();

        let mut noise_dirt = get_1d_octaves((x as f32), 0.003, 1, 0.5, 1.2, 0., 20., &perm2);
        noise_dirt = noise_dirt.floor();

        let mut noise_stone = get_1d_octaves((x as f32), 0.015, 2, 0.5, 1.2, 30., 40., &perm2);
        noise_stone = noise_stone.floor();

        for y in MIN_Y..=(-90 + noise as i32) {
            let noise_threshold_min = 0.45;
            let noise_threshold_max = 0.55;
            let noise_cave = get_2d_octaves((x as f32), (y as f32), 0.03, 3, 0.5, 1.2, 0., 1., &perm3);
            if (y as f32) >= -50. && (y as f32) <= 90. &&
                noise_cave >= noise_threshold_min && noise_cave >= noise_threshold_max {
                    continue;
                }

            let current_particle: ParticleElement = select_particle(((y + 90) as f32), noise, noise_dirt, noise_stone);
            if current_particle == ParticleElement::BedRock {
                // place data in map
                //map.insert(x, y, ParticleElement::BedRock);

            map.insert::<BedRockParticle>(&mut commands, x, y);
                // place particle on screen
                commands.spawn(BedRockParticle::new(x, y));
            } else if current_particle == ParticleElement::Dirt {
                //map.insert(x, y, ParticleElement::Dirt);
            map.insert::<DirtParticle>(&mut commands, x, y);
                commands.spawn(DirtParticle::new(x, y));
            } else if current_particle == ParticleElement::Dirt {

            map.insert::<DirtParticle>(&mut commands, x, y);
                //map.insert(x, y, ParticleElement::Dirt);
                commands.spawn(DirtParticle::new(x, y));
            }
        }
    }
}

// another example for an Update system
fn draw_rain(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
) {
    for _ in 0..RAIN_INTENSITY{
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(RAIN_MIN_X..=RAIN_MAX_X);
        let y = rng.gen_range(RAIN_MIN_Y..=RAIN_MAX_Y);
        if map.get(x, y).element == ParticleElement::Air {
            map.insert::<WaterParticle>(&mut commands, x, y);
        }
    }
}

// once your particle placement system is complete,
// you can add it to this plugin and you don't have to touch main.rs
pub struct SpawnParticles;

impl Plugin for SpawnParticles {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParticleMap::new());
        app.add_systems(Startup, draw_solid);
        app.add_systems(Update, draw_rain);
    }
}

// use this as an example for cellular automation
fn update_water(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
    mut particles: Query<(Entity, &mut Transform, &mut ParticlePosition), With<ParticleElementWater>>,
) {
    // loop through all of the queried water particles
    for (e_id, mut transform, mut position) in &mut particles {
        let (x, y) = (position.grid_x, position.grid_y);
        //info!("{:?}", map.get(x, y-1));
        if map.get(x, y - 1).element == ParticleElement::Air {
            map.move_data(&mut commands, (x, y), (x, y - 1));
        } else if map.get(x,y-1).element == ParticleElement::Dirt{
            map.remove(&mut commands, x, y);
            map.remove(&mut commands, x, y-1);
            map.insert::<WetDirtParticle>(&mut commands, x, y - 1);
        } else if map.get(x - 1, y - 1).element == ParticleElement::Air {
            map.move_data(&mut commands, (x, y), (x - 1, y - 1));
        } else if map.get(x + 1, y - 1).element == ParticleElement::Air {
            map.move_data(&mut commands, (x, y), (x + 1, y - 1));
        } else if map.get(x - 1, y).element == ParticleElement::Air {
            // check to the left as far as can go
            map.move_data(&mut commands, (x, y), (x - 1, y));
        } else if map.get(x + 1, y).element == ParticleElement::Air {
            map.move_data(&mut commands, (x, y), (x + 1, y));
        }
    }
}
fn update_wet_dirt (
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
    mut particles: Query<(Entity, &mut Transform, &mut ParticlePosition, &ParticleElement, ), With<ParticleElementWetDirt>>,
){

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


fn select_particle(y: f32, noise: f32, dirt_height: f32, stone_height: f32) -> ParticleElement {
    if y >= stone_height {
        ParticleElement::Dirt
    } else if y >= dirt_height{
        ParticleElement::Dirt
    } else {
        ParticleElement::BedRock
    }
}