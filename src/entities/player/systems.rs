use bevy::prelude::*;
use super::{blaster::{self, components::*}, components::*, resources::*};
use crate::{
    common::{
        hitbox::Hitbox,
        gravity::Gravity,
        state::{AppState, GamePhase},
    },
    entities::particle::resources::ParticleMap,
    entities::enemy::components::Enemy,
    entities::spaceship::components::{Spaceship, FoundSpaceship},
    LEVEL_H,
    LEVEL_W,
};



pub fn initialize(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let player_sheet_handle = asset_server.load("walking.png");
    //               used to be tilesize. removed TILE_SIZE and now at 100, but change as needed  \/
    let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(100), 4, 1, None, None);
    let player_layout_len = player_layout.textures.len();
    let player_layout_handle = texture_atlases.add(player_layout);
    commands.spawn((
        SpriteBundle {
            texture: player_sheet_handle,
            transform: Transform {
                translation: Vec3::new(0., 100.0, 900.),
                ..default()
            },
            sprite: Sprite {
                // Flip the logo to the left
                flip_x: false,
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: player_layout_handle,
            index: 0,
        },
        AnimationTimer(Timer::from_seconds(ANIM_TIME, TimerMode::Repeating)),
        AnimationFrameCount(player_layout_len),
        Velocity::new(),
        Health::new(100.0),
        Gravity::new(),
        Hitbox::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32, Vec2::new(0., 110.)),
        Player,
    ));
}

pub fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut Sprite, &mut Hitbox, &mut Health), With<Player>>,
    hitboxes: Query<&Hitbox, Without<Player>>,
    mut blaster_transform: Query<&mut Transform, (With<Blaster>, Without<Enemy>, Without<Player>)>,
    map: ResMut<ParticleMap>,
    mut spaceship_hitbox: Query<&Hitbox, (With<Spaceship>, Without<Player>)>,
    mut ship_event: EventWriter<FoundSpaceship>
) {
    let (mut pt, mut pv, mut ps, mut hb, mut player_health) = player.single_mut();
    let mut deltav_x = 0.;
    let mut bt = blaster_transform.single_mut();
    let mut spaceship_hb = spaceship_hitbox.single_mut();

    if input.pressed(KeyCode::KeyA) {
        if pt.translation.x >= -(LEVEL_W / 2.) + (SPRITE_WIDTH as f32) / 2.{
            deltav_x -= 1.;
            ps.flip_x = true;
        }
    }

    if input.pressed(KeyCode::KeyD) {
        if pt.translation.x <= LEVEL_W - (LEVEL_W / 2. + (SPRITE_WIDTH as f32) / 2.){
            deltav_x += 1.;
            ps.flip_x = false;
        }
    }
    let deltat = time.delta_seconds();
    let acc_x = ACCEL_RATE_X * deltat;

    if deltav_x != 0. {
        if pv.velocity.y >= 0. {
            pv.velocity.x = (pv.velocity.x + deltav_x * acc_x).clamp(-PLAYER_SPEED, PLAYER_SPEED);
        }
        else {
            pv.velocity.x = (pv.velocity.x + deltav_x * acc_x).clamp(-PLAYER_SPEED * 0.5, PLAYER_SPEED * 0.5);
        }
    } else if pv.velocity.x.abs() > acc_x {
        pv.velocity.x -= pv.velocity.x.signum() * acc_x;
    } else {
        pv.velocity.x = 0.;
    }

    //Account for player in water
    let ratio_of_water_particles = hb.ratio_of_water_grid_tiles(&map);
    if ratio_of_water_particles > 0.0 {
        pv.velocity.x = pv.velocity.x * (1. - 0.7 * ratio_of_water_particles.powf(0.5));
    }

    let change = pv.velocity * deltat;
    let new_pos = pt.translation + change.extend(0.);
    let new_hb = Hitbox::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32, new_pos.xy());

    if new_hb.collides_with(&spaceship_hb){
        ship_event.send(FoundSpaceship);
    }
    
    if new_pos.x >= -(LEVEL_W / 2.) + (SPRITE_WIDTH as f32) / 2.
        && new_pos.x <= LEVEL_W - (LEVEL_W / 2. + (SPRITE_WIDTH as f32) / 2.)
        && !new_hb.all_player_collisions(&hitboxes)
    {
        pt.translation = new_pos;
        *hb = new_hb;
        bt.translation.x = pt.translation.x + BLASTER_OFFSET_X;
        bt.translation.y = pt.translation.y + BLASTER_OFFSET_Y;
    }
    //info!("{}", pt.translation);

}

