use bevy::prelude::*;
use std::collections::HashMap;
use crate::{
    LEVEL_W,
    LEVEL_H,
    engine::test_particle::*,
};

const GRID_W: usize = (LEVEL_W / PARTICLE_SIZE) as usize;
const GRID_H: usize = (LEVEL_H / PARTICLE_SIZE) as usize;

#[derive(Resource)]
pub struct Grid {
    pub w: usize,
    pub h: usize,
    grid: [[ParticleType; GRID_H]; GRID_W],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            w: GRID_W,
            h: GRID_H,
            grid: [[ParticleType::Air; GRID_H]; GRID_W],
        }
    }

    pub fn get(&self, index: Index) -> ParticleType {
        self.grid[index.i][index.j]
    }

    pub fn set(&mut self, index: Index, block: ParticleType) {
        self.grid[index.i][index.j] = block;
    }
}

#[derive(Component, Copy, Clone)]
pub struct Index {
    pub i: usize,
    pub j: usize,
}

impl Index {
    pub fn new(i: usize, j: usize) -> Self {
        Self {
            i: i,
            j: j,
        }
    }

    pub fn up(&self) -> Index {
        if self.j < GRID_H - 1 {
            Index::new(self.i, self.j + 1)
        } else {
            *self
        }
    }

    pub fn down(&self) -> Index {
        if self.j > 0 {
            Index::new(self.i, self.j - 1)
        } else {
            *self
        }
    }

    pub fn left(&self) -> Index {
        if self.i < GRID_W - 1 {
            Index::new(self.i + 1, self.j)
        } else {
            *self
        }
    }

    pub fn right(&self) -> Index {
        if self.i > 0 {
            Index::new(self.i - 1, self.j)
        } else {
            *self
        }
    }
}