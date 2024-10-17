use bevy::prelude::*;
use rand::Rng;
use crate::{
    LEVEL_W,
    LEVEL_H,
    engine::test_particle::*,
};

pub const GRID_W: f32 = LEVEL_W / PARTICLE_SIZE;
pub const GRID_H: f32 = LEVEL_H / PARTICLE_SIZE;

#[derive(Component)]
pub struct Index {
    pub i: i32,
    pub j: i32,
}

impl Index {
    pub fn new(i: i32, j: i32) -> Self {
        Self {
            i: i,
            j: j,
        }
    }
}

#[derive(Resource)]
pub struct Grid {
    grid: [[ParticleType; GRID_H as usize]; GRID_W as usize],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            grid: [[ParticleType::Air; GRID_H as usize]; GRID_W as usize],
        }
    }

    pub fn get(&self, i: i32, j: i32) -> ParticleType {
        self.grid[i as usize][j as usize]
    }

    pub fn set(&mut self, i: i32, j: i32, block: ParticleType) {
        self.grid[i as usize][j as usize] = block;
    }
}

pub fn draw_water(
    mut grid: ResMut<Grid>,
) {
    let mut rng = rand::thread_rng();
    for _t in 0..100 {
        let i = rng.gen_range(0..GRID_W as i32);
        let j = rng.gen_range(0..GRID_H as i32);
        grid.set(i, j, ParticleType::Water);
    }
}

pub fn update_grid(mut grid: ResMut<Grid>) {
    for i in 0..GRID_W as i32 {
        for j in 0..GRID_H as i32 {
            if grid.get(i, j) != ParticleType::Water {
                continue;
            }

            if j > 0 {
                match grid.get(i, j - 1) {
                    ParticleType::Air => {
                        grid.set(i, j, ParticleType::Air);
                        grid.set(i, j - 1, ParticleType::Water);
                    },
                    ParticleType::BedRock => {
            
                    },
                    ParticleType::Water => {

                    },
                }
            } else {
                grid.set(i, j, ParticleType::Air);
                grid.set(i, GRID_H as i32 - 1, ParticleType::Water);
            }
        }
    }
}