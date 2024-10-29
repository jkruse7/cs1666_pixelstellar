use bevy::{prelude::*};
use super::{components::*, resources::*, blaster::components::*};
use crate::{
    common::{
        gravity::Gravity, hitbox::{self, Hitbox}
    },
    entities::{enemy::components::Enemy, particle::resources::ParticleMap},
    LEVEL_H,
    LEVEL_W,
};

pub fn initialize_player(
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
        Hitbox::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32, Vec2::new(0., 100.)),
        JetPack::new(1.,0.5),
        Player,
    ));
}

pub fn movee(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut Gravity, &mut Sprite, &mut Hitbox, &mut JetPack), With<Player>>,
    world_hitboxes: Query<&Hitbox, (Without<Enemy>, Without<Player>)>,
    map: ResMut<ParticleMap>,
){
    let deltat = time.delta_seconds();

    let (mut pt, mut pv, mut pg, mut ps, mut hb, mut jp) = player.single_mut();

    let mut deltav_x = 0.;
    let acc_y = ACCEL_RATE_Y * deltat;
    let acc_x = ACCEL_RATE_X * deltat;


    // Check inputs
    if input.pressed(KeyCode::KeyA) {
        if (pt.translation.x >= -(LEVEL_W / 2.) + (SPRITE_WIDTH as f32) / 2.){
            deltav_x -= 1.;
        }
        ps.flip_x = true;
    }
    if input.pressed(KeyCode::KeyD) {
        if pt.translation.x <= LEVEL_W - (LEVEL_W / 2. + (SPRITE_WIDTH as f32) / 2.){
            deltav_x += 1.;
        }
        ps.flip_x = false;
    }
    if input.pressed(KeyCode::Space) && !jp.disabled {
        pg.reset_g();
        jp.fly();
        if pt.translation.y <= (LEVEL_H / 2.) - (SPRITE_HEIGHT as f32) / 2. {
            pv.velocity.y = f32::min(MAX_FLIGHT_SPEED, pv.velocity.y + (1. * acc_y))
        } else {
            pv.velocity.y = 0.0;
        }
    } else {
        jp.recharge();
        // if at world floor  or  if colliding with a hitbox
        if pt.translation.y <= -(LEVEL_H / 2.) + (SPRITE_HEIGHT as f32){
            pv.velocity.y = 0.;
        } else {
            pg.update_g(&pv.velocity.y, &deltat);
            pv.velocity.y = pg.get_g();
        }
    }


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

    let ratio_of_water_particles = hb.ratio_of_water_grid_tiles(&map);
    if ratio_of_water_particles > 0.0 {
        pv.velocity = pv.velocity * Vec2::splat(1. - 0.7 * ratio_of_water_particles.powf(0.5));
    }
    


    // Get future position
    let change = pv.velocity * deltat;
    let new_pos = pt.translation + change.extend(0.);
    let new_hb = Hitbox::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32, new_pos.xy());

    // i hate hitboxes
    if !new_hb.on_top_of_all(&world_hitboxes){
        pt.translation = new_pos;
        *hb = new_hb;
        pv.velocity.y = 0.;

    }

    // If position is 
    /*let step = new_hb.player_step(&world_hitboxes);
    info!(step);
    if step >= 0. {
        pt.translation = new_pos;
        pt.translation.y += step;
        *hb = new_hb;
        hb.offset.y += step;
    } */

}

pub fn damage(
    mut player: Query<(&Hitbox, &mut Health), (With<Player>, Without<Enemy>)>,
    enemy:  Query<&Hitbox, (Without<Player>, With<Enemy>)>
){
    let (player_hitbox, mut player_health) = player.single_mut();
    for enemy_hitbox in enemy.iter(){
        if player_hitbox.collides_with(enemy_hitbox){
            player_health.current -= 1.;
        }

    }
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
        app.add_systems(Startup, initialize_player);
        app.add_systems(Startup, super::blaster::systems::initialize);


        app.add_systems(Update, movee);
        
        //app.add_systems(Update, flight.after(super::systems::movee));
        app.add_systems(Update, animate_player.after(movee));

        app.add_systems(Update, damage.after(movee));
        //Blaster stuff
        app.add_systems(Update, super::blaster::systems::update_blaster_aim);
        app.add_systems(Update, super::blaster::systems::update_blaster_position.after(movee));
        //app.add_systems(Update, super::blaster::systems::shoot_blaster.after(super::blaster::systems::update_blaster_aim));


    }
} 

