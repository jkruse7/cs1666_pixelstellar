use bevy::prelude::*;
use crate::engine::update_state::AppState;

// 定义菜单按钮和操作
#[derive(Component)]
struct MenuButton;

#[derive(Component)]
enum MenuAction {
    StartGame,
    Settings,
    DevLab,
    Exit,
}

// 生成主菜单界面
pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let button_material = materials.add(Color::rgb(0.15, 0.15, 0.15).into());
    let font = asset_server.load("fonts/Silkscreen-Bold.ttf");

    // 添加 Start Game 按钮
    commands.spawn_bundle(ButtonBundle {
        material: button_material.clone(),
        ..Default::default()
    })
    .insert(MenuButton)
    .insert(MenuAction::StartGame)
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Start Game",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        });
    });

    // 添加 Settings 按钮
    commands.spawn_bundle(ButtonBundle {
        material: button_material.clone(),
        ..Default::default()
    })
    .insert(MenuButton)
    .insert(MenuAction::Settings)
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Settings",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        });
    });

    // 添加 Dev Lab 按钮
    commands.spawn_bundle(ButtonBundle {
        material: button_material.clone(),
        ..Default::default()
    })
    .insert(MenuButton)
    .insert(MenuAction::DevLab)
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Dev Lab",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        });
    });

    // 添加 Exit 按钮
    commands.spawn_bundle(ButtonBundle {
        material: button_material,
        ..Default::default()
    })
    .insert(MenuButton)
    .insert(MenuAction::Exit)
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Exit",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        });
    });
}

// 按钮交互逻辑
pub fn button_interaction(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<(&Interaction, &MenuAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in interaction_query.iter_mut() {
        if *interaction == Interaction::Clicked {
            match action {
                MenuAction::StartGame => next_state.set(AppState::InGame),
                MenuAction::Settings => next_state.set(AppState::Setting),
                MenuAction::DevLab => next_state.set(AppState::DevLab),
                MenuAction::Exit => std::process::exit(0), // 退出游戏
            }
        }
    }
}

// 清理主菜单
pub fn cleanup_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuButton>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


use bevy::prelude::*;
use crate::engine::update_state::AppState;
use crate::button_plugin::{spawn_custom_button, ButtonColors};

// Define menu actions
#[derive(Component)]
struct MenuButton;

#[derive(Component)]
enum MenuAction {
    StartGame,
    Settings,
    DevLab,
    Exit,
}

// Generate main menu with custom buttons
pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    font: Handle<Font>,
) {
    let button_size = Vec2::new(200.0, 60.0);
    let button_margin = UiRect::all(Val::Px(10.0));

    // Start Game Button
    spawn_custom_button(
        &mut commands,
        "Start Game",
        button_size,
        button_margin.clone(),
        font.clone(),
        None,  // Use default colors
        None,
        None,
        None,
    );

    // Settings Button
    spawn_custom_button(
        &mut commands,
        "Settings",
        button_size,
        button_margin.clone(),
        font.clone(),
        None,  // Use default colors
        None,
        None,
        None,
    );

    // Dev Lab Button
    spawn_custom_button(
        &mut commands,
        "Dev Lab",
        button_size,
        button_margin.clone(),
        font.clone(),
        None,  // Use default colors
        None,
        None,
        None,
    );

    // Exit Button
    spawn_custom_button(
        &mut commands,
        "Exit",
        button_size,
        button_margin,
        font,
        None,  // Use default colors
        None,
        None,
        None,
    );
}