pub fn flight(
    time: Res<Time>, 
    input: Res<ButtonInput<KeyCode>>, 
    mut player: Query<(&mut Transform, &mut Velocity, &mut Gravity, &mut Hitbox), With<Player>>, 
    hitboxes: Query<&Hitbox, Without<Player>>,
    mut blaster_transform: Query<&mut Transform, (With<Blaster>, Without<Enemy>, Without<Player>)>,
    map: ResMut<ParticleMap>,
) {
    let (mut pt, mut pv, mut pg, mut hb) = player.single_mut();
    let mut bt = blaster_transform.single_mut();
    let deltat = time.delta_seconds();
    let acc_y = ACCEL_RATE_Y * deltat;

    if input.pressed(KeyCode::Space) {
        if pt.translation.y <= (LEVEL_H / 2.) - (SPRITE_HEIGHT as f32) / 2. {
            pg.reset_g();
            pv.velocity.y = f32::min(MAX_FLIGHT_SPEED, pv.velocity.y + (1. * acc_y))
        }
        else {
            pg.reset_g();
            pv.velocity.y = 0.0;
        }
    } else {
        pg.update_g(&pv.velocity.y, &deltat);
        pv.velocity.y = pg.get_g();
    }
    //Account for player in water
    let ratio_of_water_particles = hb.ratio_of_water_grid_tiles(&map);
    if ratio_of_water_particles > 0.0 {
        pv.velocity.y = pv.velocity.y * (1. - 0.7 * ratio_of_water_particles.powf(0.5));
    }
    let change = pv.velocity * deltat;
    let new_pos = pt.translation + change.extend(0.);
    let new_hb = Hitbox::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32, new_pos.xy());
    //Bound player to within level height

    if new_pos.y >= -(LEVEL_H / 2.) + (SPRITE_HEIGHT as f32) / 2.
        && new_pos.y <= (LEVEL_H / 2.) - (SPRITE_HEIGHT as f32) / 2.
        && !new_hb.all_player_collisions(&hitboxes)
    {
        pt.translation = new_pos;
        *hb = new_hb;
        bt.translation.x = pt.translation.x + BLASTER_OFFSET_X;
        bt.translation.y = pt.translation.y + BLASTER_OFFSET_Y;
    }  
    
    let new_hb = Hitbox::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32, new_pos.xy());
    // Velocity is zero when player hits the ground
    if pt.translation.y <= -(LEVEL_H / 2.) + (SPRITE_HEIGHT as f32) ||
        new_hb.all_player_collisions(&hitboxes) 
    {
        pv.velocity.y = 0.;
    }
    //assumes the player is a square and pt.translation is the lower-left corner
}

pub fn animate_player(
    time: Res<Time>,
    mut player: Query<
        (
            &Velocity,
            &mut TextureAtlas,
            &mut AnimationTimer,
            &AnimationFrameCount,
        ),
        With<Player>,
    >,
) {
    let (v, mut texture_atlas, mut timer, frame_count) = player.single_mut();
    let x_vel = Vec2::new(v.velocity.x, 0.);
    //info!(x_vel.x);
    if x_vel.cmpne(Vec2::ZERO).any() {
        timer.tick(time.delta());

        if timer.just_finished() {
        texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
         }
    }
}




pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Startup events
        app.add_systems(OnEnter(AppState::InGame), initialize);
        app.add_systems(OnEnter(GamePhase::Level2), initialize);
       // app.add_systems(PreUpdate,  initialize.run_if(state_changed::<GamePhase>));
        app.add_event::<super::blaster::components::ChangeBlasterEvent>();
        app.add_systems(OnEnter(AppState::InGame), super::blaster::systems::initialize.after(initialize));
        app.add_systems(OnEnter(GamePhase::Level2), super::blaster::systems::initialize.after(initialize));


        app.add_systems(Update, move_player.run_if(in_state(AppState::InGame)));
        
        app.add_systems(Update, flight.after(super::systems::move_player).run_if(in_state(AppState::InGame)));
        app.add_systems(Update, animate_player.after(super::systems::move_player).run_if(in_state(AppState::InGame)));
        app.add_systems(Update, super::blaster::systems::update_blaster_aim.run_if(in_state(AppState::InGame)));
        app.add_systems(Update, super::blaster::systems::shoot_blaster.run_if(in_state(AppState::InGame)));
        app.add_systems(Update, super::blaster::systems::handle_blaster_change_input.run_if(in_state(AppState::InGame)));
        app.add_systems(Update, super::blaster::systems::change_blaster_on_event.run_if(in_state(AppState::InGame)));
     //   app.add_system(super::blaster::systems::switch_blaster.system());
      //  app.add_system(super::blaster::systems::handle_blaster_switch.system());
        

    }
} 

