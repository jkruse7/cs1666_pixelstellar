//TODO: enemy,  gun permissions

use bevy::prelude::*;
use rand::Rng;
use crate::common::gravity::{change_gravity, GravityResource};
use crate::common::state::GamePhase;
use crate::entities::particle::{resources::*, components::*};
use crate::common::perlin_noise::*;
use crate::LEVEL_W;

const RAIN_INTENSITY: i32 = 6;
const RAIN_VEL: Vec2 = Vec2::new(2., -0.7);

// Map placement type functions  --------------------------------------------------------------------------------
fn generate_world(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
    grav_res: ResMut<GravityResource>,
) {
    change_gravity(grav_res, 1400., 600.);
    let perm1 = generate_permutation_array();
    let perm2 = generate_permutation_array();
    let perm3 = generate_permutation_array();
    // loop from left side of the screen to right side of the screen
    for x in MIN_X..=MAX_X {
        let mut noise = get_1d_octaves(x as f32, 0.008, 4, 0.5, 1.2, 0., 180., &perm1);
        noise = noise.floor();

        let mut noise_dirt = get_1d_octaves(x as f32, 0.012, 1, 0.5, 2.2, 0., 20., &perm2);
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
                map.insert_at::<BedRockParticle>(&mut commands, (x, y), ListType::All);
            } else if current_particle == ParticleElement::Hellstone {
                map.insert_at::<HellstoneParticle>(&mut commands, (x, y), ListType::All);
            } else if current_particle == ParticleElement::Stone {
                map.insert_at::<StoneParticle>(&mut commands, (x, y), ListType::All);
            }
        }
    }
}
fn select_particle(y: f32, noise: f32, dirt_height: f32, stone_height: f32) -> ParticleElement {
    if y >= stone_height {
        ParticleElement::Hellstone
    } else if y >= dirt_height{
        ParticleElement::Stone
    } else {
        ParticleElement::BedRock
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
            map.insert_at::<LavaParticle>(&mut commands, (x, y), ListType::OnlyAir);
            map.give_velocity(&mut commands, (x,y), RAIN_VEL);
        }
    }
}

pub struct Planet3Plugin;
impl Plugin for Planet3Plugin {
    fn build(&self, app: &mut App) {
        // Startup placements
        app.add_systems(OnEnter(GamePhase::Planet3), crate::common::ui::background::initialize_background);
        app.add_systems(OnEnter(GamePhase::Planet3), generate_world);
        app.add_systems(Update, draw_lava_rain.run_if(in_state(GamePhase::Planet3)));
    }
} 
