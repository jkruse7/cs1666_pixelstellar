// use bevy::prelude::*;
// use std::convert::From;

// use crate::{
//     world::{
//         planet1,
//         perlin_noise::get_1d_pn_value,
//     },
//     engine::hitbox::Hitbox,
//     LEVEL_H,
//     LEVEL_W,
//     WIN_H,
//     WIN_W,
// };

// const LEVEL_LEN: u32 = LEVEL_W as u32;
// const LEVEL_H_INT: i16 = LEVEL_H as i16;
// const LEVEL_W_INT: i16 = LEVEL_W as i16;
// const FLOOR_TILE_SIZE: u32 = 50;

// #[derive(Component)]
// pub struct Floor;

// pub fn initialize(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     let floor_sheet_handle = asset_server.load("planet_0/floor_tile.png");
//     let floor_layout = TextureAtlasLayout::from_grid(UVec2::splat(FLOOR_TILE_SIZE), 1, 1, None, None);
//     let floor_layout_len = floor_layout.len();
//     let floor_layout_handle = texture_atlases.add(floor_layout);


//     let mut i = 0;
//     let mut t = Vec2::new(
//         -WIN_W + (FLOOR_TILE_SIZE as f32) / 2.,
//         -WIN_H + (FLOOR_TILE_SIZE as f32) / 2.,
//     );
//     while i * FLOOR_TILE_SIZE < (LEVEL_LEN as u32) {
//         // Create noise with x coordinates, fixed y, an amplitude of 10, and a frequency of 0.01
//         let mut noise = get_1d_pn_value(t.x, 10., 0.01);
//         noise = noise.floor();
//         let mut j = 0;

//         // This inner loop is going to stack tiles at a fixed x coordinate
//         while j <= (noise as u32) {
//             t += Vec2::new(0., (FLOOR_TILE_SIZE) as f32);
//             let floor_hitbox = crate::engine::hitbox::Hitbox::new(FLOOR_TILE_SIZE as f32, FLOOR_TILE_SIZE as f32, Vec2::new(t.x, t.y));
//             commands.spawn((
//                 SpriteBundle {
//                     texture: floor_sheet_handle.clone(),
//                     transform: Transform {
//                         translation: t.extend(0.0),
//                         ..default()
//                     },
//                     ..default()
//                 },
//                 TextureAtlas {
//                     layout: floor_layout_handle.clone(),
//                     index: (i as usize) % floor_layout_len,
//                 },
//                 Floor,
//                 floor_hitbox,
//             ));
//             j += 1;
//         }
//         t -= Vec2::new(0., (j * FLOOR_TILE_SIZE) as f32);

//         i += 1;
//         t += Vec2::new((FLOOR_TILE_SIZE) as f32, 0.);
//     }

//     // //ceiling
//     // i=0;
//     // t = Vec2::new(
//     //     -WIN_W / 2. + (5.0 * FLOOR_TILE_SIZE as f32) / 2.,
//     //     WIN_H / 2. + (FLOOR_TILE_SIZE as f32) / 2.,
//     // );
//     // while (5+i) * FLOOR_TILE_SIZE < (LEVEL_LEN as u32) {
//     //     info!("Spawning brick at {:?}", t);

//     //     let floor_hitbox = Hitbox::new(FLOOR_TILE_SIZE as f32, FLOOR_TILE_SIZE as f32, Vec2::new(t.x, t.y));
//     //     //info! ("Floor AABB: {:?}", floor_aabb);
//     //     commands.spawn((
//     //         SpriteBundle {
//     //             texture: floor_sheet_handle.clone(),
//     //             transform: Transform {
//     //                 translation: t.extend(0.0),
//     //                 ..default()
//     //             },
//     //             ..default()
//     //         },
//     //         TextureAtlas {
//     //             layout: floor_layout_handle.clone(),
//     //             index: (i as usize) % floor_layout_len,
//     //         },
//     //         Floor,
//     //         floor_hitbox,
//     //     ));

//     //     i += 1;
//     //     t += Vec2::new((FLOOR_TILE_SIZE) as f32, 0.);
//     // }
// }


