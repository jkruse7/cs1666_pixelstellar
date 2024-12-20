use bevy::{
    
    app::{App, Startup,},
    asset::AssetServer,
    ecs::system::{Commands, Res},
    sprite::SpriteBundle,
    utils::default,
    window::{CursorGrabMode},
    prelude::*,
};

use crate::common::state::AppState;

#[derive(Resource)]
pub struct Counter {
    timer: Timer,
    current_image: usize,
}


impl Counter {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            current_image: 0,
        }
    }
}


impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Resource)]
struct ImageAssets {
    handles: Vec<Handle<Image>>,
}

impl FromWorld for ImageAssets {
    fn from_world(_world: &mut World) -> Self {
        ImageAssets {
            handles: Vec::new(),
        }
    }
}


fn preload_assets(asset_server: Res<AssetServer>, mut images: ResMut<ImageAssets>) {
    let names = vec![
        "image1.png",
        "image2.png",
        "image3.png",
        "image4.png",
        "image5.png",
        "image6.png",
        "image7.png",
    ];

    for name in names {
        images.handles.push(asset_server.load(name));
    }
}

fn countdown(time: Res<Time>, mut counter: ResMut<Counter>) {
    counter.timer.tick(time.delta());
}

fn switch_image(
    mut commands: Commands, 
    mut counter: ResMut<Counter>, 
    images: Res<ImageAssets>, 
    query: Query<Entity, With<Handle<Image>>>, 
) {
    if counter.timer.just_finished() {
        counter.current_image = (counter.current_image + 1) % images.handles.len();

        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        commands.spawn(SpriteBundle {
            texture: images.handles[counter.current_image].clone(),
            ..default()
        });
    }
}


fn change_title(mut windows: Query<&mut Window>, time: Res<Time>) {
    let mut window = windows.single_mut();
    window.title = format!(
        "Seconds since startup: {}",
        time.elapsed().as_secs_f32().round()
    );
}

fn toggle_cursor(mut windows: Query<&mut Window>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        let mut window = windows.single_mut();

        window.cursor.visible = !window.cursor.visible;
        window.cursor.grab_mode = match window.cursor.grab_mode {
            CursorGrabMode::None => CursorGrabMode::Locked,
            CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
        };
    }
}

fn cycle_cursor_icon(
    mut windows: Query<&mut Window>,
    input: Res<ButtonInput<MouseButton>>,
    mut index: Local<usize>,
) {
    let mut window = windows.single_mut();

    const ICONS: &[CursorIcon] = &[
        CursorIcon::Default,
        CursorIcon::Pointer,
        CursorIcon::Wait,
        CursorIcon::Text,
        CursorIcon::Copy,
    ];

    if input.just_pressed(MouseButton::Left) {
        *index = (*index + 1) % ICONS.len();
    } else if input.just_pressed(MouseButton::Right) {
        *index = if *index == 0 {
            ICONS.len() - 1
        } else {
            *index - 1
        };
    }

    window.cursor.icon = ICONS[*index];
}

pub struct EndCreditsPlugin;
impl Plugin for EndCreditsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Counter>()
        .init_resource::<ImageAssets>()
        .add_systems(Update, (countdown, switch_image).run_if(in_state(AppState::EndCredits)))
        .add_systems(Startup, preload_assets)
        .add_systems(
            Update,
            (change_title, toggle_cursor, cycle_cursor_icon).run_if(in_state(AppState::EndCredits)),
        );

    }
}