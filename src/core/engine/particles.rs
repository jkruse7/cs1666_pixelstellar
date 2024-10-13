use bevy::prelude::*;
use crate::core::gameplay::player;

use super::{gravity::{self, Gravity}, hitbox::Hitbox};

// 10/1 Julianne: base struct for all elements
// This should include all the componenets that will 
// be needed to create and work with different elements

// TODO: remove bedrock from having to be iterated over on outer loop
// TODO: add visual representation of particles

const PARTICLE_SIZE: f32 = 4.;
enum MatterState {
    LIQUID,
    GAS, 
    SOLID,
}

pub enum ELEMENT {
    WATER, BEDROCK,
}
impl ELEMENT {
    pub fn color(&self) -> Color {
        match self {
            ELEMENT::WATER => Color::srgb(0.0, 0.0, 1.0), // Blue for water
            ELEMENT::BEDROCK => Color::srgb(0.5, 0.5, 0.5), // Gray for bedrock
        }
    }
    pub fn state(&self) -> MatterState {
        match self {
            ELEMENT::WATER => MatterState::LIQUID,
            ELEMENT::BEDROCK => MatterState::SOLID,
        }
    }
}

#[derive(Component)]
pub struct Particle {
    has_gravity: bool,
    state: MatterState,
    element: ELEMENT,
    collision: bool,
    iterate_for_collision: bool,
    hitbox: Hitbox, 
    velocity: Vec2,
    gravity: Gravity,
    transform: Transform,
}
impl Default for Particle {
    fn default() -> Self {
        Particle {
            has_gravity: true,
            element: ELEMENT::BEDROCK,
            state: ELEMENT::BEDROCK.state(),
            collision: true,
            iterate_for_collision: true,
            hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE, Vec2::ZERO),
            velocity: Vec2::ZERO,
            gravity: Gravity::new(
            ),
            transform: Transform::from_translation(Vec3::ZERO),
        }
    }
}

impl Particle {

    pub fn new(
        has_gravity: bool,
        element: ELEMENT,
        collision: bool,
        iterate_for_collision: bool,
        velocity: Vec2,
        transform: Transform,
    ) -> Self {
        Particle {
            has_gravity,
            state: element.state(),
            element,
            collision,
            iterate_for_collision,
            hitbox: Hitbox::new(PARTICLE_SIZE, PARTICLE_SIZE, transform.translation.truncate()),
            velocity,
            gravity: Gravity::new_with_g(velocity.y),
            transform
        }
    }

    pub fn spawn_particle(
        commands: &mut Commands,
        particle: Particle,  // Particle instance passed in
    ) {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: particle.element.color(),  // Use the color of the element
                    custom_size: Some(Vec2::splat(PARTICLE_SIZE)),  // Set size of the sprite
                    ..Default::default()
                },
                transform: particle.transform,  // Use the transform provided by the particle
                ..Default::default()
            },
            particle.hitbox.clone(),  // Insert the Hitbox component itself
            particle,  // Insert the Particle component itself

        ));
    }

    pub fn move_and_handle_collisions(
        time: Res<Time>,
        mut parts: Query<(Entity, &mut Particle, &mut Hitbox, &mut Transform), Without<player::Player>>,
        mut player_hitboxes: Query<(&Hitbox, &player::Player)>, //player
    ) {
        let deltat = time.delta_seconds();

        // First pass: update velocities and propose new positions
        let mut proposed_movements = Vec::new();

        for (entity, mut particle, mut hitbox, mut Transform) in parts.iter_mut() {
            // Apply gravity if necessary
            if particle.has_gravity {
                let velocity_y = particle.velocity.y;
                particle.gravity.update_g(&velocity_y, &deltat);
                particle.velocity.y = particle.gravity.get_g();
            }

            let proposed_offset = hitbox.offset + particle.velocity * deltat;
            //info!("Proposed offset: {:?}", proposed_offset);
            proposed_movements.push((entity, particle.velocity, proposed_offset));
        }

        // Second pass: check for collisions and finalize movement
        for (entity, velocity, proposed_offset) in proposed_movements.iter() {
            let mut collides = false;

            // Check for collisions with other particles
            for (other_entity, _other_particle, other_hitbox, other_transform) in parts.iter_mut() {
                if *entity == other_entity {
                    continue; // Skip self
                }
                info!("Checking collision between {:?} and hb offset{:?}", entity, other_hitbox.offset);
                // If proposed movement collides with another hitbox, adjust velocity
                if Hitbox::new(PARTICLE_SIZE,PARTICLE_SIZE, *proposed_offset).collides_with(&other_hitbox) {
                    collides = true;
                    break;
                }
            }
            // Check for collisions with player
            for (player_hitbox, _player) in player_hitboxes.iter() {
                if Hitbox::new(PARTICLE_SIZE,PARTICLE_SIZE, *proposed_offset).collides_with(&player_hitbox) {
                    collides = true;
                    break;
                }
            }

            // Finalize movement based on collision check
            // Find the particle and its hit
            if !collides {
                for (ent, part, mut hb, mut tr) in parts.iter_mut() {
                    if ent == *entity {
                        hb.offset = *proposed_offset; // No collision, apply movement
                        tr.translation = Vec3::new(hb.offset.x, hb.offset.y, 0.); // Update transform
                    }
                }
            } else {
                info!("Collision detected for entity {:?}", entity);
                // Handle collision (right now just stop the particle)
                for (ent, mut part, hb, tr) in parts.iter_mut() {
                    if ent == *entity {
                        part.velocity = Vec2::ZERO; 
                    }
                }
            }
        }
    }
}
pub fn test_particle_spawn(
    mut commands: Commands,
) {
    let particle = Particle::new(
        true,
        ELEMENT::WATER,
        true,
        true,
        Vec2::new(0., 0.),
        Transform::from_translation(Vec3::new(0., 200., 0.)),
    );
    Particle::spawn_particle(&mut commands, particle);

    let particle = Particle::new(
        true,
        ELEMENT::WATER,
        true,
        true,
        Vec2::new(240., 100.),
        Transform::from_translation(Vec3::new(0., -100., 0.)),
    );
    Particle::spawn_particle(&mut commands, particle);

    let bedrock = Particle::new(
        false,
        ELEMENT::BEDROCK,
        true,
        true,
        Vec2::new(0., 0.),
        Transform::from_translation(Vec3::new(0., -120., 0.)),
    );
    info!("bedrock hb offset: {:?}", bedrock.hitbox.offset);
    Particle::spawn_particle(&mut commands, bedrock);
}