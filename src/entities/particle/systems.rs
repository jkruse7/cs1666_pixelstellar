use bevy::{prelude::*, ui::update};
use rand::Rng;
use super::{components::*, resources::*};
use crate::common::{gravity::{Gravity, GravityResource}, hitbox::Hitbox, perlin_noise::{generate_permutation_array, get_1d_octaves, get_2d_octaves}, state::{AppState, GamePhase}};
use crate::entities::player::components::Player;
use crate::{LEVEL_W, LEVEL_H};













// Update functions (cellular automata) make sure to update plugins at the bottom -------------------------------
fn update_water(
    mut map: ResMut<ParticleMap>,
    time: Res<Time>, 
    mut commands: Commands,
    mut particles: Query<&mut ParticlePosVel, With<ParticleTagWater>>,
    grav_res: ResMut<GravityResource>,
    player_hb_query: Query<& Hitbox, With<Player>>,
) {
    let deltat = time.delta_seconds() ;

    let player_hb: &Hitbox = player_hb_query.single();
    let mut rng = rand::thread_rng();
    for mut position in &mut particles {
        if position.velocity.x != 0. && position.velocity.y != 0.{
            let new_pos = ((position.grid_x as f32 + position.velocity.x) as i32, (position.grid_y as f32 + position.velocity.y) as i32);
            position.velocity.y = Gravity::update_gravity(&position.velocity.y, &deltat, &grav_res);

            let p = map.ray(&mut commands, (position.grid_x, position.grid_y), new_pos, ListType::OnlyAir);
            if let Some(position_of_part) = p {
                if position_of_part != new_pos{
                    position.velocity = Vec2::splat(0.);
                }
                map.delete_at(&mut commands, (position.grid_x, position.grid_y));
                map.insert_at::<WaterParticle>(&mut commands, position_of_part, ListType::OnlyAir);
                map.give_velocity(&mut commands, position_of_part, Vec2::new(position.velocity.x, position.velocity.y), );
            }
        } else {
            let mut rng = rand::thread_rng();
            let viscosity = rng.gen::<f64>() < WATER_VISCOSITY as f64;
                let (x, y) = (position.grid_x, position.grid_y);
                if viscosity && map.insert_at::<WaterParticle>(&mut commands, (x, y-1), ListType::OnlyAir) {
                    map.delete_at(&mut commands, (x, y));
                } else if viscosity && map.get_element_at((x,y-1)) == ParticleElement::Lava && !player_hb.is_particle_in_hitbox((x, y-1)) {
                    map.delete_at(&mut commands, (x, y));
                    map.delete_at(&mut commands, (x, y-1));
                    map.insert_at::<ObsidianParticle>(&mut commands, (x, y-1), ListType::OnlyAir);
                }

                else if viscosity && map.insert_at::<WaterParticle>(&mut commands, (x-1, y-1), ListType::OnlyAir){
                    map.delete_at(&mut commands, (x, y));
                } else if viscosity && map.get_element_at((x-1,y-1)) == ParticleElement::Lava && !player_hb.is_particle_in_hitbox((x-1, y-1)) {
                    map.delete_at(&mut commands, (x, y));
                    map.delete_at(&mut commands, (x-1, y-1));
                    map.insert_at::<ObsidianParticle>(&mut commands, (x-1, y-1), ListType::OnlyAir);
                }

                else if viscosity && map.insert_at::<WaterParticle>(&mut commands, (x+1, y-1), ListType::OnlyAir){
                    map.delete_at(&mut commands, (x, y));
                } else if viscosity && map.get_element_at((x+1,y-1)) == ParticleElement::Lava && !player_hb.is_particle_in_hitbox((x+1, y-1)) {
                    map.delete_at(&mut commands, (x, y));
                    map.delete_at(&mut commands, (x+1, y-1));
                    map.insert_at::<ObsidianParticle>(&mut commands, (x+1, y-1), ListType::OnlyAir);
                } 

                else if viscosity && map.insert_at::<WaterParticle>(&mut commands, (x-1, y), ListType::OnlyAir){
                    map.delete_at(&mut commands, (x, y));
                } else if viscosity && map.get_element_at((x-1, y)) == ParticleElement::Lava && !player_hb.is_particle_in_hitbox((x-1, y)) {
                    map.delete_at(&mut commands, (x, y));
                    map.delete_at(&mut commands, (x-1, y));
                    map.insert_at::<ObsidianParticle>(&mut commands, (x-1, y), ListType::OnlyAir);
                }
                 
                else if viscosity && map.insert_at::<WaterParticle>(&mut commands, (x+1, y), ListType::OnlyAir){
                    map.delete_at(&mut commands, (x, y));
                } else if viscosity && map.get_element_at((x+1, y)) == ParticleElement::Lava && !player_hb.is_particle_in_hitbox((x+1, y)){
                    map.delete_at(&mut commands, (x, y));
                    map.delete_at(&mut commands, (x+1, y));
                    map.insert_at::<ObsidianParticle>(&mut commands, (x+1, y), ListType::OnlyAir);
                }
        }
    }
}


