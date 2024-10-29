use bevy::prelude::*;
//use crate::engine::update_state::AppState;
use crate::GameState;

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
/*pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let button_material = Color::rgb(0.15, 0.15, 0.15);
    let font = asset_server.load("fonts/Silkscreen-Bold.ttf");

    // 添加 Start Game 按钮
    commands.spawn(ButtonBundle {
        background_color: bevy::prelude::BackgroundColor(button_material.clone()),
        ..default()
    })
    .insert(MenuButton)
    .insert(MenuAction::StartGame)
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: "Start Game",
            style: TextStyle {
                font: font.clone(),
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
            ..default()

    });
    });

    // 添加 Settings 按钮
    commands.spawn(ButtonBundle {
        background_color: bevy::prelude::BackgroundColor(button_material.clone()),
        ..Default::default()
    })
    .insert(MenuButton)
    .insert(MenuAction::Settings)
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: "Settings",
            style: TextStyle {
                font: font.clone(),
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
            ..default()
    });
    });

    // 添加 Dev Lab 按钮
    commands.spawn(ButtonBundle {
        background_color:bevy::prelude::BackgroundColor(button_material.clone()),
        ..Default::default()
    })
    .insert(MenuButton)
    .insert(MenuAction::DevLab)
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: "Dev Lab",
            style: TextStyle {
                font: font.clone(),
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
            ..default()
    });
    });

    // 添加 Exit 按钮
    commands.spawn(ButtonBundle {
        background_color: bevy::prelude::BackgroundColor(button_material.clone()),
        ..Default::default()
    })
    .insert(MenuButton)
    .insert(MenuAction::Exit)
    .with_children(|parent| {
        parent.spawn(TextBundle {
                text: "Exit",
                style: TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
                ..default()
        });
    });
}

// 按钮交互逻辑
pub fn button_interaction(
    mut interaction_query: Query<(&Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match action {
                MenuAction::StartGame => next_state.set(GameState::Level1),
                //MenuAction::Settings => next_state.set(AppState::Setting),
                //MenuAction::DevLab => next_state.set(AppState::DevLab),
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


/*use bevy::prelude::*;
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
*/
// Generate main menu with custom buttons
/*pub fn spawn_main_menu(
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
}*/*/

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct StartButton;

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Level1);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
    StartButton,))
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                }, StartButton))
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ), StartButton));
                });
        });
}

pub fn despawn_menu(
    mut commands: Commands,
    query: Query<(Entity), With<StartButton>>,
){
    //TODO: Check if collided with blaster particle 
    for (entity) in query.iter() {
            commands.entity(entity).despawn();
        
    }
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup)
        .add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)))
        .add_systems(
            OnExit(GameState::MainMenu), despawn_menu);

    }
}
