use bevy::prelude::*;
use crate::{LEVEL_H, LEVEL_W,};

const BG_TILE_SIZE: u32 = 150;

#[derive(Component)]
pub struct Background;

pub fn initialize_background (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let planet: u32 = 0;
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

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_background);
    }
}