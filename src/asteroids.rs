use std::ops::Range;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::asset_loader::SceneAssets;
use crate::schedule::InGameSet;
use crate::hud::GameScoreChangeEvent;
use crate::movement::{DirVector, MovingObjectBundle};
use crate::health::Health;
use crate::collision_detection::CollisionDamage;
use crate::despawn_routine::{DESPAWN_DISTANCE, DisposableEntity};

const VELOCITY_SCALAR: f32 = 20.;
const VELOCITY_DEST_RADIUS_RANGE: Range<f32> = 0.0..50.0;

const SPAWN_RANGE_RADIUS: Range<f32> = 81.0..(DESPAWN_DISTANCE - 1.);
const SPAWN_SECONDS: f32 = 0.5;

const ROTATION_SPEED: f32 = 2.5;
const RADIUS: f32 = 2.5;
const HEALTH: f32 = 80.;
const COLLISION_DAMAGE: f32 = 35.;


#[derive(Component, Debug)]
pub struct Asteroid;


#[derive(Component, Debug)]
pub struct AsteroidParticle;


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
                explode_dead_asteroids,
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

    let translation = DirVector::rng_polar_range(SPAWN_RANGE_RADIUS);
    let destination = DirVector::rng_polar_range(VELOCITY_DEST_RADIUS_RANGE);
    let velocity = DirVector::new((destination.value - translation.value).normalize_or_zero() * VELOCITY_SCALAR);

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::linear(velocity.value),
            collider: Collider::ball(RADIUS),
            ..default()
        },
        SceneBundle {
            scene: scene_assets.get_random_asteroid(),
            transform: translation.get_transform(),
            ..default()
        },
        Asteroid,
        Health::new(HEALTH),
        CollisionDamage::new(COLLISION_DAMAGE),
    ));
}


fn rotate_asteroids(mut query: Query<&mut Transform, Or<(With<Asteroid>, With<AsteroidParticle>)>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATION_SPEED * time.delta_seconds());
    }
}


fn explode_dead_asteroids(
    mut commands: Commands,
    query: Query<(Entity, &Health), With<Asteroid>>,
    children_query: Query<&Children>,
    mesh_query: Query<&Handle<Mesh>>,
    mut score_change_event_writer: EventWriter<GameScoreChangeEvent>,
) {
    for (entity, health) in query.iter() {
        if health.value > 0. {
            continue;
        }
        for child in children_query.iter_descendants(entity) {
            if mesh_query.get(child).is_err() {
                continue;  // Not interested in meshless entities
            }
            let Some(mut child_commands) = commands.get_entity(child) else { continue };
            let velocity = DirVector::rng_unit(Some(VELOCITY_SCALAR));
            child_commands.remove_parent_in_place();
            child_commands.insert((
                MovingObjectBundle {
                    velocity: Velocity::linear(velocity.value),
                    collider: Collider::ball(RADIUS / 10.),
                    ..default()
                },
                AsteroidParticle,
                DisposableEntity,
            ));
        }
        score_change_event_writer.send(GameScoreChangeEvent {
            score_delta: 1,
            clear_score: false,
        });
        let Some(asteroid_commands) = commands.get_entity(entity) else { continue };
        asteroid_commands.despawn_recursive();
    }
}