fn update_lava(
    mut map: ResMut<ParticleMap>,
    time: Res<Time>, 
    mut commands: Commands,
    mut particles: Query<&mut ParticlePosVel, With<ParticleTagLava>>,
    player_hb_query: Query<& Hitbox, With<Player>>,
    grav_res: ResMut<GravityResource>,
) {
    let deltat = time.delta_seconds() ;
    
    let player_hb: &Hitbox = player_hb_query.single();
    let mut rng = rand::thread_rng();
    for mut position in &mut particles {
        if position.velocity.x != 0. && position.velocity.y != 0.{
            let new_pos = ((position.grid_x as f32 + position.velocity.x) as i32, (position.grid_y as f32 + position.velocity.y) as i32);
            position.velocity.y = Gravity::update_gravity(&position.velocity.y, &deltat, &grav_res);

            let p = map.ray(&mut commands, (position.grid_x, position.grid_y), new_pos, ListType::OnlyAir);
            if let Some(position_of_part) = p {
                if position_of_part != new_pos{
                    position.velocity = Vec2::splat(0.);
                }
                map.delete_at(&mut commands, (position.grid_x, position.grid_y));
                map.insert_at::<LavaParticle>(&mut commands, position_of_part, ListType::OnlyAir);
                map.give_velocity(&mut commands, position_of_part, Vec2::new(position.velocity.x, position.velocity.y), );
            }
        } else {
            let mut rng = rand::thread_rng();
            let viscosity = rng.gen::<f64>() < LAVA_VISCOSITY as f64;
                let (x, y) = (position.grid_x, position.grid_y);
                if viscosity && map.insert_at::<LavaParticle>(&mut commands, (x, y-1), ListType::OnlyAir) {
                    map.delete_at(&mut commands, (x, y));
                } else if viscosity && map.get_element_at((x, y-1)) == ParticleElement::Water && !player_hb.is_particle_in_hitbox((x, y-1)) {
                    map.delete_at(&mut commands, (x, y));
                    map.delete_at(&mut commands, (x, y-1));
                    map.insert_at::<ObsidianParticle>(&mut commands, (x, y-1), ListType::OnlyAir);
                }

                else if viscosity && map.insert_at::<LavaParticle>(&mut commands, (x-1, y-1), ListType::OnlyAir){
                    map.delete_at(&mut commands, (x, y));
                } else if viscosity && map.get_element_at((x-1, y-1)) == ParticleElement::Water && !player_hb.is_particle_in_hitbox((x-1, y-1)){
                    map.delete_at(&mut commands, (x, y));
                    map.delete_at(&mut commands, (x-1, y-1));
                    map.insert_at::<ObsidianParticle>(&mut commands, (x-1, y-1), ListType::OnlyAir);
                } 

                else if viscosity && map.insert_at::<LavaParticle>(&mut commands, (x+1, y-1), ListType::OnlyAir){
                    map.delete_at(&mut commands, (x, y));
                } else if viscosity && map.get_element_at((x+1, y-1)) == ParticleElement::Water && !player_hb.is_particle_in_hitbox((x+1, y-1)) {
                    map.delete_at(&mut commands, (x, y));
                    map.delete_at(&mut commands, (x+1, y-1));
                    map.insert_at::<ObsidianParticle>(&mut commands, (x+1, y-1), ListType::OnlyAir);
                } 

                else if viscosity && map.insert_at::<LavaParticle>(&mut commands, (x-1, y), ListType::OnlyAir){
                    map.delete_at(&mut commands, (x, y));
                } else if viscosity && map.get_element_at((x-1, y)) == ParticleElement::Water && !player_hb.is_particle_in_hitbox((x-1, y)){
                    map.delete_at(&mut commands, (x, y));
                    map.delete_at(&mut commands, (x-1, y));
                    map.insert_at::<ObsidianParticle>(&mut commands, (x-1, y), ListType::OnlyAir);
                }

                else if viscosity && map.insert_at::<LavaParticle>(&mut commands, (x+1, y), ListType::OnlyAir){
                    map.delete_at(&mut commands, (x, y));
                } else if viscosity && map.get_element_at((x+1, y)) == ParticleElement::Water && !player_hb.is_particle_in_hitbox((x+1, y)) {
                    map.delete_at(&mut commands, (x, y));
                    map.delete_at(&mut commands, (x+1, y));
                    map.insert_at::<ObsidianParticle>(&mut commands, (x+1, y), ListType::OnlyAir);
                }
        }
    }
}

