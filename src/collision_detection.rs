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
    mut contact_event_reader: EventReader<ContactForceEvent>,
    mut health_query: Query<&mut Health>,
    collision_damage_query: Query<&CollisionDamage>,
) {
    for event in contact_event_reader.read() {
        let Ok(mut health) = health_query.get_mut(event.collider1) else { continue };
        let Ok(collision_damage) = collision_damage_query.get(event.collider2) else { continue };
        health.value -= collision_damage.amount;
    }
}
