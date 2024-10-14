use bevy::prelude::*;

use crate::engine::{
    gravity::Gravity,
    hitbox::Hitbox,
};

// 水块组件
#[derive(Component)]
pub struct WaterTile {
    pub gravity: Gravity,
    pub hitbox: Hitbox,
}

impl WaterTile {
    pub fn new(width: f32, height: f32, offset: Vec2) -> Self {
        Self {
            gravity: Gravity::new(),
            hitbox: Hitbox {
                width,
                height,
                offset,
            },
        }
    }
}

// 设置水块
pub fn setup_water_tiles(
    mut commands: Commands,
    tile_size: f32,
    tile_count_x: u32,
    tile_count_y: u32,
) {
    for x in 0..tile_count_x {
        for y in 0..tile_count_y {
            let x_pos = x as f32 * tile_size - (tile_size * tile_count_x as f32 / 2.0);
            let y_pos = y as f32 * tile_size - (tile_size * tile_count_y as f32 / 2.0);
            
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 0.0, 1.0, 0.5), // 半透明蓝色
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(x_pos, y_pos, 0.0),
                        scale: Vec3::splat(tile_size),
                        ..default()
                    },
                    ..default()
                },
                WaterTile::new(tile_size, tile_size, Vec2::new(x_pos, y_pos)), // 使用 tile_size 初始化 hitbox
            ));
        }
    }
}

// 更新水块的位置
pub fn update_water_tiles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &WaterTile)>,
) {
    for (mut transform, water_tile) in query.iter_mut() {
        // 以固定速度向下移动
        transform.translation.y -= 50.0 * time.delta_seconds(); // 50.0 是下落速度，你可以根据需要调整

        // 这里可以添加其他逻辑，例如边界检测等
        // 例如：如果水块下移到某个边界，可以选择删除或停下
    }
}

// 这个函数用于更新重力
fn update_gravity(gravity: &mut Gravity, time: &Res<Time>) {
    let current_velocity = gravity.get_g(); // 获取当前的重力值
    gravity.update_g(&current_velocity, &time.delta_seconds());
}

// 水块的水平移动逻辑
fn move_water_tile(transform: &mut Transform, water_tile: &WaterTile, has_left_water: bool, has_right_water: bool) {
    if !has_left_water && transform.translation.x + water_tile.hitbox.width < 100.0 { // 右边界
        transform.translation.x += water_tile.hitbox.width * 0.1; // 水块向右移动
    } else if !has_right_water && transform.translation.x - water_tile.hitbox.width > -100.0 { // 左边界
        transform.translation.x -= water_tile.hitbox.width * 0.1; // 水块向左移动
    }
}
