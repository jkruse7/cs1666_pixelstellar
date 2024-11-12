use bevy::prelude::*;
use super::{components::*, resources::*};
use crate::{
    common::{
        hitbox::Hitbox,
        gravity::Gravity,
        
        state::{AppState, GamePhase},
    },
    common::state::set_next_state,
    LEVEL_H,
    LEVEL_W,
    WIN_W,
};



pub fn initialize(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let ship_sheet_handle = asset_server.load("spaceship_nobkg.png");
    let ship_layout = TextureAtlasLayout::from_grid(UVec2::splat(100), 1, 1, None, None);
    let ship_layout_handle = texture_atlases.add(ship_layout);
    commands.spawn((
        SpriteBundle {
            texture: ship_sheet_handle,
            transform: Transform {
                translation: Vec3::new(WIN_W/3., 100.0, 902.),
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
            layout: ship_layout_handle,
            index: 0,
        },
        Velocity::new(),
        Gravity::new(),
        Hitbox::new(25 as f32, 100 as f32, Vec2::new(WIN_W/3., 110.)),
        Spaceship,
        FoundFlag::new(),
    ));
}


pub fn spaceship_gravity(
    time: Res<Time>, 
    mut ship: Query<(&mut Transform, &mut Velocity, &mut Gravity, &mut Hitbox), With<Spaceship>>, 
    hitboxes: Query<&Hitbox, Without<Spaceship>>,
) {
    /*Julianne 10/8: This function is the same as player flight, but only makes the downward force on the enemy (no flight)*/
    for (mut pt, mut pv, mut pg, mut hb) in &mut ship{

    let deltat = time.delta_seconds();

    //update gravity here
        pg.update_g(&pv.velocity.y, &deltat);
        pv.velocity.y = pg.get_g();
    

    let change = pv.velocity * deltat;
    let new_pos = pt.translation + change.extend(0.);
    let new_hb = Hitbox::new(25 as f32, 100 as f32, Vec2::new(new_pos.x-25., new_pos.y));
    //Bound enemy to within level height
    if new_pos.y >= -(LEVEL_H / 2.) + (100 as f32) / 2.
        && new_pos.y <= LEVEL_H - (100 as f32) / 2.
        && !new_hb.all_ship_collisions(&hitboxes)
    {    

            pt.translation = new_pos;
            *hb = new_hb; 
    }  
    let new_hb = Hitbox::new(25 as f32, 100 as f32,Vec2::new(new_pos.x-25., new_pos.y));
    // Velocity is zero when hits the ground
    if pt.translation.y <= -(LEVEL_H / 2.) + (50 as f32) ||
        new_hb.all_ship_collisions(&hitboxes)
    {
        pv.velocity.y = 0.;
        
    }
}
}

fn found_spaceship_event_listener(
    mut ship_event: EventReader<FoundSpaceship>,
    //mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GamePhase>>,
    next_phase: ResMut<NextState<GamePhase>>,
    next_app_state: ResMut<NextState<AppState>>,
    
) {
    if !ship_event.is_empty() {
        info!("player found ship!");
        //next_app_state.set(AppState::WinScreen);
        ship_event.clear();
        set_next_state(state, next_phase, next_app_state);
        
    }
}


pub struct SpaceshipPlugin;
impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), initialize)
        .add_systems(OnEnter(GamePhase::Planet2), initialize)
        //app.add_systems(PreUpdate,  initialize.run_if(state_changed::<GamePhase>))
        .add_systems(Update, spaceship_gravity.run_if(in_state(AppState::InGame)))
        .add_systems(Update, found_spaceship_event_listener.run_if(in_state(AppState::InGame)))
        .add_event::<FoundSpaceship>();
        
    }
}