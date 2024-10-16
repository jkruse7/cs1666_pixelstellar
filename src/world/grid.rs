use bevy::prelude::*;

use crate::engine::particles::Particle;
use crate::engine::particles::ELEMENT;
use crate::engine::particles::PARTICLE_SIZE;
use crate::LEVEL_H;
use crate::LEVEL_W;

use crate::world::perlin_noise::get_1d_pn_value;


#[derive(Component, Resource)]
pub struct StateArray {
    pub grid: Vec<Vec<i32>>,
}

impl StateArray {
    // Constructor to initialize the grid with the given dimensions
    pub fn initialize(level_w: f32, level_h: f32, particle_size: f32) -> Self {
        // Calculate the number of rows and columns based on level dimensions and particle size
        let rows: usize = (level_h / particle_size) as usize;
        let cols: usize = (level_w / particle_size) as usize;
        print!("rows: {}, cols{}\n", rows, cols);
        // Create a 2D vector initialized to zeros
        let grid = vec![vec![0; cols]; rows];

        StateArray {
            grid,
        }
    }


    // Getter method to access the 2D array
    pub fn get_state_array(&self) -> &Vec<Vec<i32>> {
        &self.grid
    }

    // Method to update a specific cell in the grid
    pub fn spawn(&mut self, commands: &mut Commands, col: usize, row: usize, value: i32) {

        //Transform::from_translation(Vec3::new(-3200. + 8.0, 1800. - 8.0, 0.)),
        //top left: -3192, 1792
        if col < self.grid.len() && row < self.grid[col].len() {
            let x: f32 = (col as f32) * PARTICLE_SIZE - (LEVEL_W / 2.) + (PARTICLE_SIZE / 2.);
            let y: f32 = (row as f32) * -PARTICLE_SIZE + (LEVEL_H / 2.) - (PARTICLE_SIZE / 2.);


            self.grid[row][col] = value;

            let bedrock = Particle::new(
                false,
                ELEMENT::WATER,
                true,
                true,
                Vec2::new(0., 0.),
                Transform::from_translation(Vec3::new(x, y, 0.)),
            );
            //info!("Spawned at (bevy coords): {}, {}", x, y);
            Particle::spawn_particle(commands, bedrock);
        }
    }
    pub fn get_closest(x: f32, y: f32) -> (usize, usize){
        /*
        info!("({}, {})({}, {})({}, {})",
            StateArray::get_closest(-3199., 1799.).0, StateArray::get_closest(-3199., 1799.).1,
            StateArray::get_closest(-3196., 1796.).0, StateArray::get_closest(-3196., 1796.).1,
            StateArray::get_closest(-3195., 1795.).0, StateArray::get_closest(-3195., 1795.).1);
        */
        let col: f32 = f32::round(( x + (LEVEL_W / 2.) - (PARTICLE_SIZE / 2.) ) / PARTICLE_SIZE);
        let row: f32 = f32::round(( y - (LEVEL_H / 2.) + (PARTICLE_SIZE / 2.) ) / -PARTICLE_SIZE);
        //info!("ROW: {} COL: {}", row, col);
        (col as usize, row as usize)
    }
    

}


