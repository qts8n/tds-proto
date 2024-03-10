use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::schedule::InGameSet;
use crate::state::GameState;
use crate::asset_loader::SceneAssets;
use crate::movement::{DirVector, MovingObjectBundle};
use crate::health::Health;
use crate::collision_detection::CollisionDamage;
use crate::despawn_routine::DisposableEntity;

const SPACESHIP_SPAWN: Vec3 = Vec3::new(0., 0., -20.);
const SPACESHIP_SPEED: f32 = 25.;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIUS: f32 = 4.;
const SPACESHIP_HEALTH: f32 = 100.;
const SPACESHIP_COLLISION_DAMAGE: f32 = 70.;

const MISSILE_SPEED: f32 = 50.;
const MISSILE_FORWARD_SCALAR: f32 = 8.;
const MISSILE_RADIUS: f32 = 0.2;
const MISSILE_COOLDOWN: f32 = 0.2;
const MISSILE_SCALE: Vec3 = Vec3::new(5., 5., 5.);
const MISSILE_HEALTH: f32 = 1.;
const MISSILE_COLLISION_DAMAGE: f32 = 40.;


#[derive(Component, Debug)]
pub struct Spaceship;


#[derive(Component, Debug)]
pub struct SpaceshipMissile;


#[derive(Resource, Default, Debug)]
struct SpaceshipMissileCooldownTimer {
    timer: Timer,
}


impl SpaceshipMissileCooldownTimer {
    fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}


pub struct SpaceshipPlugin;


impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpaceshipMissileCooldownTimer::new(MISSILE_COOLDOWN))
            .add_systems(Update, (
                spaceship_movement_controls,
                spaceship_weapon_controls,
            ).chain().in_set(InGameSet::UserInput))
            .add_systems(Update, check_spaceship.in_set(InGameSet::EntityUpdates))
            .add_systems(OnExit(GameState::Menu), spawn_spaceship)
            .add_systems(OnEnter(GameState::GameOver), spawn_spaceship);
    }
}


fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let spaceship_transform = DirVector::new(SPACESHIP_SPAWN).get_transform();
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::linear(Vec3::ZERO),
            rigid_body: RigidBody::KinematicPositionBased,
            collider: Collider::ball(SPACESHIP_RADIUS),
            ..default()
        },
        SceneBundle {
            scene: scene_assets.get_random_spaceship(),
            transform: spaceship_transform,
            ..default()
        },
        Spaceship,
        Health::new(SPACESHIP_HEALTH),
        CollisionDamage::new(SPACESHIP_COLLISION_DAMAGE),
        DisposableEntity,
    ));
}


fn spaceship_movement_controls(
    mut query: Query<&mut Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = query.get_single_mut() else { return };

    let mut rotation = 0.;
    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation -= SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation += SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    let mut roll = 0.;
    if keyboard_input.pressed(KeyCode::KeyQ) {
        roll -= SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        roll += SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    let mut movement = 0.;
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement -= SPACESHIP_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        movement += SPACESHIP_SPEED * time.delta_seconds();
    }

    // Modify transform based on the processed input
    // NOTE: negative forward cause of the model direction
    //       that is set by Poly Pizza
    let translation = -transform.clone().forward() * movement;
    transform.translation += translation;
    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
}


fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cd_timer: ResMut<SpaceshipMissileCooldownTimer>,
    scene_assets: Res<SceneAssets>,
    time: Res<Time>,
) {
    if !keyboard_input.pressed(KeyCode::Space) {
        return;
    }

    cd_timer.timer.tick(time.delta());
    if !cd_timer.timer.just_finished() {
        return;
    }

    let Ok(transform) = query.get_single() else { return };
    let missile_transform = DirVector::new(transform.translation - transform.forward() * MISSILE_FORWARD_SCALAR)
        .get_transform().with_scale(MISSILE_SCALE);
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::linear(-transform.forward() * MISSILE_SPEED),
            collider: Collider::ball(MISSILE_RADIUS),
            ..default()
        },
        SceneBundle {
            scene: scene_assets.get_random_bullet(),
            transform: missile_transform,
            ..default()
        },
        SpaceshipMissile,
        Health::new(MISSILE_HEALTH),
        CollisionDamage::new(MISSILE_COLLISION_DAMAGE),
        DisposableEntity,
    ));
}


fn check_spaceship(mut next_state: ResMut<NextState<GameState>>, query: Query<(), With<Spaceship>>) {
    if query.get_single().is_err() {
        next_state.set(GameState::GameOver);
    }
}
