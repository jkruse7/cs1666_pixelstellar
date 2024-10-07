use bevy::prelude::*;
use super::{gravity::{self, Gravity}, hitbox::Hitbox};
use std::collections::HashMap;

// 10/1 Julianne: base struct for all elements
// This should include all the componenets that will 
// be needed to create and work with different elements

// TODO: remove bedrock from having to be iterated over on outer loop
// TODO: add visual representation of particles

const PARTICLE_SIZE: f32 = 4.;
enum MATTER_STATE {
    LIQUID,
    GAS, 
    SOLID,
}

pub enum ELEMENT {
    WATER, BEDROCK,
}

#[derive(Component)]
pub struct Particle {
    has_gravity: bool,
    state: MATTER_STATE,
    element: ELEMENT,
    collision: bool,
    iterate_for_collision: bool,
    hitbox: Hitbox, 
    velocity: Vec2,
    gravity: Gravity,
}

impl Particle {
    pub fn new_liquid(offset: Vec2, velocity: Vec2, element: ELEMENT) -> Self {
        Particle {
            has_gravity: true,
            state: MATTER_STATE::LIQUID,
            element,
            collision: true,
            iterate_for_collision: true,
            hitbox: Hitbox::new(PARTICLE_SIZE,PARTICLE_SIZE, offset),
            velocity,
            gravity: Gravity::new(),
        }
    }



    pub fn new_bedrock(offset: Vec2) -> Self {
        Particle {
            has_gravity: false,
            state: MATTER_STATE::SOLID,
            element: ELEMENT::BEDROCK,
            collision: false,
            iterate_for_collision: true,
            hitbox: Hitbox::new(PARTICLE_SIZE,PARTICLE_SIZE, offset),
            velocity: Vec2::splat(0.),
            gravity: Gravity::new(),
        }
    }

    pub fn move_and_handle_collisions(
        time: Res<Time>,
        mut parts: Query<(Entity, &mut Particle, &mut Hitbox)>, //particles
    ) {
        let deltat = time.delta_seconds();

        // First pass: update velocities and propose new positions
        let mut proposed_movements = Vec::new();

        for (entity, mut particle, mut hitbox) in parts.iter_mut() {
            // Apply gravity if necessary
            if particle.has_gravity {
                let velocity_y = particle.velocity.y;
                particle.gravity.update_G(&velocity_y, &deltat);
                particle.velocity.y = particle.gravity.get_G();
            }

            let proposed_offset = hitbox.offset + particle.velocity * deltat;

            proposed_movements.push((entity, particle.velocity, proposed_offset));
        }

        // Second pass: check for collisions and finalize movement
        for (entity, velocity, proposed_offset) in proposed_movements.iter() {
            let mut collides = false;

            // Check for collisions with other particles
            for (other_entity, _other_particle, other_hitbox) in parts.iter_mut() {
                if *entity == other_entity {
                    continue; // Skip self
                }

                // If proposed movement collides with another hitbox, adjust velocity
                if Hitbox::new(PARTICLE_SIZE,PARTICLE_SIZE, *proposed_offset).collides_with(&other_hitbox) {
                    collides = true;
                    break;
                }
            }

            // Finalize movement based on collision check
            // Find the particle and its hit
            if !collides {
                for (ent, part, mut hb) in parts.iter_mut() {
                    if ent == *entity {
                        hb.offset = *proposed_offset; // No collision, apply movement
                    }
                }
            } else {
                // Handle collision (right now just stop the particle)
                for (ent, mut part, hb) in parts.iter_mut() {
                    if ent == *entity {
                        part.velocity = Vec2::ZERO; 
                    }
                }
            }
        }
    }
}