use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use crate::common::state::GamePhase;
use crate::entities::{
    particle::{resources::*, components::*},
    player::components::Player,
};
use crate::common::perlin_noise::*;

// Define structs --------------------------------------------------------------------------------
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum ParticleType {
    Stone,
    Dirt,
    BedRock,
    // Add more particle types as needed. The order doesn't matter.

}

#[derive(Resource)]
pub struct NoiseSettings {
    /// The starting frequency of the noise function.
    /// Determines the initial scale of the noise pattern.
    /// Lower values result in broader, smoother features,
    /// while higher values create finer, more frequent variations.
    pub start_frequency: f32,

    /// The number of octaves to use in the noise generation.
    /// Each octave adds a layer of detail by superimposing noise at higher frequencies
    /// and lower amplitudes. Increasing the number of octaves adds complexity
    /// to the noise pattern.
    pub octaves: usize,

    /// Controls the amplitude scaling (persistence) for each successive octave.
    /// It determines how quickly the amplitude decreases for higher octaves.
    /// A lower persistence value results in a smoother noise (less influence from higher octaves),
    /// while a higher value retains more detail from the higher-frequency octaves.
    pub persistence: f32,

    /// Adjusts the frequency for each successive octave.
    /// This multiplier increases the frequency at each octave level,
    /// allowing for finer details at higher octaves.
    /// It controls how rapidly the frequency increases with each octave.
    pub frequency_modifier: f32,

    /// The minimum value of the mapped noise range.
    /// After generating the raw noise values, they are mapped to a range between
    /// `noise_range_min` and `noise_range_max`. This parameter sets the lower bound
    /// of that range.
    pub noise_range_min: f32,

    /// The maximum value of the mapped noise range.
    /// Sets the upper bound of the mapped noise values after scaling.
    /// This allows you to control the maximum effect the noise can have on the terrain,
    /// such as limiting the maximum height variation in a terrain layer.
    pub noise_range_max: f32,
}

#[derive(Resource)]
pub struct CaveSettings {
    /// Controls the starting frequency of the 2D noise for cave generation.
    /// Lower values will result in larger, more sparse caves; higher values will make caves more frequent and smaller.
    pub start_frequency: f32,

    /// The number of octaves used for the 2D noise.
    /// More octaves add complexity and detail to the caves, but might also reduce large cave systems.
    pub octaves: usize,

    /// The persistence value of the cave noise function.
    /// Determines how much each octave contributes to the final cave generation.
    pub persistence: f32,

    /// Frequency modifier that adjusts how rapidly the frequency increases per octave.
    pub frequency_modifier: f32,

    /// The minimum Y-coordinate for cave placement.
    pub min_y: i32,

    /// The maximum Y-coordinate for cave placement.
    pub max_y: i32,

    /// Minimum threshold value for cave generation.
    /// Only cave cells that have a noise value above this will be placed.
    pub noise_threshold_min: f32,

    /// Maximum threshold value for cave generation.
    /// Caves will not be placed if the noise value exceeds this threshold.
    pub noise_threshold_max: f32,
}

pub struct LayerSettings {
    pub particle_type: ParticleType,
    pub noise_settings: NoiseSettings,
}

#[derive(Resource)]
pub struct WorldGenSettings {
    pub layers: Vec<LayerSettings>,
    pub caves: Option<CaveSettings>,  // Optional cave settings
    pub perm: [usize; 512],
}

// Parameter adjustment --------------------------------------------------------------------------------
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
            layers: vec![
                LayerSettings {
                    particle_type: ParticleType::Stone,
                    noise_settings: NoiseSettings {
                        start_frequency: 0.015,
                        octaves: 2,
                        noise_range_min: 30.,
                        noise_range_max: 40.,
                        ..Default::default()
                    },
                },
                LayerSettings {
                    particle_type: ParticleType::Dirt,
                    noise_settings: NoiseSettings {
                        start_frequency: 0.012,
                        octaves: 1,
                        noise_range_min: 0.,
                        noise_range_max: 20.,
                        ..Default::default()
                    },
                },
                LayerSettings {
                    particle_type: ParticleType::BedRock,
                    noise_settings: NoiseSettings {
                        start_frequency: 0.03,
                        octaves: 5,
                        noise_range_min: 0.,
                        noise_range_max: 180.,
                        ..Default::default()
                    },
                },
                // Add more layers here if needed, in order from top to bottom

            ],
            caves: Some(CaveSettings::default()),  // "caves: None" to disable caves
            perm: generate_permutation_array(),
        }
    }
}

