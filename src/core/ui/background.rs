use bevy::prelude::*;
use std::convert::From;

use crate::core::world::planet1;

use crate::LEVEL_H;
use crate::LEVEL_W;
const LEVEL_H_INT: i16 = LEVEL_H as i16;
const LEVEL_W_INT: i16 = LEVEL_W as i16;

const BG_TILE_SIZE: u32 = 150;


#[derive(Component)]
struct Background;

pub fn initialize (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let planet: u32 = 0;
    let bg_sheet_handle = asset_server.load(format!("planet_{planet}/background.png"));
    let bg_layout = TextureAtlasLayout::from_grid(UVec2::splat(BG_TILE_SIZE), 1, 1, None, None);
    //let bg_layout_len = bg_layout.textures.len();
    let bg_layout_handle = texture_atlases.add(bg_layout);

    // Start at 0, 0 (-3200, -1800)

    for x in ((-LEVEL_W_INT / 2)..(LEVEL_W_INT / 2)).step_by(BG_TILE_SIZE as usize){
        for y in ((-LEVEL_H_INT / 2)..(LEVEL_H_INT / 2)).step_by(BG_TILE_SIZE as usize){

            let t = Vec3::new(
                f32::from(x + (BG_TILE_SIZE / 2) as i16),
                f32::from(y + (BG_TILE_SIZE / 2) as i16),
                -10.,
            );

            commands.spawn((
                SpriteBundle {
                    texture: bg_sheet_handle.clone(),
                    transform: Transform {
                        translation: t,
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    index: 0,
                    //index: i % bg_layout_len   // add an i counter
                    layout: bg_layout_handle.clone(),
                },
            )).insert(Background);
        }
    }



}