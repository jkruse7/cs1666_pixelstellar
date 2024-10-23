use bevy::prelude::*;

// Define the InputPlugin struct
pub struct InputPlugin; 

// // Implement the Plugin trait for InputPlugin
// impl Plugin for InputPlugin {
//     fn build(&self, app: &mut App) {
//         // Add systems for handling input
//         app.add_system(handle_keyboard_input)
//            .add_system(handle_mouse_input);
//     }
// }

// System to handle keyboard input using ButtonInput<KeyCode>
fn handle_keyboard_input(input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        println!("Space key just pressed!");
    }
    if input.pressed(KeyCode::Space) {
        println!("Space key is being held!");
    }
    if input.just_released(KeyCode::Space) {
        println!("Space key just released!");
    }
}

// System to handle mouse input using ButtonInput<MouseButton>
fn handle_mouse_input(input: Res<ButtonInput<MouseButton>>) {
    if input.just_pressed(MouseButton::Left) {
        println!("Left mouse button just pressed!");
    }
    if input.pressed(MouseButton::Left) {
        println!("Left mouse button is being held!");
    }
    if input.just_released(MouseButton::Left) {
        println!("Left mouse button just released!");
    }
}
