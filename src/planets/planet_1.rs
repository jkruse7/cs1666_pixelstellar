use bevy::prelude::*;
use rand::Rng;
use crate::common::state::GamePhase;
use crate::entities::particle::{resources::*, components::*};
use crate::common::perlin_noise::*;
use crate::entities::player::components::Player;
use crate::common::gravity::GravityResource;

// Define structs --------------------------------------------------------------------------------
// WorldGenSettings defines configurations for different terrain layers in world generation.
// Each layer (height, dirt, stone...) uses its own NoiseSettings to control features like
// frequency, octaves, persistence, and range.

// NoiseSettings defines the noise parameters for a given layer, including the frequency,
// number of octaves (for more detail), persistence (controls amplitude scaling), and
// frequency_modifier (to adjust frequency per octave). It also includes a min/max range
// to map generated noise values into a useful range for the layer's purpose.
#[derive(Resource)]
pub struct WorldGenSettings {
    pub height_noise: NoiseSettings,  // Controls general terrain height
    pub dirt_noise: NoiseSettings,    // Controls dirt layer height
    pub stone_noise: NoiseSettings,   // Controls stone layer height
    pub perm: [usize; 512],
}

#[derive(Resource)]
pub struct NoiseSettings {
    pub start_frequency: f32,       // just treat it as the noise frequency
    pub octaves: usize,             // for more detail
    pub persistence: f32,           // controls amplitude scaling
    pub frequency_modifier: f32,    // adjusts frequency per octave
    pub noise_range_min: f32,       // maps generated noise values into a range for the layer's purpose
    pub noise_range_max: f32,
}



// Parameter adjustment --------------------------------------------------------------------------------
// Default implementations for NoiseSettings and WorldGenSettings.
// These provide starting values for noise parameters that can be adjusted as needed.
// WorldGenSettings defines unique values for each layer to control its appearance.
impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            start_frequency: 0.05,
            octaves: 3,
            persistence: 0.5,
            frequency_modifier: 1.2,
            noise_range_min: 0.,
            noise_range_max: 180.,
        }
    }
}

impl Default for WorldGenSettings {
    fn default() -> Self {
        Self {
            height_noise: NoiseSettings {
                start_frequency: 0.03,
                octaves: 5,
                noise_range_min: 0.,
                noise_range_max: 180.,
                ..Default::default()
            },
            dirt_noise: NoiseSettings {
                start_frequency: 0.012,
                octaves: 1,
                noise_range_min: 0.,
                noise_range_max: 20.,
                ..Default::default()
            },
            stone_noise: NoiseSettings {
                start_frequency: 0.015,
                octaves: 2,
                noise_range_min: 30.,
                noise_range_max: 40.,
                ..Default::default()
            },
            perm: generate_permutation_array(),
        }
    }
}



// Map placement type functions  --------------------------------------------------------------------------------
fn generate_world(
    map: &mut ResMut<ParticleMap>,
    commands: &mut Commands,
    config: &Res<WorldGenSettings>,  // Use WorldGenSettings resource
    chunk: (i32, i32),
) {
    //crate::common::gravity::initialize(commands); //init gravity
    //commands.insert_resource(GravityResource::new(3600., 750.));

    let perm = generate_permutation_array();

    let x_start = chunk.0 * 64;
    let x_end = x_start + 64;

    let y_start = chunk.1 * 64;
    let mut y_end = y_start + 64;

    for x in x_start..=x_end {
        let noise = get_1d_octaves(
            x as f32,
            config.height_noise.start_frequency,
            config.height_noise.octaves,
            config.height_noise.persistence,
            config.height_noise.frequency_modifier,
            config.height_noise.noise_range_min,
            config.height_noise.noise_range_max,
            &config.perm,
        )
        .floor();

        let y_terrain = -90 + noise as i32;
        if y_start > y_terrain {
            continue;
        }
        if y_end > y_terrain {
            y_end = y_terrain;
        }

        let noise_dirt = get_1d_octaves(
            x as f32,
            config.dirt_noise.start_frequency,
            config.dirt_noise.octaves,
            config.dirt_noise.persistence,
            config.dirt_noise.frequency_modifier,
            config.dirt_noise.noise_range_min,
            config.dirt_noise.noise_range_max,
            &config.perm,
        )
        .floor();

        let noise_stone = get_1d_octaves(
            x as f32,
            config.stone_noise.start_frequency,
            config.stone_noise.octaves,
            config.stone_noise.persistence,
            config.stone_noise.frequency_modifier,
            config.stone_noise.noise_range_min,
            config.stone_noise.noise_range_max,
            &config.perm,
        )
        .floor();

        for y in y_start..=y_end {
            let noise_threshold_min = 0.45;
            let noise_threshold_max = 0.55;
            let noise_cave = get_2d_octaves(x as f32, y as f32, 0.03, 3, 0.5, 1.2, 0., 1., &config.perm);
            if (y as f32) >= -50. && (y as f32) <= 90. &&
                noise_cave >= noise_threshold_min && noise_cave >= noise_threshold_max {
                    continue;
                }

            let current_particle = select_particle((y + 90) as f32, noise, noise_dirt, noise_stone);
            if current_particle == ParticleElement::BedRock {
                // place data in map
                map.insert_at::<BedRockParticle>(commands, (x, y), ListType::All);
            } else if current_particle == ParticleElement::Dirt {
                map.insert_at::<StoneParticle>(commands, (x, y), ListType::All);
            } else if current_particle == ParticleElement::Stone {
                map.insert_at::<DirtParticle>(commands, (x, y), ListType::All);
            }
        }
    }
}

