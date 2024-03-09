use std::ops::Range;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::schedule::InGameSet;
use crate::asset_loader::SceneAssets;
use crate::movement::{DirVector, MovingObjectBundle};
use crate::health::Health;
use crate::collision_detection::CollisionDamage;

const VELOCITY_SCALAR: f32 = 10.;

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_SECONDS: f32 = 1.;

const ROTATION_SPEED: f32 = 2.5;
const RADIUS: f32 = 2.5;
const HEALTH: f32 = 80.;
const COLLISION_DAMAGE: f32 = 35.;


#[derive(Component, Debug)]
pub struct Asteroid;


#[derive(Resource, Default, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}


impl SpawnTimer {
    fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}


pub struct AsteroidPlugin;


impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpawnTimer::new(SPAWN_SECONDS))
            .add_systems(Update, (
                spawn_asteroid,
                rotate_asteroids,
            ).in_set(InGameSet::EntityUpdates));
    }
}


fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }
    let translation = DirVector::rng_range(SPAWN_RANGE_X, SPAWN_RANGE_Z);
    let velocity = DirVector::rng_unit(Some(VELOCITY_SCALAR));

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::linear(velocity.value),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(RADIUS),
            gravity_scale: GravityScale(0.),
            sleeping: Sleeping::disabled(),
            ccd: Ccd::enabled(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            model: SceneBundle {
                scene: scene_assets.get_random_asteroid(),
                transform: translation.get_transform(),
                ..default()
            },
        },
        Asteroid,
        Health::new(HEALTH),
        CollisionDamage::new(COLLISION_DAMAGE),
    ));
}


fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATION_SPEED * time.delta_seconds());
    }
}
