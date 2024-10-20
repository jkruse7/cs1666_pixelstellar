use bevy::prelude::*;
use crate::{
    engine::{
        hitbox::Hitbox,
        gravity::Gravity,
    },
    gameplay::enemy::Enemy,
    LEVEL_H,
    LEVEL_W,
    WIN_W,
    WIN_H,
};

//blaster or gun
#[derive(Component)]
pub struct Blaster;

#[derive(Component)] //direction blaster is facing
pub struct BlasterVector {
    pub vector: Vec2,
}

impl BlasterVector {
    pub fn new() -> Self {
        Self {
            vector: Vec2::splat(0.),
        }
    }
}

pub fn initialize(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){ 
    let blaster_sheet_handle = asset_server.load("blaster.png");
    let blaster_layout = TextureAtlasLayout::from_grid(UVec2::new(19, 11), 1, 1, None, None);
    let blaster_layout_handle = texture_atlases.add(blaster_layout);

    commands.spawn((
        SpriteBundle {
            texture: blaster_sheet_handle,
            transform: Transform {
                translation: Vec3::new(0., -(WIN_H / 2.) + ((100.0 as f32) * 1.5), 905.),
                ..default()
            },
            sprite: Sprite {
                // Flip the logo to the left
                flip_x: false,
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: blaster_layout_handle,
            ..Default::default()
        },
        BlasterVector::new(),
        Blaster,)
    );
}
