use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::{entities::particle::components::*, LEVEL_W, LEVEL_H};

pub const PARTICLE_SIZE: f32 = 4.;

pub enum ListType {
    ReplaceAll,
    ReplaceOnlyAir,
    Whitelist(Vec<ParticleElement>),  // Only replace specific particle types
    Blacklist(Vec<ParticleElement>),  // Replace any particle except those specified
}

pub const MIN_X: i32 = ((-LEVEL_W / 2.) / PARTICLE_SIZE) as i32;
pub const MAX_X: i32 = ((LEVEL_W / 2.) / PARTICLE_SIZE) as i32;
pub const MIN_Y: i32 = ((-LEVEL_H / 2.) / PARTICLE_SIZE) as i32;
pub const MAX_Y: i32 = ((LEVEL_H / 2.) / PARTICLE_SIZE) as i32;

#[derive(Resource)]
pub struct ParticleMap {
    pub particle_map: HashMap<(i32, i32), (Entity, ParticleElement)>,
}

impl ParticleMap {
    pub fn new() -> Self {
        Self {
            particle_map: HashMap::new(),
        }
    }

    pub fn get_element_at(&self, pos: (i32, i32)) -> ParticleElement {
        self.particle_map.get(&pos)
            .map(|(_, particle_element)| *particle_element)
            .unwrap_or(ParticleElement::Air)
    }


    /* useful for anyone implementing other functions here: you can get the actual Particle or whatever values based off of Entity. */
    pub fn get_entity_at(&self, pos: (i32, i32)) -> Option<Entity> {
        self.particle_map.get(&pos).map(|(entity, _)| *entity)
    }
    /* for example was useful here. */
    pub fn give_velocity(&mut self, commands: &mut Commands, pos: (i32, i32), vel: Vec2) {
        if let Some(entity) = self.get_entity_at(pos) {
            // .insert replaces the component/bundle/whateveritscalled with the new one
            commands.entity(entity).insert(ParticlePosVel { grid_x: pos.0, grid_y: pos.1, velocity: vel });
        }
    }


    /* Usage: 
        Replace regardless of whats there:
            particle_map.insert_at::<WaterParticle>(&mut commands, (x, y), ListType::ReplaceAll);
        Replace only if air is there:
            particle_map.insert_at::<WaterParticle>(&mut commands, (x, y), ListType::ReplaceOnlyAir);
        Replace only if ParticleElement is in list (whitelist). i.e. will only replace air and stone:
            particle_map.insert_at::<WaterParticle>(&mut commands, (x, y), ListType::WhiteList(vec![ParticleElement::Air, ParticleElement::Stone]))
        Replace if particleElement is NOT in list (blacklist). i.e. will replace everything except for air and stone:
            particle_map.insert_at::<WaterParticle>(&mut commands, (x, y), ListType::Blacklist(vec![ParticleElement::Air, ParticleElement::Stone]))
    */
    pub fn insert_at<P: NewParticle + Bundle>(&mut self, commands: &mut Commands, pos: (i32, i32), list_type: ListType) -> bool {
        let element_at_pos = self.get_element_at(pos);
    
        
        let should_replace = match list_type {
            ListType::ReplaceAll => true,
            ListType::ReplaceOnlyAir => element_at_pos == ParticleElement::Air,
            ListType::Whitelist(ref whitelist) => whitelist.contains(&element_at_pos),
            ListType::Blacklist(ref blacklist) => !blacklist.contains(&element_at_pos),
        };
    
        if should_replace{
            // If we need to replace a non-air element, delete it
            if element_at_pos != ParticleElement::Air {
                self.delete_at(commands, pos);
            }
            let particle_instance = P::new(pos.0, pos.1, Vec2::splat(0.));
            let entity = commands.spawn(particle_instance).id();
            let element = P::ELEMENT;
            self.particle_map.insert(pos, (entity, element));
            return true;
        }
        
        false
    }


    /* Usage: particle_map.delete(&mut commands, (x,y));
            Will delete the particle from a pos if there is something there.
            if its air it doesnt do anything since no particle exists there.
     */
    pub fn delete_at(&mut self, commands: &mut Commands, pos: (i32, i32)){
        if let Some(old_entity) = self.particle_map.get(&pos).map(|(entity, _)| *entity) {
            commands.entity(old_entity).despawn();
            self.particle_map.remove(&pos);
        }
    }
    
    /* Returns the first position between start (x0, y0) and end (x1, y1) that is not in ignore. For example

          start          1            2            3             4            5            6           end
        y   +------------+------------+------------+-----------Stone----------+------------+------------+

            ray(&mut commands, (0,y), (7,y), &[])                               will return (3, y).
            ray(&mut commands, (0,y), (7,y), &[ParticleElement::Stone])         will return (7, y).

            this makes it possible to move a particle from start to end, and getting the first position hit, move to that location.
            you can check that ray(...) != end_position if you just want to detect that nothing was hit
    */
    pub fn ray(&mut self, commands: &mut Commands, start: (i32, i32), end: (i32, i32), ignore: &[ParticleElement]) -> Option<(i32, i32)> {
        let (mut x0, mut y0) = start;
        let (x1, y1) = end;
    
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        
        let mut previous = (x0, y0);

        let e2 = 2 * err;
        // Offset first position
        if e2 > -dy { err -= dy; x0 += sx; }
        if e2 < dx { err += dx; y0 += sy; }

        while (x0, y0) != (x1, y1) {
            let element = self.get_element_at((x0, y0));
            if element != ParticleElement::Air && !ignore.contains(&element){ 
                // returns the spot before a hit
                return Some(previous)
                // returns the location it hit first
                //return Some((x0, y0)) 
            }
    
            previous = (x0, y0);
            // Bresenham's line algorithm step
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
        Some((x1, y1))
    }
}



// Utility functions.
pub fn convert_to_grid_position(x: f32, y: f32) -> (i32, i32) {
    let x = (x / PARTICLE_SIZE).round() as i32;
    let y = (y / PARTICLE_SIZE).round() as i32;
    (x, y)
}


pub fn grid_coords_within_map(pos: (i32, i32)) -> bool {
    let x = pos.0 as f32 * PARTICLE_SIZE;
    let y = pos.1 as f32 * PARTICLE_SIZE;
    if (x > -(LEVEL_W / 2.)) && (x < (LEVEL_W / 2.)) && (y > -(LEVEL_H / 2.)) && (y < (LEVEL_H / 2.)){
        return true
    }
    false
}

