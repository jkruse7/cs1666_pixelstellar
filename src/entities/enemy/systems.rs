use std::mem::take;

use bevy::{asset::io::embedded, prelude::*, scene::ron::de};
use crate::{
    common::{
        death::Death, gravity::{Gravity, GravityResource}, hitbox::Hitbox, state::{AppState, GamePhase}
    },
    entities::{
        particle::resources::ParticleMap,
        player::{components::{AnimationFrameCount, AnimationTimer, Health, Player}, resources::PlayerSoundTracker, systems::take_damage},
    },
    LEVEL_H,
    LEVEL_W,
    WIN_W,
};
use super::{
    components::*,
    resources::*,
};


pub fn initialize(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    state: Res<State<GamePhase>>,
) {
    let game_state =  state.get();
    if *game_state == GamePhase::Planet1 {
        let enemy_sheet_handle = asset_server.load("enemy_walking.png");
        //             used to be tilesize. removed TILE_SIZE and now at 100, but change as needed  \/
        let enemy_layout = TextureAtlasLayout::from_grid(UVec2::splat(100), 4, 1, None, None);
        let enemy_layout_len = enemy_layout.textures.len();
        let enemy_layout_handle = texture_atlases.add(enemy_layout);
        commands.spawn((
            SpriteBundle {
                texture: enemy_sheet_handle,
                transform: Transform {
                    // Julianne 10/8: For now, enemy is being spawned at WIN_W. This will need to be changed eventually.
                    translation: Vec3::new(WIN_W / 2., 100.0, 900.),
                    ..default()
                },
                sprite: Sprite {
                    flip_x: false,
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: enemy_layout_handle,
                index: 0,
            },
            AnimationTimer(Timer::from_seconds(W1_ANIM_TIME, TimerMode::Repeating)),
            AnimationFrameCount(enemy_layout_len),
            Velocity::new(),
            EnemyHealth::new(2.),
            Gravity::new(),
            Hitbox::new(40 as f32, 40 as f32, Vec2::new(0., -210.)),
            DamageBox::new(50.0, 50.0, Vec2::new(0., -210.)),
            Jump::new(),  
            Enemy,
        ));
    } else if *game_state == GamePhase::Planet2 {
        let enemy_sheet_handle = asset_server.load("planet_2/ice_cream.png");
        //             used to be tilesize. removed TILE_SIZE and now at 100, but change as needed  \/
        let enemy_layout = TextureAtlasLayout::from_grid(UVec2::new(W3_SPRITE_WIDTH, W3_SPRITE_HEIGHT), 1, 1, None, None);
        let enemy_layout_len = enemy_layout.textures.len();
        let enemy_layout_handle = texture_atlases.add(enemy_layout);
        commands.spawn((
            SpriteBundle {
                texture: enemy_sheet_handle,
                transform: Transform {
                    // Julianne 10/8: For now, enemy is being spawned at WIN_W. This will need to be changed eventually.
                    translation: Vec3::new(WIN_W / 2., 100.0, 900.),
                    ..default()
                },
                sprite: Sprite {
                    flip_x: false,
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: enemy_layout_handle,
                index: 0,
            },
            AnimationTimer(Timer::from_seconds(W1_ANIM_TIME, TimerMode::Repeating)),
            AnimationFrameCount(enemy_layout_len),
            Velocity::new(),
            EnemyHealth::new(4.),
            Gravity::new(),
            Hitbox::new(W3_SPRITE_WIDTH as f32, W3_SPRITE_HEIGHT as f32, Vec2::new(0., -210.)),
            DamageBox::new(W3_SPRITE_WIDTH as f32, W3_SPRITE_HEIGHT as f32, Vec2::new(0., -210.)),
            Jump::new(),  
            Enemy,
        ));

    } else if *game_state == GamePhase::Planet3 {
        let enemy_sheet_handle = asset_server.load("planet_3/ghost.png");
        //             used to be tilesize. removed TILE_SIZE and now at 100, but change as needed  \/
        let enemy_layout = TextureAtlasLayout::from_grid(UVec2::new(W3_SPRITE_WIDTH, W3_SPRITE_HEIGHT), 1, 1, None, None);
        let enemy_layout_len = enemy_layout.textures.len();
        let enemy_layout_handle = texture_atlases.add(enemy_layout);
        commands.spawn((
            SpriteBundle {
                texture: enemy_sheet_handle,
                transform: Transform {
                    // Julianne 10/8: For now, enemy is being spawned at WIN_W. This will need to be changed eventually.
                    translation: Vec3::new(WIN_W / 2., 100.0, 900.),
                    ..default()
                },
                sprite: Sprite {
                    flip_x: false,
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: enemy_layout_handle,
                index: 0,
            },
            AnimationTimer(Timer::from_seconds(W1_ANIM_TIME, TimerMode::Repeating)),
            AnimationFrameCount(enemy_layout_len),
            Velocity::new(),
            EnemyHealth::new(4.),
            Gravity::new(),
            Hitbox::new(W3_SPRITE_WIDTH as f32, W3_SPRITE_HEIGHT as f32, Vec2::new(0., -210.)),
            DamageBox::new(W3_SPRITE_WIDTH as f32, W3_SPRITE_HEIGHT as f32, Vec2::new(0., -210.)),
            Jump::new(),  
            Enemy,
        ));
    } else if *game_state == GamePhase::Planet4 {

    } else if *game_state == GamePhase::Planet5 {
         //let enemy_sheet_handle = asset_server.load("enemy_walking.png");
        let enemy_sheet_handle = asset_server.load("planet_5/enemy_walking.png");
        //             used to be tilesize. removed TILE_SIZE and now at 100, but change as needed  \/
        let enemy_layout = TextureAtlasLayout::from_grid(UVec2::splat(100), 4, 1, None, None);
        let enemy_layout_len = enemy_layout.textures.len();
        let enemy_layout_handle = texture_atlases.add(enemy_layout);
        commands.spawn((
            SpriteBundle {
                texture: enemy_sheet_handle,
                transform: Transform {
                    // Julianne 10/8: For now, enemy is being spawned at WIN_W. This will need to be changed eventually.
                    translation: Vec3::new(WIN_W / 2., 100.0, 900.),
                    ..default()
                },
                sprite: Sprite {
                    flip_x: false,
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: enemy_layout_handle,
                index: 0,
            },
            AnimationTimer(Timer::from_seconds(W1_ANIM_TIME, TimerMode::Repeating)),
            AnimationFrameCount(enemy_layout_len),
            Velocity::new(),
            EnemyHealth::new(5.),
            Gravity::new(),
            Hitbox::new(40 as f32, 40 as f32, Vec2::new(0., -210.)),
            DamageBox::new(50.0, 50.0, Vec2::new(0., -210.)),
            Jump::new(),  
            Enemy,
        ));
    } else if *game_state == GamePhase::Planet6 {

    } else if *game_state == GamePhase::Planet7 {
        

    } else if *game_state == GamePhase::Planet8 {
        let enemy_sheet_handle = asset_server.load("planet_8/sun.png");
        //             used to be tilesize. removed TILE_SIZE and now at 100, but change as needed  \/
        let enemy_layout = TextureAtlasLayout::from_grid(UVec2::new(W8_SPRITE_WIDTH, W8_SPRITE_HEIGHT), 1, 1, None, None);
        let enemy_layout_len = enemy_layout.textures.len();
        let enemy_layout_handle = texture_atlases.add(enemy_layout);
        commands.spawn((
            SpriteBundle {
                texture: enemy_sheet_handle,
                transform: Transform {
                    // Julianne 10/8: For now, enemy is being spawned at WIN_W. This will need to be changed eventually.
                    translation: Vec3::new(WIN_W / 3., 300.0, 900.),
                    ..default()
                },
                sprite: Sprite {
                    flip_x: false,
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: enemy_layout_handle,
                index: 0,
            },
            AnimationTimer(Timer::from_seconds(W1_ANIM_TIME, TimerMode::Repeating)),
            AnimationFrameCount(enemy_layout_len),
            Velocity::new(),
            EnemyHealth::new(20.),
            Gravity::new(),
            Hitbox::new(W8_SPRITE_WIDTH as f32, W8_SPRITE_HEIGHT as f32, Vec2::new(0., -210.)),
            DamageBox::new(W8_SPRITE_WIDTH as f32, W8_SPRITE_HEIGHT as f32, Vec2::new(0., -210.)),
            Jump::new(),  
            Enemy,
        ));
    }
}

#[derive(Component, Clone, Debug)]
pub struct DamageBox {
    pub width: f32,
    pub height: f32,
    pub offset: Vec2,
}

impl DamageBox {
    pub fn new(width: f32, height: f32, offset: Vec2) -> Self {
        Self { width, height, offset }
    }

    pub fn collides_with(&self, other: &Hitbox, self_pos: Vec2, other_pos: Vec2) -> bool {
        let self_tr = self_pos + self.offset + Vec2::new(self.width, self.height);
        let other_tr = other_pos + other.offset + Vec2::new(other.width, other.height);
        
        self_pos.x < other_tr.x && self_tr.x > other_pos.x && self_pos.y < other_tr.y && self_tr.y > other_pos.y
    }
}

pub fn enemy_gravity(
    time: Res<Time>, 
    mut enemy: Query<(&mut Transform, &mut Velocity, &mut Gravity, &mut Hitbox, &mut Jump), With<Enemy>>, 
    hitboxes: Query<&Hitbox, Without<Enemy>>,
    grav_res: ResMut<GravityResource>,
    state: Res<State<GamePhase>>,
) {
    let game_state =  state.get();
    if *game_state == GamePhase::Planet1 {
        /*Julianne 10/8: This function is the same as player flight, but only makes the downward force on the enemy (no flight)*/
        for (mut pt, mut pv, mut pg, mut hb, mut e_jump) in &mut enemy{

            let deltat = time.delta_seconds();

            //update gravity here
            if e_jump.needs_jump && !e_jump.jumped{
                pg.reset_g();
                let acc_y = W1_ACCEL_RATE_Y * deltat;
                pv.velocity.y = f32::min(250., pv.velocity.y + (1. * acc_y));
                e_jump.needs_jump = false;
                e_jump.is_jumping = true;
            }else {
                pg.update_g(&pv.velocity.y, &deltat, &grav_res);
                pv.velocity.y = pg.get_g();
            }
            

            let change = pv.velocity * deltat;
            let new_pos = pt.translation + change.extend(0.);
            let new_hb = Hitbox::new(W1_SPRITE_WIDTH as f32, W1_SPRITE_HEIGHT as f32, new_pos.xy());
            //Bound enemy to within level height
            if new_pos.y >= -(LEVEL_H / 2.) + (W1_SPRITE_HEIGHT as f32) / 2.
                && new_pos.y <= LEVEL_H - (W1_SPRITE_HEIGHT as f32) / 2.
                && (!new_hb.all_enemy_collisions(&hitboxes)) && !e_jump.jumped
            {    

                    pt.translation = new_pos;
                    *hb = new_hb; 
                    e_jump.jumped = true;
            }  
            let new_hb = Hitbox::new(W1_SPRITE_WIDTH as f32, W1_SPRITE_HEIGHT as f32,Vec2::new(new_pos.x + 1., new_pos.y));
            // Velocity is zero when enemy hits the ground
            if pt.translation.y <= -(LEVEL_H / 2.) + (W1_SPRITE_HEIGHT as f32) ||
                new_hb.all_enemy_collisions(&hitboxes)
            {
                pv.velocity.y = 0.;
                e_jump.is_jumping = false;
                e_jump.jumped = false;
                
            }
        }
    } else if *game_state == GamePhase::Planet2 {

    } else if *game_state == GamePhase::Planet3 {

    } else if *game_state == GamePhase::Planet4 {

    } else if *game_state == GamePhase::Planet5 {
        /*Julianne 10/8: This function is the same as player flight, but only makes the downward force on the enemy (no flight)*/
        for (mut pt, mut pv, mut pg, mut hb, mut e_jump) in &mut enemy{

            let deltat = time.delta_seconds();

            //update gravity here
            if e_jump.needs_jump && !e_jump.jumped{
                pg.reset_g();
                let acc_y = W1_ACCEL_RATE_Y * deltat;
                pv.velocity.y = f32::min(250., pv.velocity.y + (1. * acc_y));
                e_jump.needs_jump = false;
                e_jump.is_jumping = true;
            }else {
                pg.update_g(&pv.velocity.y, &deltat, &grav_res);
                pv.velocity.y = pg.get_g();
            }
            

            let change = pv.velocity * deltat;
            let new_pos = pt.translation + change.extend(0.);
            let new_hb = Hitbox::new(W1_SPRITE_WIDTH as f32, W1_SPRITE_HEIGHT as f32, new_pos.xy());
            //Bound enemy to within level height
            if new_pos.y >= -(LEVEL_H / 2.) + (W1_SPRITE_HEIGHT as f32) / 2.
                && new_pos.y <= LEVEL_H - (W1_SPRITE_HEIGHT as f32) / 2.
                && (!new_hb.all_enemy_collisions(&hitboxes)) && !e_jump.jumped
            {    

                    pt.translation = new_pos;
                    *hb = new_hb; 
                    e_jump.jumped = true;
            }  
            let new_hb = Hitbox::new(W1_SPRITE_WIDTH as f32, W1_SPRITE_HEIGHT as f32,Vec2::new(new_pos.x + 1., new_pos.y));
            // Velocity is zero when enemy hits the ground
            if pt.translation.y <= -(LEVEL_H / 2.) + (W1_SPRITE_HEIGHT as f32) ||
                new_hb.all_enemy_collisions(&hitboxes)
            {
                pv.velocity.y = 0.;
                e_jump.is_jumping = false;
                e_jump.jumped = false;
                
            }
        }

    } else if *game_state == GamePhase::Planet6 {

    } else if *game_state == GamePhase::Planet7 {

    } else if *game_state == GamePhase::Planet8 {
        for (mut pt, mut pv, mut pg, mut hb, mut e_jump) in &mut enemy{

            let deltat = time.delta_seconds();

            //update gravity here
            if e_jump.needs_jump && !e_jump.jumped{
                pg.reset_g();
                let acc_y = W1_ACCEL_RATE_Y * deltat;
                pv.velocity.y = f32::min(250., pv.velocity.y + (1. * acc_y));
                e_jump.needs_jump = false;
                e_jump.is_jumping = true;
            }else {
                pg.update_g(&pv.velocity.y, &deltat, &grav_res);
                pv.velocity.y = pg.get_g();
            }
            

            let change = pv.velocity * deltat;
            let new_pos = pt.translation + change.extend(0.);
            let new_hb = Hitbox::new(W1_SPRITE_WIDTH as f32, W1_SPRITE_HEIGHT as f32, new_pos.xy());
            //Bound enemy to within level height
            if new_pos.y >= -(LEVEL_H / 2.) + (W1_SPRITE_HEIGHT as f32) / 2.
                && new_pos.y <= LEVEL_H - (W1_SPRITE_HEIGHT as f32) / 2.
                && (!new_hb.all_enemy_collisions(&hitboxes)) && !e_jump.jumped
            {    

                    pt.translation = new_pos;
                    *hb = new_hb; 
                    e_jump.jumped = true;
            }  
            let new_hb = Hitbox::new(W1_SPRITE_WIDTH as f32, W1_SPRITE_HEIGHT as f32,Vec2::new(new_pos.x + 1., new_pos.y));
            // Velocity is zero when enemy hits the ground
            if pt.translation.y <= -(LEVEL_H / 2.) + (W1_SPRITE_HEIGHT as f32) ||
                new_hb.all_enemy_collisions(&hitboxes)
            {
                pv.velocity.y = 0.;
                e_jump.is_jumping = false;
                e_jump.jumped = false;
                
            }
        }
    }
}

pub fn animate_enemy(
    time: Res<Time>,
    mut enemy: Query<
        (
            &Velocity,
            &mut TextureAtlas,
            &mut AnimationTimer,
            &AnimationFrameCount,
        ),
        With<Enemy>,
    >,
    state: Res<State<GamePhase>>,
) {
    let game_state =  state.get();
    if *game_state == GamePhase::Planet1 {
        for (v, mut texture_atlas, mut timer, frame_count) in &mut enemy {
            //let (v, mut texture_atlas, mut timer, frame_count) = enemy.single_mut();
            let x_vel = Vec2::new(v.velocity.x, 0.);
            //info!(x_vel.x);
            if x_vel.cmpne(Vec2::ZERO).any() {
                timer.tick(time.delta());

                if timer.just_finished() {
                texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
                }
            }
        }
    } else if *game_state == GamePhase::Planet2 {

    } else if *game_state == GamePhase::Planet3 {

    } else if *game_state == GamePhase::Planet4 {

    } else if *game_state == GamePhase::Planet5 {
        for (v, mut texture_atlas, mut timer, frame_count) in &mut enemy {
            //let (v, mut texture_atlas, mut timer, frame_count) = enemy.single_mut();
            let x_vel = Vec2::new(v.velocity.x, 0.);
            //info!(x_vel.x);
            if x_vel.cmpne(Vec2::ZERO).any() {
                timer.tick(time.delta());

                if timer.just_finished() {
                texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
                }
            }
        }

    } else if *game_state == GamePhase::Planet6 {

    } else if *game_state == GamePhase::Planet7 {
    
    } else if *game_state == GamePhase::Planet8 {

    }
}

/*Julianne 10/8: This finds if the player is on the left or right side
 and simply makes enemy walk towards the player, changing x translation only
 This also check if enemy is within camera frame. If they are not, they will not move*/
pub fn track_player(
    time: Res<Time>,
    mut enemy: Query<(&mut Transform, &mut Velocity, &mut Sprite, &mut Hitbox, &mut AnimationTimer, &mut Jump), (With<Enemy>, Without<Player>)>,
    mut player: Query<(&mut Transform, &mut Health), (With<Player>, Without<Enemy>)>,
    hitboxes: Query<&Hitbox, Without<Enemy>>, 
    mut player_hitbox: Query<&Hitbox, (With<Player>, Without<Enemy>)>,
    mut camera: Query<&mut Transform, (Without<Player>, Without<Enemy>, With<Camera>)>,
    mut death_event: EventWriter<Death>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut sound_tracker: ResMut<PlayerSoundTracker>,
    map: ResMut<ParticleMap>,
    state: Res<State<GamePhase>>,
){
    let game_state =  state.get();
    if *game_state == GamePhase::Planet1 {
         //get enemy, player and camera
         for (mut et, mut ev, mut es, mut ehb, mut timer, mut e_jump) in &mut enemy{
            let (pt, mut player_health) = player.single_mut();
            let player_hb = player_hitbox.single_mut();
            
            let cam_t = camera.single_mut();
            let mut deltav_x = 0.;
        
            // Is enemy within the camera frame?
            if et.translation.x > cam_t.translation.x + (WIN_W/2.){
                return}
            else{
                timer.tick(time.delta());
            }
            //face player and walk towards player
            if pt.translation.x >= et.translation.x {
                deltav_x += 1.;
                es.flip_x=false;
            }
            else{
                deltav_x -= 1.;
                es.flip_x = true;
            }
        
            let deltat = time.delta_seconds();
            let acc_x = W1_ACCEL_RATE_X * deltat;
        
            if deltav_x != 0. {
                if ev.velocity.y >= 0. {
                    ev.velocity.x = (ev.velocity.x + deltav_x * acc_x).clamp(-W1_ENEMY_SPEED, W1_ENEMY_SPEED);
                }
                else {
                    ev.velocity.x = (ev.velocity.x + deltav_x * acc_x).clamp(-W1_ENEMY_SPEED * 0.3, W1_ENEMY_SPEED * 0.3);
                }
            } else if ev.velocity.x.abs() > acc_x {
                ev.velocity.x -= ev.velocity.x.signum() * acc_x;
            } else {
                ev.velocity.x = 0.;
            }
        
            let change = ev.velocity * deltat;
            let new_pos = et.translation + change.extend(0.);
            let new_hb = Hitbox::new(W1_SPRITE_WIDTH as f32, W1_SPRITE_HEIGHT as f32, new_pos.xy());
            
        
            let mut no_jump = false;
            if player_hb.collides_with(&new_hb) {
                no_jump = true;
                take_damage(&mut player_health, 1.0, &mut death_event, &asset_server, &mut commands, &mut sound_tracker, &time);
        
                //info!("Player hit! Current health: {:?}", player_health.current); // 记录伤害
                if player_health.current == 0.{
                    death_event.send(Death);
                }
            }
            if new_pos.x >= -(WIN_W / 2.) + (W1_SPRITE_WIDTH as f32) / 2.
                && new_pos.x <= LEVEL_W - (WIN_W / 2. + (W1_SPRITE_WIDTH as f32) / 2.)
                && new_hb.all_enemy_collisions(&hitboxes) && !e_jump.is_jumping && !no_jump
            {
                ev.velocity.x = 0.;
                e_jump.needs_jump = true;
            }
            if new_pos.x >= -(WIN_W / 2.) + (W1_SPRITE_WIDTH as f32) / 2.
                && new_pos.x <= LEVEL_W - (WIN_W / 2. + (W1_SPRITE_WIDTH as f32) / 2.)
                && !new_hb.all_enemy_collisions(&hitboxes)
            {
                et.translation = new_pos;
                *ehb = new_hb;
            }
            
        }
       
    }
    else if *game_state == GamePhase::Planet2 {
        for (mut enemy_transform, mut enemy_velocity, mut enemy_sprite, mut enemy_hb, mut timer, mut enemy_jump) in &mut enemy{
            let (player_transform, mut player_health) = player.single_mut();
            let player_hb = player_hitbox.single_mut();
            let cam_transform = camera.single_mut();
            let mut deltav_x = 0.;
            let mut deltav_y = 0.;

            // Is enemy within the camera frame?
            if enemy_transform.translation.x > cam_transform.translation.x + (WIN_W/2.){
                return}
            else{
                timer.tick(time.delta());
            }

            //face player and fly towards player
            if player_transform.translation.x >= enemy_transform.translation.x {
                deltav_x += 1.;
                enemy_sprite.flip_x=false;
            } else {
                deltav_x -= 1.;
                enemy_sprite.flip_x = true;
            }
            if player_transform.translation.y >= enemy_transform.translation.y {
                deltav_y += 1.;
            } else {
                deltav_y -= 1.;
            }

            let deltat = time.delta_seconds();
            let acc_x = W3_ACCEL_RATE_X * deltat;
            let acc_y = W3_ACCEL_RATE_Y * deltat;
            if deltav_x != 0. || deltav_y != 0. {
                let new_velocity = Vec2::new(deltav_x, deltav_y) + enemy_velocity.velocity;
                if new_velocity.length() > W3_ENEMY_SPEED {
                    enemy_velocity.velocity = new_velocity.normalize() * W3_ENEMY_SPEED;
                } else {
                    enemy_velocity.velocity = new_velocity;
                }
                
                let new_pos = enemy_transform.translation + enemy_velocity.velocity.extend(0.);
                if new_pos.x >= -(WIN_W / 2.) + (W1_SPRITE_WIDTH as f32) / 2.
                && new_pos.x <= LEVEL_W - (WIN_W / 2. + (W1_SPRITE_WIDTH as f32) / 2.)
                && new_pos.y >= -(LEVEL_H / 2.) + (W1_SPRITE_HEIGHT as f32) / 2.
                && new_pos.y <= LEVEL_H - (W1_SPRITE_HEIGHT as f32) / 2. {
                    enemy_transform.translation = new_pos;
                    *enemy_hb = Hitbox::new(W3_SPRITE_WIDTH as f32, W3_SPRITE_HEIGHT as f32, enemy_transform.translation.xy());
                }
                if player_hb.collides_with(&enemy_hb) {
                    take_damage(&mut player_health, 1.0, &mut death_event, &asset_server, &mut commands, &mut sound_tracker, &time);
                    //info!("Player hit! Current health: {:?}", player_health.current); // 记录伤害
                    if player_health.current == 0.{
                        death_event.send(Death);
                    }
                }
            }
        }
    }
    else if *game_state == GamePhase::Planet3 {
        for (mut enemy_transform, mut enemy_velocity, mut enemy_sprite, mut enemy_hb, mut timer, mut enemy_jump) in &mut enemy{
            let (player_transform, mut player_health) = player.single_mut();
            let player_hb = player_hitbox.single_mut();
            let cam_transform = camera.single_mut();
            let mut deltav_x = 0.;
            let mut deltav_y = 0.;

            // Is enemy within the camera frame?
            if enemy_transform.translation.x > cam_transform.translation.x + (WIN_W/2.){
                return}
            else{
                timer.tick(time.delta());
            }

            //face player and fly towards player
            if player_transform.translation.x >= enemy_transform.translation.x {
                deltav_x += 1.;
                enemy_sprite.flip_x=false;
            } else {
                deltav_x -= 1.;
                enemy_sprite.flip_x = true;
            }
            if player_transform.translation.y >= enemy_transform.translation.y {
                deltav_y += 1.;
            } else {
                deltav_y -= 1.;
            }

            let deltat = time.delta_seconds();
            let acc_x = W3_ACCEL_RATE_X * deltat;
            let acc_y = W3_ACCEL_RATE_Y * deltat;
            if deltav_x != 0. || deltav_y != 0. {
                let new_velocity = Vec2::new(deltav_x, deltav_y) + enemy_velocity.velocity;
                if new_velocity.length() > W3_ENEMY_SPEED {
                    enemy_velocity.velocity = new_velocity.normalize() * W3_ENEMY_SPEED;
                } else {
                    enemy_velocity.velocity = new_velocity;
                }
                
                let new_pos = enemy_transform.translation + enemy_velocity.velocity.extend(0.);
                if new_pos.x >= -(WIN_W / 2.) + (W1_SPRITE_WIDTH as f32) / 2.
                && new_pos.x <= LEVEL_W - (WIN_W / 2. + (W1_SPRITE_WIDTH as f32) / 2.)
                && new_pos.y >= -(LEVEL_H / 2.) + (W1_SPRITE_HEIGHT as f32) / 2.
                && new_pos.y <= LEVEL_H - (W1_SPRITE_HEIGHT as f32) / 2. {
                    enemy_transform.translation = new_pos;
                    *enemy_hb = Hitbox::new(W3_SPRITE_WIDTH as f32, W3_SPRITE_HEIGHT as f32, enemy_transform.translation.xy());
                }
                if player_hb.collides_with(&enemy_hb) {
                    take_damage(&mut player_health, 1.0, &mut death_event, &asset_server, &mut commands, &mut sound_tracker, &time);
                    //info!("Player hit! Current health: {:?}", player_health.current); // 记录伤害
                    if player_health.current == 0.{
                        death_event.send(Death);
                    }
                }
            }
        }

    }
    else if *game_state == GamePhase::Planet4 {
    }
    else if *game_state == GamePhase::Planet5 {
         //get enemy, player and camera
         for (mut et, mut ev, mut es, mut ehb, mut timer, mut e_jump) in &mut enemy{
            let (pt, mut player_health) = player.single_mut();
            let player_hb = player_hitbox.single_mut();
            
            let cam_t = camera.single_mut();
            let mut deltav_x = 0.;
        
            // if the player is hiding in quicksand, the enemy will not track player
            let ratio_of_quicksand_particles = player_hb.ratio_of_quicksand_grid_tiles(&map);
            //player needs to be more than half submerged to hide
            if ratio_of_quicksand_particles > 0.5 {
                return;
            }
            
            // Is enemy within the camera frame?
            if et.translation.x > cam_t.translation.x + (WIN_W/2.){
                return}
            else{
                timer.tick(time.delta());
            }
            //face player and walk towards player
            if pt.translation.x >= et.translation.x {
                deltav_x += 1.;
                es.flip_x=false;
            }
            else{
                deltav_x -= 1.;
                es.flip_x = true;
            }
        
            let deltat = time.delta_seconds();
            let acc_x = W1_ACCEL_RATE_X * deltat;
        
            if deltav_x != 0. {
                if ev.velocity.y >= 0. {
                    ev.velocity.x = (ev.velocity.x + deltav_x * acc_x).clamp(-W1_ENEMY_SPEED, W1_ENEMY_SPEED);
                }
                else {
                    ev.velocity.x = (ev.velocity.x + deltav_x * acc_x).clamp(-W1_ENEMY_SPEED * 0.3, W1_ENEMY_SPEED * 0.3);
                }
            } else if ev.velocity.x.abs() > acc_x {
                ev.velocity.x -= ev.velocity.x.signum() * acc_x;
            } else {
                ev.velocity.x = 0.;
            }
        
            let change = ev.velocity * deltat;
            let new_pos = et.translation + change.extend(0.);
            let new_hb = Hitbox::new(W1_SPRITE_WIDTH as f32, W1_SPRITE_HEIGHT as f32, new_pos.xy());
            
        
            let mut no_jump = false;
            if player_hb.collides_with(&new_hb) {
                no_jump = true;
                take_damage(&mut player_health, 1.0, &mut death_event, &asset_server, &mut commands, &mut sound_tracker, &time);
        
                //info!("Player hit! Current health: {:?}", player_health.current); // 记录伤害
                if player_health.current == 0.{
                    death_event.send(Death);
                }
            }
            if new_pos.x >= -(WIN_W / 2.) + (W1_SPRITE_WIDTH as f32) / 2.
                && new_pos.x <= LEVEL_W - (WIN_W / 2. + (W1_SPRITE_WIDTH as f32) / 2.)
                && new_hb.all_enemy_collisions(&hitboxes) && !e_jump.is_jumping && !no_jump
            {
                ev.velocity.x = 0.;
                e_jump.needs_jump = true;
            }
            if new_pos.x >= -(WIN_W / 2.) + (W1_SPRITE_WIDTH as f32) / 2.
                && new_pos.x <= LEVEL_W - (WIN_W / 2. + (W1_SPRITE_WIDTH as f32) / 2.)
                && !new_hb.all_enemy_collisions(&hitboxes)
            {
                et.translation = new_pos;
                *ehb = new_hb;
            }
            
        }
        
    }
    else if *game_state == GamePhase::Planet6 {
    }
    else if *game_state == GamePhase::Planet7 {
       
    }
    else if *game_state == GamePhase::Planet8 {
    }

    
}


pub fn check_enemy_damage(
    mut query: Query<(Entity, &mut Hitbox, &mut EnemyHealth), With<Enemy>>,
    map: ResMut<ParticleMap>,
    state: Res<State<GamePhase>>,
){
    let game_state =  state.get();
    if *game_state == GamePhase::Planet1 {
        for (entity, ehb, mut eHealth) in query.iter_mut() {
            if ehb.are_any_grid_tiles_water(&map) {
                // info!("Enemy hit by water particle");
                eHealth.hp -= 1.;
            }
        }
    }
    else if *game_state == GamePhase::Planet2 {
        for (entity, ehb, mut eHealth) in query.iter_mut() {
            if ehb.are_any_grid_tiles_water(&map) {
                // info!("Enemy hit by water particle");
                eHealth.hp -= 1.;
            }
        }

    }
    else if *game_state == GamePhase::Planet3 {
        for (entity, ehb, mut eHealth) in query.iter_mut() {
            if ehb.are_any_grid_tiles_water(&map) {
                // info!("Enemy hit by water particle");
                eHealth.hp -= 1.;
            }
        }
    }
    else if *game_state == GamePhase::Planet4 {

    }
    else if *game_state == GamePhase::Planet5 {

    }
    else if *game_state == GamePhase::Planet6 {

    }
    else if *game_state == GamePhase::Planet7 {

        
    }
    else if *game_state == GamePhase::Planet8 {
        for (entity, ehb, mut eHealth) in query.iter_mut() {
            if ehb.are_any_grid_tiles_water(&map) {
                // info!("Enemy hit by water particle");
                eHealth.hp -= 1.;
            }
        }
    }
}
pub fn check_enemy_death(
    mut commands: Commands,
    query: Query<(Entity, &mut Hitbox, &mut EnemyHealth), With<Enemy>>,
){
    for (entity, ehb, e_health) in query.iter() {
        if e_health.hp <= 0. {
            commands.entity(entity).despawn();
        }
    }
}


pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), initialize)
        //.add_systems(OnEnter(GamePhase::Planet1), initialize)
        .add_systems(OnEnter(GamePhase::Planet2), initialize)
        .add_systems(OnEnter(GamePhase::Planet3), initialize)
        .add_systems(OnEnter(GamePhase::Planet4), initialize)
        .add_systems(OnEnter(GamePhase::Planet5), initialize)
        .add_systems(OnEnter(GamePhase::Planet6), initialize)
        .add_systems(OnEnter(GamePhase::Planet7), initialize)
        .add_systems(OnEnter(GamePhase::Planet8), initialize)
        //app.add_systems(PreUpdate,  initialize.run_if(state_changed::<GamePhase>))
        .add_systems(Update, enemy_gravity.after(track_player).run_if(in_state(AppState::InGame)))
        .add_systems(Update, track_player.run_if(in_state(AppState::InGame)))
        .add_systems(Update, animate_enemy.after(track_player).run_if(in_state(AppState::InGame)))
        .add_systems(Update, check_enemy_damage.run_if(in_state(AppState::InGame)))
        .add_systems(Update, check_enemy_death.run_if(in_state(AppState::InGame)).after(check_enemy_damage));
    }
}