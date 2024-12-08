use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use crate::common::state::GamePhase;
use crate::entities::particle::{resources::*, components::*};
use crate::common::perlin_noise::*;

use crate::LEVEL_W;

const GEYSER_INTENSITY: i32 = 4;

// Define structs --------------------------------------------------------------------------------
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum ParticleType {
    AcidicDirt,
    Stone,
    AcidicWater,
    ToxicGas,
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
}

// Parameter adjustment --------------------------------------------------------------------------------
impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            start_frequency: 0.005,
            octaves: 2,
            persistence: 0.25,
            frequency_modifier: 1.05,
            noise_range_min: 0.,
            noise_range_max: 140.,
        }
    }
}

impl Default for WorldGenSettings {
    fn default() -> Self {
        Self {
            layers: vec![
                LayerSettings {
                    particle_type: ParticleType::AcidicDirt,
                    noise_settings: NoiseSettings {
                        start_frequency: 0.015,
                        octaves: 2,
                        noise_range_min: 30.,
                        noise_range_max: 40.,
                        ..Default::default()
                    },
                },
                LayerSettings {
                    particle_type: ParticleType::Stone,
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
        }
    }
}

impl Default for CaveSettings {
    fn default() -> Self {
        Self {
            start_frequency: 0.009,
            octaves: 5,
            persistence: 0.5,
            frequency_modifier: 1.2,
            min_y: -170,
            max_y: 10,
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
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
    config: Res<WorldGenSettings>,
) {
    let perm = generate_permutation_array();

    for x in MIN_X..=MAX_X {
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
                &perm,
            )
            .floor();

            layer_noises.push((layer.particle_type, noise));
        }

        let max_noise = layer_noises
            .iter()
            .map(|(_, noise)| *noise)
            .fold(f32::MIN, f32::max);

        if let Some(cave_settings) = &config.caves {
            for y in MIN_Y..=(-90 + max_noise as i32) {
                let noise_cave = get_2d_octaves(
                    x as f32,
                    y as f32,
                    cave_settings.start_frequency,
                    cave_settings.octaves,
                    cave_settings.persistence,
                    cave_settings.frequency_modifier,
                    0.,
                    1.,
                    &perm,
                );

                if (y as f32) >= cave_settings.min_y as f32 && (y as f32) <= cave_settings.max_y as f32 &&
                    (noise_cave >= cave_settings.noise_threshold_min && noise_cave <= cave_settings.noise_threshold_max) {
                    continue;
                }

                let current_particle = select_particle_layers((y + 90) as f32, &layer_noises);

                match current_particle {
                    ParticleType::BedRock => {
                        map.insert_at::<BedRockParticle>(&mut commands, (x, y), ListType::All);
                    }
                    ParticleType::AcidicDirt => {
                        map.insert_at::<AcidicDirtParticle>(&mut commands, (x, y), ListType::All);
                    }
                    ParticleType::Stone => {
                        map.insert_at::<StoneParticle>(&mut commands, (x, y), ListType::All);
                    }
                    ParticleType::AcidicWater => { }
                    ParticleType::ToxicGas => { }
                    // Handle other particle types if necessary

                }
            }
        }
    }
}




fn draw_toxic_gas(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
) {
    for _ in 0..8{
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-(LEVEL_W/2.)..=(LEVEL_W/2.)) as i32;
        let y = rng.gen_range(-175..-80);
        if map.get_element_at((x, y)) == ParticleElement::Air {
            map.insert_at::<ToxicGasParticle>(&mut commands, (x, y), ListType::OnlyAir);
        }
    }
}



pub struct Planet6Plugin;
impl Plugin for Planet6Plugin {
    fn build(&self, app: &mut App) {
        // Startup placements
        app.add_systems(OnEnter(GamePhase::Planet6), crate::common::ui::background::initialize_background);
        app.insert_resource(WorldGenSettings::default());
        app.add_systems(OnEnter(GamePhase::Planet6), generate_world);
        app.add_systems(Update, draw_toxic_gas.run_if(in_state(GamePhase::Planet6)));
    }
}