fn handle_chunks(
    config: Res<WorldGenSettings>,
    mut chunks: ResMut<ChunkList>,
    mut particles: ResMut<ParticleMap>,
    mut commands: Commands,
    player_transform: Query<&Transform, With<Player>>,
) {
    let pt = player_transform.single().translation;
    let position = ((pt.x / PARTICLE_SIZE).floor() as i32, (pt.y / PARTICLE_SIZE).floor() as i32);

    let new_chunks = chunks.load(position);
    for chunk in new_chunks {
        generate_world(&mut particles, &mut commands, &config, chunk);
    }

    let old_chunks = chunks.unload(position);
    for chunk in old_chunks {
        particles.despawn_chunk(&mut commands, chunk);
    }
}

// fn generate_world(
//     mut map: ResMut<ParticleMap>,
//     mut commands: Commands,
// ) {
//     let perm1 = generate_permutation_array();
//     let perm2 = generate_permutation_array();
//     let perm3 = generate_permutation_array();
//     // loop from left side of the screen to right side of the screen
//     for x in MIN_X..=MAX_X {
//         let mut noise = get_1d_octaves(x as f32, 0.05, 3, 0.5, 1.2, 0., 180., &perm1);
//         noise = noise.floor();

//         let mut noise_dirt = get_1d_octaves(x as f32, 0.012, 1, 0.5, 1.2, 0., 20., &perm2);
//         noise_dirt = noise_dirt.floor();

//         let mut noise_stone = get_1d_octaves(x as f32, 0.015, 2, 0.5, 1.2, 30., 40., &perm2);
//         noise_stone = noise_stone.floor();

        
//         for y in MIN_Y..=(-90 + noise as i32) {
//             let noise_threshold_min = 0.45;
//             let noise_threshold_max = 0.55;
//             let noise_cave = get_2d_octaves(x as f32, y as f32, 0.03, 3, 0.5, 1.2, 0., 1., &perm3);
//             if (y as f32) >= -50. && (y as f32) <= 90. &&
//                 noise_cave >= noise_threshold_min && noise_cave >= noise_threshold_max {
//                     continue;
//                 }

//             let current_particle: ParticleElement = select_particle((y + 90) as f32, noise, noise_dirt, noise_stone);
//             if current_particle == ParticleElement::BedRock {
//                 // place data in map
//                 map.insert_at::<BedRockParticle>(&mut commands, (x, y), ListType::All);
//             } else if current_particle == ParticleElement::Dirt {
//                 map.insert_at::<StoneParticle>(&mut commands, (x, y), ListType::All);
//             } else if current_particle == ParticleElement::Stone {
//                 map.insert_at::<DirtParticle>(&mut commands, (x, y), ListType::All);
//             }
//         }
//     }
// }

fn select_particle(y: f32, noise: f32, dirt_height: f32, stone_height: f32) -> ParticleElement {
    if y >= stone_height {
        ParticleElement::Stone
    } else if y >= dirt_height{
        ParticleElement::Dirt
    } else {
        ParticleElement::BedRock
    }
}

fn update_grass(
    mut map: ResMut<ParticleMap>,
    time: Res<Time>, 
    mut commands: Commands,
    mut particles: Query<&mut ParticlePosVel, With<ParticleTagDirt>>,
) {
    for mut position in &mut particles {
        let (x, y) = (position.grid_x, position.grid_y);
        if map.get_element_at((x, y+1)) == ParticleElement::Air{
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<GrassParticle>(&mut commands, (x, y), ListType::OnlyAir);
        }
        if ((map.get_element_at((x + 1, y)) == ParticleElement::Air &&
             map.get_element_at((x+1, y-1)) == ParticleElement::Air)||
            (map.get_element_at((x + 1, y)) == ParticleElement::Air &&
             map.get_element_at((x+1, y-1)) == ParticleElement::Air))&&
           (map.get_element_at((x, y-1)) == ParticleElement::Dirt ||
            map.get_element_at((x, y-1)) == ParticleElement::Grass ){
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<GrassParticle>(&mut commands, (x, y), ListType::OnlyAir);
        }
    }
}

fn set_crosshair_cursor( mut q_window: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
) {
    let mut window = q_window.single_mut();
    window.cursor.icon = CursorIcon::Cell;
}

pub struct Planet1Plugin;
impl Plugin for Planet1Plugin {
    fn build(&self, app: &mut App) {
        // Startup placements
        app.add_systems(OnEnter(GamePhase::Planet1), crate::common::ui::background::initialize_background);
        app.insert_resource(WorldGenSettings::default());
        app.insert_resource(ChunkList::new());
        app.insert_resource(GravityResource::new(3600., 750.));
        //app.add_systems(OnEnter(GamePhase::Planet1), generate_world);
        app.add_systems(Update, handle_chunks.run_if(in_state(GamePhase::Planet1)));


        app.add_systems(OnEnter(GamePhase::Planet1), set_crosshair_cursor);
        //app.add_systems(OnEnter(GamePhase::Planet1), generate_world);
        //app.add_systems(OnEnter(GamePhase::Planet1), update_grass.after(generate_world));
    }
} 
