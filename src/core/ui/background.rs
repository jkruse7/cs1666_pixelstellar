use bevy::prelude::*;
use std::convert::From;

use crate::WIN_H;
use crate::WIN_W;
const WIN_H_INT: i16 = WIN_H as i16;
const WIN_W_INT: i16 = WIN_W as i16;





#[derive(Component)]
struct Background;

pub fn initialize (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let bg_texture_handle = asset_server.load("background.png");
    for x in 0i16..5{
        for y in 0i16..5{
            let t = Vec3::new(
                f32::from(x * WIN_H_INT - WIN_H_INT * 2) * f32::from(x) * 1.2,
                f32::from(y * WIN_W_INT - WIN_W_INT * 2) * f32::from(y) * 1.2,
                -10.,
            );
            commands.spawn(SpriteBundle {
                texture: bg_texture_handle.clone(),
                transform: Transform {
                    translation: t,
                    ..default()
                },
                ..default()
            }).insert(Background);
        }
    }
    let bg_texture_handle = asset_server.load("background.png");

    commands
        .spawn(SpriteBundle {
            texture: bg_texture_handle.clone(),
            transform: Transform::from_translation(Vec3::splat(0.)),
            ..default()
        })
        .insert(Background);



}