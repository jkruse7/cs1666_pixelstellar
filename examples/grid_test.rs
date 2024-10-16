use bevy::{
    prelude::*,
    window::PresentMode
};

const TITLE: &str = "Test";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const CELL_SIZE: f32 = 4.;

#[derive(Component)]
struct Cell {
    x: isize,
    y: isize,
}

impl Cell {
    fn new(x: isize, y: isize) -> Self {
        Self {
            x: x,
            y: y,
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(0, 0, 0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_grid)
        .add_systems(PostStartup, check_grid)
        .run();
}

fn setup_grid(mut commands: Commands) {
    let mut i: f32 = -WIN_W / 2. + CELL_SIZE / 2.;
    while i <= WIN_W / 2. - CELL_SIZE / 2. {
        let mut j: f32 = -WIN_H / 2. + CELL_SIZE / 2.;
        while j <= WIN_H / 2. - CELL_SIZE / 2. {
            commands
                .spawn(Cell::new((i / CELL_SIZE) as isize, (j / CELL_SIZE) as isize));
            j += CELL_SIZE;
        }
        i += CELL_SIZE;
    }
}

fn check_grid(cells: Query<&Cell>) {
    for cell in &cells {
        println!("x: {}", cell.x);
        println!("y: {}", cell.y);
    }
}