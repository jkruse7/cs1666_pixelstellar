use bevy::prelude::*;
#[derive(Component)]
pub struct ButtonColors { 
    pub normal: Color,   
    pub hovered: Color,   
    pub pressed: Color,   
}
// Define default button colors for different states
const DEFAULT_NORMAL_COLOR: Color = Color::srgba(0.15, 0.15, 0.15, 1.0);  // Default normal state color
const DEFAULT_HOVERED_COLOR: Color = Color::srgba(0.25, 0.25, 0.25, 1.0); // Default hovered state color
const DEFAULT_PRESSED_COLOR: Color = Color::srgba(0.35, 0.75, 0.35, 1.0); // Default pressed state color

/// Function to spawn a button with customizable colors and opacity
pub fn spawn_custom_button(
    commands: &mut Commands,
    text: &str,                  // Button label text
    size: Vec2,                  // Button size
    position: Option<Vec2>,            // Button position
    font: Handle<Font>,          // Font handle for button text
    normal_color: Option<Color>, // Customizable normal state color
    hovered_color: Option<Color>, // Customizable hovered state color
    pressed_color: Option<Color>, // Customizable pressed state color
    parent: Option<Entity>,      // Optional parent entity
) -> Entity {
    // Use provided colors or fallback to defaults
    let normal_color = normal_color.unwrap_or(DEFAULT_NORMAL_COLOR);
    let hovered_color = hovered_color.unwrap_or(DEFAULT_HOVERED_COLOR);
    let pressed_color = pressed_color.unwrap_or(DEFAULT_PRESSED_COLOR);


    const WIN_W: f32 = 1280.;
    const WIN_H: f32 = 720.;

    let position_offset = position.unwrap_or(Vec2::ZERO);
    let screen_center_x = (WIN_W - size.x) / 2.0;
    let screen_center_y = (WIN_H - size.y) / 2.0;

    let left = Val::Px(screen_center_x + position_offset.x);
    let top = Val::Px(screen_center_y + position_offset.y);

    // Define the button's text style
    let button_text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::srgba(0.9, 0.9, 0.9, 1.0), // Text color
    };

    // Create the button entity
    let button_entity = commands.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            position_type: PositionType::Absolute, // Absolute positioning
            left,
            top, // Calculated position relative to screen center
            justify_content: JustifyContent::Center, // Center content
            align_items: AlignItems::Center, // Center items
            ..default()
        },
        background_color: normal_color.into(), // Set the normal background color
        ..default()
    })
    .insert(ButtonColors {
        normal: normal_color,
        hovered: hovered_color,
        pressed: pressed_color,
    }) // Insert custom color configuration
    .id();

    // Add the text entity as a child of the button
    commands.entity(button_entity).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            button_text_style,
        ));
    });

    // Attach button to parent if provided
    if let Some(parent_entity) = parent {
        commands.entity(parent_entity).add_child(button_entity);
    }

    button_entity
}

// Struct to store the button's color configurations


// System to handle button interactions and color changes
pub fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColors),
        (Changed<Interaction>, With<Button>)
    >,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = button_colors.pressed.into(); // Use the pressed color
                println!("Button pressed!");
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into(); // Use the hovered color
            }
            Interaction::None => {
                *color = button_colors.normal.into(); // Use the normal color
            }
        }
    }
}