fn update_gas(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
    mut particles: Query<&mut ParticlePosVel, With<ParticleTagGas>>,
) {
    for mut position in &mut particles {
        let mut rng = rand::thread_rng();
        let move_r = 2;
        let decay_rate = 50;
        // This decay logic just says if the positions 10 away in each cardinal direction is air theres a small chance to despawn.
        // just means that 
        //               1 to disable or just comment out
        if rng.gen_range(0..decay_rate) == 0 {
            if map.get_element_at((position.grid_x+10, position.grid_y)) == ParticleElement::Air &&
               map.get_element_at((position.grid_x, position.grid_y+10)) == ParticleElement::Air &&
               map.get_element_at((position.grid_x-10, position.grid_y)) == ParticleElement::Air &&
               map.get_element_at((position.grid_x, position.grid_y-10)) == ParticleElement::Air {
                map.delete_at(&mut commands, (position.grid_x, position.grid_y));
               }
        } else if rng.gen_range(0..=1) == 0{
            let radius: i32 = rng.gen_range(1..=6);
            let center_x = position.grid_x;
            let center_y = position.grid_y;
            

            // We can use this to give certain elements different densities. some may float very quickly, some may disperse more, etc.
            //70% chance to pick an upward angle, 30% chance for any angle
            let angle = if rng.gen_bool(0.2) {
                // Bias towards an upward angle (between π/4 and 3π/4)
                rng.gen_range(std::f32::consts::FRAC_PI_4..=3.0*std::f32::consts::FRAC_PI_4)
            } else {
                rng.gen_range(0.0..=2.0 * std::f32::consts::PI)
            };
            // or just be random.
            //let angle = rng.gen_range(0.0..=2.0 * std::f32::consts::PI);

            let dx = (radius as f32 * angle.cos()).round() as i32;
            let dy = (radius as f32 * angle.sin()).round() as i32;
            let new_pos = (center_x + dx, center_y + dy);
            
            if let Some(position_of_part) = map.ray(&mut commands, (center_x, center_y), new_pos, ListType::Whitelist(vec!(ParticleElement::Gas, ParticleElement::Air))) {
                if map.get_element_at(position_of_part) == ParticleElement::Air {
                    map.delete_at(&mut commands, (center_x, center_y));
                    // Check that the new coordinates are within bounds before spawning
                    if grid_coords_within_map(position_of_part) {
                        map.insert_at::<GasParticle>(&mut commands, position_of_part, ListType::OnlyAir);
                    }
                }
            }
        }
    }
}








