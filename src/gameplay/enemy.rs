use bevy::prelude::*;
use crate::ParticleMap;
use crate::{
    engine::{
        hitbox::Hitbox,
        gravity::Gravity,
    },
    gameplay::player::{
        Player,
        Health,
    },
    ui::health::update_health_bar,
    world::tiles::tiles,
    LEVEL_H,
    LEVEL_W,
    WIN_W,
    WIN_H,
};

const TILE_SIZE: u32 = 100;
const ENEMY_SPEED: f32 = 100.;
const ACCEL_RATE_X: f32 = 5000.;
const ACCEL_RATE_Y: f32 = 10800.;
const ANIM_TIME: f32 = 0.2;
const SPRITE_HEIGHT: u32 = 50;
const SPRITE_WIDTH: u32 = 30;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Jump {
    is_jumping: bool,
    needs_jump: bool,
    jumped: bool,
}

impl Jump{
    fn new() -> Self {
        Self {
            is_jumping: false,
            needs_jump: false,
            jumped: false,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationFrameCount(usize);

#[derive(Component)]
pub struct Velocity {
    velocity: Vec2,
}

impl Velocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Self { velocity }
    }
}

#[derive(Component)]
pub struct EnemyHealth {
    hp: i32,
}

impl EnemyHealth {
    fn new() -> Self {
        Self {
            hp: 100,
        }
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize)
        .add_systems(Update, enemy_gravity.after(track_player))
        .add_systems(Update, track_player)
        .add_systems(Update, animate_enemy.after(track_player))
        .add_systems(Update, check_enemy_death);
    }
}


pub fn initialize(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let enemy_sheet_handle = asset_server.load("enemy_walking.png");
    let enemy_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 4, 1, None, None);
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
        AnimationTimer(Timer::from_seconds(ANIM_TIME, TimerMode::Repeating)),
        AnimationFrameCount(enemy_layout_len),
        Velocity::new(),
        EnemyHealth::new(),
        Gravity::new(),
        Hitbox::new(40 as f32, 40 as f32, Vec2::new(0., -210.)),
        DamageBox::new(50.0, 50.0, Vec2::new(0., -210.)),
        Jump::new(),  
        Enemy,
    ));
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
    hitboxes: Query<(&Hitbox), Without<Enemy>>,
    tiles: Query<(&tiles)>
) {
    /*Julianne 10/8: This function is the same as player flight, but only makes the downward force on the enemy (no flight)*/
    for (mut pt, mut pv, mut pg, mut hb, mut e_jump) in &mut enemy{

    let deltat = time.delta_seconds();

    //update gravity here
    if e_jump.needs_jump && !e_jump.jumped{
        pg.reset_g();
        let acc_y = ACCEL_RATE_Y * deltat;
        pv.velocity.y = f32::min(250., pv.velocity.y + (1. * acc_y));
        e_jump.needs_jump = false;
        e_jump.is_jumping = true;
    }else {
        pg.update_g(&pv.velocity.y, &deltat);
        pv.velocity.y = pg.get_g();
    }
    

    let change = pv.velocity * deltat;
    let new_pos = pt.translation + change.extend(0.);
    let new_hb = Hitbox::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32, new_pos.xy());
    //Bound enemy to within level height
    if new_pos.y >= -(LEVEL_H / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.y <= LEVEL_H - (TILE_SIZE as f32) / 2.
        && (!new_hb.all_enemy_collisions(&hitboxes)) && !e_jump.jumped
    {    

            pt.translation = new_pos;
            *hb = new_hb; 
            e_jump.jumped = true;
    }  
    let new_hb = Hitbox::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32,Vec2::new(new_pos.x + 1., new_pos.y));
    // Velocity is zero when enemy hits the ground
    if (pt.translation.y <= -(LEVEL_H / 2.) + (TILE_SIZE as f32) ||
        new_hb.all_enemy_collisions(&hitboxes) )
    {
        pv.velocity.y = 0.;
        e_jump.is_jumping = false;
        e_jump.jumped = false;
        
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
) {
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
}

/*Julianne 10/8: This finds if the player is on the left or right side
 and simply makes enemy walk towards the player, changing x translation only
 This also check if enemy is within camera frame. If they are not, they will not move*/
pub fn track_player(
    time: Res<Time>,
    mut commands: Commands, 
    mut enemy: Query<(&mut Transform, &mut Velocity, &mut Sprite, &mut Hitbox, &mut DamageBox, &mut AnimationTimer, &mut Gravity, &mut Jump), (With<Enemy>, Without<Player>)>,
    mut player: Query<(&mut Transform, &mut Health), (With<Player>, Without<Enemy>)>,
    hitboxes: Query<(&Hitbox), Without<Enemy>>, 
    mut player_hitbox: Query<(&Hitbox), (With<Player>, Without<Enemy>)>,
    mut camera: Query<&mut Transform, (Without<Player>, Without<Enemy>, With<Camera>)>
){
    //get enemy, player and camera
    for (mut et, mut ev, mut es, mut ehb, mut edb, mut timer, mut eg, mut e_jump) in &mut enemy{
    let (mut pt, mut player_health) = player.single_mut();
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
    let acc_x = ACCEL_RATE_X * deltat;

    if deltav_x != 0. {
        if ev.velocity.y >= 0. {
            ev.velocity.x = (ev.velocity.x + deltav_x * acc_x).clamp(-ENEMY_SPEED, ENEMY_SPEED);
        }
        else {
            ev.velocity.x = (ev.velocity.x + deltav_x * acc_x).clamp(-ENEMY_SPEED * 0.3, ENEMY_SPEED * 0.3);
        }
    } else if ev.velocity.x.abs() > acc_x {
        ev.velocity.x -= ev.velocity.x.signum() * acc_x;
    } else {
        ev.velocity.x = 0.;
    }

    let change = ev.velocity * deltat;
    let new_pos = et.translation + change.extend(0.);
    let new_hb = Hitbox::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32, new_pos.xy());
    
    let enemy_pos = et.translation.xy();
    let player_pos = pt.translation.xy();

    let mut no_jump = false;
    if player_hb.collides_with(&new_hb) {
        no_jump = true;
        player_health.current -= 1.; 
        info!("Player hit! Current health: {:?}", player_health.current); // 记录伤害
    }
    if new_pos.x >= -(WIN_W / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.x <= LEVEL_W - (WIN_W / 2. + (TILE_SIZE as f32) / 2.)
        && new_hb.all_enemy_collisions(&hitboxes) && !e_jump.is_jumping && !no_jump
    {
        ev.velocity.x = 0.;
        e_jump.needs_jump = true;
    }
    if new_pos.x >= -(WIN_W / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.x <= LEVEL_W - (WIN_W / 2. + (TILE_SIZE as f32) / 2.)
        && !new_hb.all_enemy_collisions(&hitboxes)
    {
        et.translation = new_pos;
        *ehb = new_hb;
    }
    
}
}


pub fn check_enemy_death(
    mut commands: Commands,
    query: Query<(Entity, &mut Hitbox), (With<Enemy>)>,
    map: ResMut<ParticleMap>,
){
    //TODO: Check if collided with blaster particle 
    for (entity, ehb) in query.iter() {
        if ehb.are_any_grid_tiles_water(&map) {
            info!("Enemy hit by water particle");
            commands.entity(entity).despawn();
        }
        
    }
}