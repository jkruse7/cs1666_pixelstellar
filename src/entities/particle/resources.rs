use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::{entities::particle::components::*, LEVEL_W, LEVEL_H};

pub const PARTICLE_SIZE: f32 = 4.;
pub const MIN_X: i32 = ((-LEVEL_W / 2.) / PARTICLE_SIZE) as i32;
pub const MAX_X: i32 = ((LEVEL_W / 2.) / PARTICLE_SIZE) as i32;
pub const MIN_Y: i32 = ((-LEVEL_H / 2.) / PARTICLE_SIZE) as i32;
//pub const MAX_Y: i32 = ((LEVEL_H / 2.) / PARTICLE_SIZE) as i32;

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


    pub fn insert_at<P: NewParticle + Bundle>(&mut self, commands: &mut Commands, pos: (i32, i32)) {
        if self.get_element_at(pos) != ParticleElement::Air{
            self.delete_at(commands, pos);
        }
        let particle_instance = P::new(pos.0, pos.1);
        let entity = commands.spawn(particle_instance).id();
        let element = P::ELEMENT;
        self.particle_map.insert(pos, (entity, element));
        //let particle_type = ParticleType::from_element(P::ELEMENT);

        //self.particle_map.insert((x, y),);
    }

    pub fn delete_at(&mut self, commands: &mut Commands, pos: (i32, i32)){
        if let Some(old_entity) = self.particle_map.get(&pos).map(|(entity, _)| *entity) {
            commands.entity(old_entity).despawn();
            self.particle_map.remove(&pos);
        }
    }
    
    pub fn ray(&mut self, commands: &mut Commands, start: (i32, i32), end: (i32, i32)) -> Option<(i32, i32)> {
        let (mut x0, mut y0) = start;
        let (x1, y1) = end;
    
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
    
        while (x0, y0) != (x1, y1) {
            let element = self.get_element_at((x0, y0));
            if element != ParticleElement::Air { return Some((x0, y0)) }
    
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
    
        // you can return None if theres nothing between (x0,y0)->(x1,y1)
        // or just return the position (x1,y1) if there was nothing in between p0 and p1
        //Some(x1, y1)
        if self.get_element_at((x1, y1)) != ParticleElement::Air {
            Some((x1, y1))
        } else {
            None
        }
    }


    /*pub fn ray(&mut self){
        ;
    }*/

    /*
    pub fn get(&self, pos: (i32, i32)) -> ParticleElement {
        *self.particle_map.get(&(x, y)).unwrap_or(&ParticleElement::Air)
    }
    pub fn insert(&mut self, x: i32, y: i32, particle_type: ParticleElement) {
        self.particle_map.insert((x, y), particle_type);
    }
    pub fn move_data(&mut self, old: (i32, i32), new: (i32, i32), data: &ParticleElement) {
        self.particle_map.remove(&old);
        self.particle_map.insert(new, *data);
    }
    pub fn remove(&mut self, x: i32, y: i32) {
        self.particle_map.remove(&(x, y));
    }*/
}
