use bevy::prelude::*;
use rand::Rng;
use crate::{common::perlin_noise::{generate_permutation_array, get_1d_octaves, get_2d_octaves}, 
            entities::particle::{components::*, resources::*}, 
            LEVEL_H, LEVEL_W};


pub fn generate_noises(
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

        let mut noise_dirt = get_1d_octaves(x as f32, 0.012, 1, 0.5, 1.2, 0., 20., &perm2);
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
            } else if current_particle == ParticleElement::Dirt {
                map.insert_at::<StoneParticle>(&mut commands, (x, y), ListType::All);
            } else if current_particle == ParticleElement::Stone {
                map.insert_at::<DirtParticle>(&mut commands, (x, y), ListType::All);
            }
        }
    }
}


fn select_particle(y: f32, noise: f32, dirt_height: f32, stone_height: f32) -> ParticleElement {
    if y >= stone_height {
        ParticleElement::Stone
    } else if y >= dirt_height{
        ParticleElement::Dirt
    } else {
        ParticleElement::BedRock
    }
}


fn draw_rain(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
) {
    for _ in 0..2{
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-(LEVEL_W/2.)..(LEVEL_W/2.)) as i32;
        let y = rng.gen_range(((LEVEL_H/2.)-10.)..=((LEVEL_H/2.)-1.)) as i32;
        if map.get_element_at((x, y)) == ParticleElement::Air {
            map.insert_at::<WaterParticle>(&mut commands, (x, y), ListType::OnlyAir);
        }
    }
}


pub struct Planet0Plugin;
impl Plugin for Planet0Plugin {
    fn build(&self, app: &mut App) {
        // Startup placements
        app.add_systems(Startup, generate_noises);
        app.add_systems(Update, draw_rain);
    }
} 
