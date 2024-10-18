use bevy::prelude::*;
use crate::{world::{grid::*, perlin_noise::*}, engine::test_particle::*};

pub fn generate_floor(
    mut grid: ResMut<Grid>,
) {
    for i in 0..grid.w {
        println!("(i: {}, j: {})", i, get_1d_pn_value(i as f32, 100.0, 0.001).floor() as usize);
        let max = get_1d_pn_value(i as f32, 10.0, 0.01).floor() as usize + grid.h / 8;
        for j in 0..max {
            grid.set(Index::new(i, j), ParticleType::BedRock);
        }
    }
}