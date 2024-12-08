use bevy::prelude::*;
use crate::{LEVEL_H, LEVEL_W, common::state::GamePhase};

const BG_TILE_SIZE: u32 = 100;

#[derive(Component)]
pub struct Background;

pub fn initialize_background (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    state: Res<State<GamePhase>>,
) {
    let planet: i32 = match state.get() {
        GamePhase::Planet1 => 1,
        GamePhase::Planet2 => 2,
        GamePhase::Planet3 => 3,
        GamePhase::Planet4 => 4,
        GamePhase::Planet5 => 5,
        GamePhase::Planet6 => 6,
        GamePhase::Planet7 => 7,
        GamePhase::Planet8 => 8,
        _ => 0,
    };
    let bg_sheet_handle = asset_server.load(format!("planet_{planet}/background.png"));
    let bg_layout = TextureAtlasLayout::from_grid(UVec2::splat(BG_TILE_SIZE), 1, 1, None, None);
    let bg_layout_handle = texture_atlases.add(bg_layout);

    for x in (((-LEVEL_W / 2.) as isize)..((LEVEL_W / 2.) as isize)).step_by(BG_TILE_SIZE as usize){
        for y in (((-LEVEL_H / 2.) as isize)..((LEVEL_H / 2.) as isize)).step_by(BG_TILE_SIZE as usize){

            let t = Vec3::new(
                (x + (BG_TILE_SIZE / 2) as isize) as f32,
                (y + (BG_TILE_SIZE / 2) as isize) as f32,
                -900.,
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
