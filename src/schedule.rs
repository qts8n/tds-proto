use bevy::{ecs::schedule::ScheduleBuildSettings, prelude::*};

use crate::state::GameState;


#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub enum InGameSet {
    UserInput,
    EntityUpdates,
    CollisionDetection,
    DespawnEntities,
}


pub struct SchedulePlugin;


impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_schedules(ScheduleBuildSettings {
                auto_insert_apply_deferred: false, // Manually configure flush points
                ..default()
            })
            .configure_sets(Update, (
                InGameSet::DespawnEntities,
                // Flush point [#1] goes here
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
            ).chain().run_if(in_state(GameState::InGame)))
            // Insert a flush point [#1]
            .add_systems(Update, apply_deferred.after(InGameSet::DespawnEntities).before(InGameSet::UserInput));
    }
}
