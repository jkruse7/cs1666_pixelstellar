use bevy::prelude::*;
use std::convert::From;
use crate::core::engine::collision::AABB;
use crate::LEVEL_H;
use crate::LEVEL_W;
use crate::WIN_W;
use crate::WIN_H;

const LEVEL_LEN: u32 = LEVEL_W as u32;
const LEVEL_H_INT: i16 = LEVEL_H as i16;
const LEVEL_W_INT: i16 = LEVEL_W as i16;
const TILE_SIZE: u32 = 100;

#[derive(Component)]
pub struct Floor;

// Function to place a tile of any type at a specified location
pub fn place_tile(
    commands: &mut Commands,
    texture: Handle<Image>,               // Handle to the tile's image
    tile_type: impl Component,            // The type of tile (Floor)
    position: Vec2,                       // The position on the grid
) {
    let tile_aabb = AABB::new(
        position,
        position + Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
    );

    // Spawn the tile at the specified position with a collision box (AABB)
    commands.spawn((
        SpriteBundle {
            texture: texture.clone(),
            transform: Transform {
                translation: position.extend(0.0),  // Set the position of the tile
                ..default()
            },
            ..default()
        },
        tile_type,
        tile_aabb,
    ));

   // info!("Placed tile at position: {:?}", position);
}
// Example function to initialize and place floor tiles on the grid
pub fn initialize(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let floor_texture_handle = asset_server.load("planet_0/floor_tile.png");

    let num_tiles_x = (WIN_W / TILE_SIZE as f32).ceil() as u32; // Number of tiles horizontally

    // Calculate the Y positions for the top and bottom of the screen
    let bottom_y = -WIN_H / 2. + TILE_SIZE as f32 / 2.; // Bottom row Y position
    let top_y = WIN_H / 2. - TILE_SIZE as f32 / 2.;     // Top row Y position

    // Loop over each horizontal tile position
    for x in 0..num_tiles_x {
        let x_position = -WIN_W / 2. + x as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.;

        // Place tile at the bottom of the screen
        place_tile(
            &mut commands,
            floor_texture_handle.clone(),
            Floor,  // Tile type is Floor
            Vec2::new(x_position, bottom_y),
        );

        // Place tile at the top of the screen
        place_tile(
            &mut commands,
            floor_texture_handle.clone(),
            Floor,  // Tile type is Floor
            Vec2::new(x_position, top_y),
        );
    }
}