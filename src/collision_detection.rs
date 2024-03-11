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
    mut query: Query<(&mut Health, &CollisionDamage)>,
    parent_query: Query<&Parent>,
) {
    for &event in collision_event_reader.read() {
        let CollisionEvent::Started(entity1, entity2, _) = event else { continue };

        let mut collided_entity1 = entity1;
        if !query.contains(entity1) {
            for ancestor in parent_query.iter_ancestors(entity1) {
                if query.contains(ancestor) {
                    collided_entity1 = ancestor;
                    break;
                }
            }
        }

        let mut collided_entity2 = entity2;
        if !query.contains(entity2) {
            for ancestor in parent_query.iter_ancestors(entity2) {
                if query.contains(ancestor) {
                    collided_entity2 = ancestor;
                    break;
                }
            }
        }

        let Ok([
            (mut health1, collision_damage1),
            (mut health2, collision_damage2),
        ]) = query.get_many_mut([collided_entity1, collided_entity2]) else {
            continue;
        };

        health1.value -= collision_damage2.amount;
        health2.value -= collision_damage1.amount;
    }
}
