use bevy::prelude::*;

use crate::schedule::InGameSet;
use crate::state::GameState;
use crate::health::Health;

pub const DESPAWN_DISTANCE: f32 = 100.;


#[derive(Component, Debug)]
pub struct DisposableEntity;


pub struct DespawnPlugin;


impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                despawn_far_away_entities,
                despawn_dead_entities,
            ).in_set(InGameSet::DespawnEntities))
            .add_systems(OnEnter(GameState::GameOver), despawn_all_entities);
    }
}


fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform), With<DisposableEntity>>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        // Entity is far away from the camera's viewport.
        if distance > DESPAWN_DISTANCE {
            if let Some(entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn_recursive();
            }
        }
    }
}


fn despawn_dead_entities(mut commands: Commands, query: Query<(Entity, &Health), (With<Health>, With<DisposableEntity>)>) {
    for (entity, health) in query.iter() {
        if health.value > 0. {
            continue;
        }

        if let Some(entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
        }
    }
}


fn despawn_all_entities(mut commands: Commands, query: Query<Entity, Or<(With<Health>, With<DisposableEntity>)>>) {
    for entity in query.iter() {
        if let Some(entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
        }
    }
}
