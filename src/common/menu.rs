use bevy::prelude::*;
//use crate::engine::update_state::AppState;
use crate::common::state::AppState;

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
    mut next_state: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>,

) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if *current_state.get()==AppState::Menu{
                    next_state.set(AppState::InGame);}
                if *current_state.get()==AppState::WinScreen{
                    next_state.set(AppState::EndCredits); }               
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                if *current_state.get()==AppState::Menu{
                    text.sections[0].value = "Play".to_string();}
                if *current_state.get()==AppState::WinScreen{
                        text.sections[0].value = "Credits".to_string();}
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::WHITE;
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
                        width: Val::Px(175.0),
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
                            font: asset_server.load("fonts/Silkscreen-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ), StartButton));
                });
        });
}

pub fn despawn_menu(
    mut commands: Commands,
    query: Query<Entity, With<StartButton>>,
){
    //TODO: Check if collided with blaster particle 
    for entity in query.iter() {
            commands.entity(entity).despawn();
        
    }
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup)
        .add_systems(Update, button_system.run_if(in_state(AppState::Menu)))
        .add_systems(
            OnExit(AppState::Menu), despawn_menu);
        app.add_systems(OnEnter(AppState::WinScreen), setup)
        .add_systems(Update, button_system.run_if(in_state(AppState::WinScreen)))
        .add_systems(
            OnExit(AppState::WinScreen), despawn_menu);

    }
}
