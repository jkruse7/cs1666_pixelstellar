use bevy::prelude::*;
use rand::Rng;
use crate::common::gravity::{change_gravity, GravityResource};
use crate::common::state::GamePhase;
use crate::entities::particle::{resources::*, components::*};
use crate::common::perlin_noise::*;
use crate::LEVEL_W;

const DIRT_INTENSITY: i32 = 10;
const DIRT_VEL: Vec2 = Vec2::new(2., -20.);

#[derive(Resource)]
pub struct WorldGenSettings {
    pub height_noise: NoiseSettings,
    pub dirt_noise: NoiseSettings, 
    pub slime_noise: NoiseSettings, 
}

#[derive(Resource)]
pub struct NoiseSettings {
    pub start_frequency: f32,
    pub octaves: usize,
    pub persistence: f32,
    pub frequency_modifier: f32,
    pub noise_range_min: f32,
    pub noise_range_max: f32,
}

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
                start_frequency: 0.0015,
                octaves: 2,
                noise_range_min: 0.,
                noise_range_max: 20.,
                ..Default::default()
            },
            slime_noise: NoiseSettings {
                start_frequency: 0.0015,
                octaves: 1,
                noise_range_min: 40.,
                noise_range_max: 30.,
                ..Default::default()
            },
        }
    }
}

fn generate_world(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
    config: Res<WorldGenSettings>,
) {
    crate::common::gravity::initialize(&mut commands);

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
        ).floor();

        let noise_dirt = get_1d_octaves(
            x as f32,
            config.dirt_noise.start_frequency,
            config.dirt_noise.octaves,
            config.dirt_noise.persistence,
            config.dirt_noise.frequency_modifier,
            config.dirt_noise.noise_range_min,
            config.dirt_noise.noise_range_max,
            &perm,
        ).floor();

        let noise_slime = get_1d_octaves(
            x as f32,
            config.slime_noise.start_frequency,
            config.slime_noise.octaves,
            config.slime_noise.persistence,
            config.slime_noise.frequency_modifier,
            config.slime_noise.noise_range_min,
            config.slime_noise.noise_range_max,
            &perm,
        ).floor();

        for y in MIN_Y..=(-90 + noise as i32) {
            let noise_cave = get_2d_octaves(x as f32, y as f32, 0.03, 3, 0.5, 1.2, 0., 1., &perm);
            if (y as f32) >= -50. && (y as f32) <= 90. && noise_cave >= 0.45 && noise_cave <= 0.55 {
                continue;
            }

            let current_particle = select_particle((y + 90) as f32, noise, noise_dirt, noise_slime);
            if current_particle == ParticleElement::BedRock {
                map.insert_at::<BedRockParticle>(&mut commands, (x, y), ListType::All);
            } else if current_particle == ParticleElement::Dirt {
                map.insert_at::<DirtParticle>(&mut commands, (x, y), ListType::All);
            } else if current_particle == ParticleElement::Slime {
                map.insert_at::<SlimeParticle>(&mut commands, (x, y), ListType::All);
            }
        }
    }
}

fn select_particle(y: f32, noise: f32, dirt_height: f32, slime_height: f32) -> ParticleElement {
    if y >= slime_height {
        ParticleElement::Dirt
    } else if y >= dirt_height {
        ParticleElement::Dirt
    } else {
        ParticleElement::BedRock
    }
}

fn update_slime(
    mut map: ResMut<ParticleMap>,
    time: Res<Time>, 
    mut commands: Commands,
    mut particles: Query<&mut ParticlePosVel, Or<(With<ParticleTagDirt>, With<ParticleTagSlime>)>>,
) {

    for _ in 0..DIRT_INTENSITY {
        for mut position in &mut particles {
            let (x, y) = (position.grid_x, position.grid_y);

            if map.get_element_at((x, y + 1)) == ParticleElement::Air {
                map.delete_at(&mut commands, (x, y));
                map.insert_at::<SlimeParticle>(&mut commands, (x, y + 1), ListType::OnlyAir);
            }

            if ((map.get_element_at((x + 1, y)) == ParticleElement::Air &&
                 map.get_element_at((x + 1, y - 1)) == ParticleElement::Air) ||
                (map.get_element_at((x - 1, y)) == ParticleElement::Air &&
                 map.get_element_at((x - 1, y - 1)) == ParticleElement::Air)) &&
                (map.get_element_at((x, y - 1)) == ParticleElement::Dirt ||
                 map.get_element_at((x, y - 1)) == ParticleElement::Slime)
            {
                map.delete_at(&mut commands, (x, y));
                map.insert_at::<SlimeParticle>(&mut commands, (x, y), ListType::OnlyAir);
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

fn set_crosshair_cursor( mut q_window: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
) {
    let mut window = q_window.single_mut();
    window.cursor.icon = CursorIcon::Cell;
}

pub struct Planet7Plugin;
impl Plugin for Planet7Plugin {
    fn build(&self, app: &mut App) {
        // Startup placements
        app.add_systems(OnEnter(GamePhase::Planet7), crate::common::ui::background::initialize_background);
        app.insert_resource(WorldGenSettings::default());
        app.add_systems(OnEnter(GamePhase::Planet7), set_crosshair_cursor);
        app.add_systems(OnEnter(GamePhase::Planet7), generate_world);
        app.add_systems(OnEnter(GamePhase::Planet7), update_grass.after(generate_world));
    
        app.add_systems(OnEnter(GamePhase::Planet7), update_slime.after(generate_world));
    }
} 
 
