use bevy::prelude::*;
use rand::Rng;
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

            let current_particle: ParticleElement = select_particle((y + 90) as f32, noise, noise_dirt, noise_stone);
            if current_particle == ParticleElement::BedRock {
                // place data in map
                map.insert_at::<BedRockParticle>(&mut commands, (x, y));
            } else if current_particle == ParticleElement::Dirt {
                map.insert_at::<DirtParticle>(&mut commands, (x, y));
            } else if current_particle == ParticleElement::Stone {
                map.insert_at::<StoneParticle>(&mut commands, (x, y));
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
        if map.get_element_at((x, y)) == ParticleElement::Air {
            map.insert_at::<WaterParticle>(&mut commands, (x, y));
        }
    }
}



fn update_water(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
    mut particles: Query<(&mut Transform, &mut ParticlePosition, &ParticleElement), With<ParticleTagWater>>,
) {
    for (mut transform, mut position, data) in &mut particles {
        let (x, y) = (position.grid_x, position.grid_y);
        if map.get_element_at((x, y-1)) == ParticleElement::Air {
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<WaterParticle>(&mut commands, (x, y-1));
        } else if map.get_element_at((x-1, y-1)) == ParticleElement::Air {
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<WaterParticle>(&mut commands, (x-1, y-1));
            // check particle to bottom right
        } else if map.get_element_at((x+1, y-1)) == ParticleElement::Air {
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<WaterParticle>(&mut commands, (x+1, y-1));
        } else if map.get_element_at((x-1, y)) == ParticleElement::Air {
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<WaterParticle>(&mut commands, (x-1, y));
        }
        else if map.get_element_at((x+1, y)) == ParticleElement::Air {
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<WaterParticle>(&mut commands, (x+1, y));
        }
    }
}
fn update_gas(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
    mut particles: Query<(&mut Transform, &mut ParticlePosition, &ParticleElement), With<ParticleTagGas>>,
) {
    for (mut transform, mut position, data) in &mut particles {
        let (x, y) = (position.grid_x, position.grid_y);
        if map.get_element_at((x, y-1)) == ParticleElement::Air {
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<WaterParticle>(&mut commands, (x, y-1));
        } else if map.get_element_at((x-1, y-1)) == ParticleElement::Air {
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<WaterParticle>(&mut commands, (x-1, y-1));
            // check particle to bottom right
        } else if map.get_element_at((x+1, y-1)) == ParticleElement::Air {
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<WaterParticle>(&mut commands, (x+1, y-1));
        } else if map.get_element_at((x-1, y)) == ParticleElement::Air {
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<WaterParticle>(&mut commands, (x-1, y));
        }
        else if map.get_element_at((x+1, y)) == ParticleElement::Air {
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<WaterParticle>(&mut commands, (x+1, y));
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

fn select_particle(y: f32, noise: f32, dirt_height: f32, stone_height: f32) -> ParticleElement {
    if y >= stone_height {
        ParticleElement::Stone
    } else if y >= dirt_height{
        ParticleElement::Dirt
    } else {
        ParticleElement::BedRock
    }
}

pub fn build(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    let (l, r) = (buttons.pressed(MouseButton::Left), buttons.pressed(MouseButton::Right));

    if l || r{
        if let Some(world_position) = 
            window.cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
        {
            let size = 1.;
            let mut y: f32 = -size * PARTICLE_SIZE;
            while y < size * PARTICLE_SIZE + 0.1{
                let mut x: f32 = -size * PARTICLE_SIZE;
                while x < size * PARTICLE_SIZE + 0.1{
                    let position = (((world_position.x+x) / PARTICLE_SIZE) as i32, ((world_position.y+y) / PARTICLE_SIZE) as i32);
                    info!("x, y: {}/{}", x, y);
                    if l {
                        map.delete_at(&mut commands, (((world_position.x+x) / PARTICLE_SIZE) as i32, ((world_position.y+y) / PARTICLE_SIZE) as i32));
                    }
                    if r && map.get_element_at((position.0, position.1)) == ParticleElement::Air{
                        map.insert_at::<DirtParticle>(&mut commands, (position.0, position.1));
                    }
                    x += PARTICLE_SIZE;
                }
                y += PARTICLE_SIZE;
            }
        }
    }
}


// once your automata system is complete,
// you can add it to this plugin and you don't have to touch main.rs
pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        // Startup placements
        app.insert_resource(ParticleMap::new());
        app.add_systems(Startup, draw_solid);

        // Updates i.e. all automata goes here
        app.add_systems(Update, draw_rain);
        app.add_systems(Update, update_water);
        
        app.add_systems(Update, build);
    }
} 
