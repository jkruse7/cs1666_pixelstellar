use bevy::prelude::*;
use rand::Rng;
use crate::common::state::GamePhase;
use crate::entities::particle::{resources::*, components::*};
use crate::common::perlin_noise::*;
use crate::LEVEL_W;



const RAIN_INTENSITY: i32 = 10;
const RAIN_VEL: Vec2 = Vec2::new(2., -20.);
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
    pub sand_noise: NoiseSettings,    // Controls sand layer height
    pub quicksand_noise: NoiseSettings,    // Controls quicksand layer height
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
            sand_noise: NoiseSettings {
                start_frequency: 0.0015,
                octaves: 2,
                noise_range_min: 0.,
                noise_range_max: 20.,
                ..Default::default()
            },
            quicksand_noise: NoiseSettings {
                start_frequency: 0.0015,
                octaves: 1,
                noise_range_min: 40.,
                noise_range_max: 30.,
                ..Default::default()
            },
        }
    }
}



// Map placement type functions  --------------------------------------------------------------------------------
fn generate_world(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
    config: Res<WorldGenSettings>,  // Use WorldGenSettings resource
) {
    crate::common::gravity::initialize(&mut commands); //init gravity

    let perm = generate_permutation_array();

    for x in MIN_X..=MAX_X {
        let noise = get_1d_octaves(
            x as f32,
            config.height_noise.start_frequency,
            config.height_noise.octaves,
            config.height_noise.persistence,
            config.height_noise.frequency_modifier,
            config.height_noise.noise_range_min,
            config.height_noise.noise_range_max,
            &perm,
        )
        .floor();

        let noise_sand = get_1d_octaves(
            x as f32,
            config.sand_noise.start_frequency,
            config.sand_noise.octaves,
            config.sand_noise.persistence,
            config.sand_noise.frequency_modifier,
            config.sand_noise.noise_range_min,
            config.sand_noise.noise_range_max,
            &perm,
        )
        .floor();

        let noise_quicksand = get_1d_octaves(
            x as f32,
            config.quicksand_noise.start_frequency,
            config.quicksand_noise.octaves,
            config.quicksand_noise.persistence,
            config.quicksand_noise.frequency_modifier,
            config.quicksand_noise.noise_range_min,
            config.quicksand_noise.noise_range_max,
            &perm,
        )
        .floor();


        for y in MIN_Y..=(-90 + noise as i32) {
            let noise_threshold_min = 0.45;
            let noise_threshold_max = 0.55;
            let noise_cave = get_2d_octaves(x as f32, y as f32, 0.03, 3, 0.5, 1.2, 0., 1., &perm);
            if (y as f32) >= -50. && (y as f32) <= 90. &&
                noise_cave >= noise_threshold_min && noise_cave >= noise_threshold_max {
                    continue;
                }

            let current_particle = select_particle((y + 90) as f32, noise, noise_sand, noise_quicksand);
            if current_particle == ParticleElement::BedRock {
                // place data in map
                map.insert_at::<BedRockParticle>(&mut commands, (x, y), ListType::All);
            } else if current_particle == ParticleElement::Sand {
                map.insert_at::<SandParticle>(&mut commands, (x, y), ListType::All);
            }
            else if current_particle == ParticleElement::QuickSand {
                //map.insert_at::<QuickSandParticle>(&mut commands, (x, y), ListType::All);
            }
        }
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

fn select_particle(y: f32, noise: f32, sand_height: f32, quicksand_height: f32) -> ParticleElement {
    if y >= quicksand_height {
        ParticleElement::Sand
} else if y >= sand_height {
    ParticleElement::Sand
} 
else {
        ParticleElement::BedRock
    }
}

fn update_quicksand(
    mut map: ResMut<ParticleMap>,
    time: Res<Time>, 
    mut commands: Commands,
    mut particles: Query<&mut ParticlePosVel, Or<(With<ParticleTagSand>, With<ParticleTagQuickSand>)>>,
) {

    for _ in 0..RAIN_INTENSITY{
    for mut position in &mut particles {
        let (x, y) = (position.grid_x, position.grid_y);
        if map.get_element_at((x, y+1)) == ParticleElement::Air{
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<QuickSandParticle>(&mut commands, (x, y+1), ListType::OnlyAir);
        }
        if ((map.get_element_at((x + 1, y)) == ParticleElement::Air &&
             map.get_element_at((x+1, y-1)) == ParticleElement::Air)||
            (map.get_element_at((x + 1, y)) == ParticleElement::Air &&
             map.get_element_at((x+1, y-1)) == ParticleElement::Air))&&
           (map.get_element_at((x, y-1)) == ParticleElement::Sand ||
            map.get_element_at((x, y-1)) == ParticleElement::QuickSand ){
            map.delete_at(&mut commands, (x, y));
            map.insert_at::<QuickSandParticle>(&mut commands, (x, y), ListType::OnlyAir);
        }
    }
}
}

fn draw_lava_rain(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
) {
    for _ in 0..RAIN_INTENSITY{
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-(LEVEL_W/2.)..=(LEVEL_W/2.)) as i32;
        let y = rng.gen_range(100..200);
        if map.get_element_at((x, y)) == ParticleElement::Air {
            map.insert_at::<QuickSandParticle>(&mut commands, (x, y), ListType::OnlyAir);
            map.give_velocity(&mut commands, (x,y), RAIN_VEL);
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
        app.add_systems(OnEnter(GamePhase::Planet1), set_crosshair_cursor);
        app.add_systems(OnEnter(GamePhase::Planet1), generate_world);
        app.add_systems(OnEnter(GamePhase::Planet1), update_grass.after(generate_world));
    }
} 