// Player interaction functions -------------------------------------------------------------------------------
pub fn build_or_destroy(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    let (l, r) = (buttons.pressed(MouseButton::Left), buttons.pressed(MouseButton::Right));

    if l || r{
        if let Some(world_position) = 
            window.cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
        {
            let size = 2.;
            let mut y: f32 = -size * PARTICLE_SIZE;
            while y < size * PARTICLE_SIZE + 0.1{
                let mut x: f32 = -size * PARTICLE_SIZE;
                while x < size * PARTICLE_SIZE + 0.1{
                    let position = (((world_position.x+x) / PARTICLE_SIZE) as i32, ((world_position.y+y) / PARTICLE_SIZE) as i32);
                    if l {
                        map.delete_at(&mut commands, (((world_position.x+x) / PARTICLE_SIZE) as i32, ((world_position.y+y) / PARTICLE_SIZE) as i32));
                    }
                    if r{
                        map.insert_at::<GasParticle>(&mut commands, (position.0, position.1), ListType::Blacklist(vec![ParticleElement::Air, ParticleElement::Stone]));
                    }
                    x += PARTICLE_SIZE;
                }
                y += PARTICLE_SIZE;
            }
        }
    }
}

pub fn paint_with_ray(
    mut map: ResMut<ParticleMap>,
    mut commands: Commands,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    buttons: Res<ButtonInput<MouseButton>>,
    player: Query<&Transform, With<Player>>
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    let pt = player.single();

    if buttons.pressed(MouseButton::Left){
        if let Some(world_position) = 
            window.cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
        {
            
            let p = map.ray(&mut commands, 
                convert_to_grid_position(pt.translation.x , pt.translation.y), 
                convert_to_grid_position(world_position.x, world_position.y), 
                ListType::Whitelist(vec!(ParticleElement::Dirt)));
                // a little weird here ^ might need to experiment w white/black lists to see exactly how they work
            if let Some(position_of_part) = p {
                map.insert_at::<BedRockParticle>(&mut commands, position_of_part, ListType::All);
            }
        }
    }
}

pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        // Startup placements
        app.insert_resource(ParticleMap::new());

        // Updates i.e. all automata goes here
        //app.add_systems(Update, draw_rain);
        app.add_systems(Update, update_water.after(crate::entities::player::blaster::systems::shoot_blaster)
                        .run_if(in_state(AppState::InGame)));
        app.add_systems(Update, update_gas
                        .run_if(in_state(AppState::InGame)));
        app.add_systems(Update, update_lava.after(update_water)
                        .run_if(in_state(AppState::InGame)));
        //app.add_systems(Update, paint_with_ray.after(update_water));
        //app.add_systems(Update, build_or_destroy.after(update_water));
    }
} 



fn handle_chunks(
    mut chunks: ResMut<ChunkList>,
    mut particles: ResMut<ParticleMap>,
    mut commands: Commands,
    player_transform: Query<&Transform, With<Player>>,
) {
    let pt = player_transform.single().translation;
    let position = ((pt.x / PARTICLE_SIZE).floor() as i32, (pt.y / PARTICLE_SIZE).floor() as i32);

    let new_chunks = chunks.load(position);

    let old_chunks = chunks.unload(position);
    for chunk in old_chunks {
        particles.despawn_chunk(&mut commands, chunk);
    }
}

pub struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkList::new());
        app.add_systems(Update, handle_chunks.run_if(in_state(AppState::InGame)));
    }
} 