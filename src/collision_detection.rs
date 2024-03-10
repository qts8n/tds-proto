use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::schedule::InGameSet;
use crate::health::Health;


#[derive(Component, Default, Debug)]
pub struct CollisionDamage {
    pub amount: f32,
}


impl CollisionDamage {
    pub fn new(amount: f32) -> Self { Self { amount } }
}


pub struct CollisionDetectionPlugin;


impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, apply_collision_damage.in_set(InGameSet::CollisionDetection));
    }
}


pub fn apply_collision_damage(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut health_query: Query<&mut Health>,
    collision_damage_query: Query<&CollisionDamage>,
) {
    for &event in collision_event_reader.read() {
        let CollisionEvent::Started(entity1, entity2, _) = event else { continue };

        let Ok(mut health) = health_query.get_mut(entity1) else { continue };
        let Ok(collision_damage) = collision_damage_query.get(entity2) else { continue };
        health.value -= collision_damage.amount;

        let Ok(mut health) = health_query.get_mut(entity2) else { continue };
        let Ok(collision_damage) = collision_damage_query.get(entity1) else { continue };
        health.value -= collision_damage.amount;
    }
}