impl Default for CaveSettings {
    fn default() -> Self {
        Self {
            start_frequency: 0.03,
            octaves: 3,
            persistence: 0.5,
            frequency_modifier: 1.2,
            min_y: -50,
            max_y: 90,
            noise_threshold_min: 0.45,
            noise_threshold_max: 0.55,
        }
    }
}

// Map placement type functions  --------------------------------------------------------------------------------
fn select_particle_layers(y: f32, layer_noises: &[(ParticleType, f32)]) -> ParticleType {
    for (particle_type, noise_height) in layer_noises.iter() {
        if y >= *noise_height {
            return *particle_type;
        }
    }
    layer_noises.last().unwrap().0
}

fn generate_world(
    map: &mut ResMut<ParticleMap>,
    commands: &mut Commands,
    config: &Res<WorldGenSettings>,
    chunk: (i32, i32),
) {
    //let perm = generate_permutation_array();

    let x_start = chunk.0 * 64;
    let x_end = x_start + 63;

    let y_start = chunk.1 * 64;
    let mut y_end = y_start + 63;

    for x in x_start..=x_end {
        let mut layer_noises = Vec::new();

        for layer in &config.layers {
            let noise = get_1d_octaves(
                x as f32,
                layer.noise_settings.start_frequency,
                layer.noise_settings.octaves,
                layer.noise_settings.persistence,
                layer.noise_settings.frequency_modifier,
                layer.noise_settings.noise_range_min,
                layer.noise_settings.noise_range_max,
                &config.perm,
            )
            .floor();

            layer_noises.push((layer.particle_type, noise));
        }

        let max_noise = layer_noises
            .iter()
            .map(|(_, noise)| *noise)
            .fold(f32::MIN, f32::max);

        let y_terrain = -90 + max_noise as i32;
        if y_start > y_terrain {
            continue;
        }
        if y_end > y_terrain {
            y_end = y_terrain;
        }

        if let Some(cave_settings) = &config.caves {
            for y in y_start..=y_end {
                let noise_cave = get_2d_octaves(
                    x as f32,
                    y as f32,
                    cave_settings.start_frequency,
                    cave_settings.octaves,
                    cave_settings.persistence,
                    cave_settings.frequency_modifier,
                    0.,
                    1.,
                    &config.perm,
                );

                if (y as f32) >= cave_settings.min_y as f32 && (y as f32) <= cave_settings.max_y as f32 &&
                    (noise_cave >= cave_settings.noise_threshold_min && noise_cave <= cave_settings.noise_threshold_max) {
                    continue;
                }

                let current_particle = select_particle_layers((y + 90) as f32, &layer_noises);

                match current_particle {
                    ParticleType::BedRock => {
                        map.insert_at::<GrassParticle>(commands, (x, y), ListType::All);
                    }
                    ParticleType::Dirt => {
                        map.insert_at::<DirtParticle>(commands, (x, y), ListType::All);
                    }
                    ParticleType::Stone => {
                        map.insert_at::<StoneParticle>(commands, (x, y), ListType::All);
                    }
                    // Handle other particle types if necessary

                }
            }
        }
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

pub struct Planet4Plugin;
impl Plugin for Planet4Plugin {
    fn build(&self, app: &mut App) {
        // Startup placements
        app.add_systems(OnEnter(GamePhase::Planet4), crate::common::ui::background::initialize_background);
        app.insert_resource(WorldGenSettings::default());
        //app.add_systems(OnEnter(GamePhase::Planet4), generate_world);
        //app.add_systems(OnEnter(GamePhase::Planet4), update_grass.after(generate_world));
        app.insert_resource(ChunkList::new());
        app.add_systems(Update, handle_chunks.run_if(in_state(GamePhase::Planet4)));
    }
}
