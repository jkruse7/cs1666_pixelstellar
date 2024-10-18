use bevy::prelude::*;
use crate::{world::{grid::*, perlin_noise::*}, engine::test_particle::*};

const AMPLITUDE: f32 = 10.0;
const FREQUENCY: f32 = 0.05;
pub fn generate_floor(
    mut grid: ResMut<Grid>,
) {
    for i in 0..grid.w {
        println!("(i: {}, j: {})", i, get_1d_pn_value(i as f32, AMPLITUDE, FREQUENCY).floor() as usize);
        let max = get_1d_pn_value(i as f32, AMPLITUDE, FREQUENCY).floor() as usize + grid.h / 8;
        for j in 0..max {
            grid.set(Index::new(i, j), ParticleType::BedRock);
        }
    }
}