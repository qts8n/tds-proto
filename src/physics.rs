use bevy::{prelude::*, transform::TransformSystem};
use bevy_rapier3d::prelude::*;

use crate::state::GameState;


pub struct PhysicsPlugin;



impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        let rapier_plugin = RapierPhysicsPlugin::<NoUserData>::default()
            .with_default_system_setup(false);
        app
            .add_plugins(rapier_plugin)
            .add_plugins(RapierDebugRenderPlugin::default())
            .configure_sets(PostUpdate, (
                PhysicsSet::SyncBackend,
                PhysicsSet::StepSimulation,
                PhysicsSet::Writeback,
            ).chain().before(TransformSystem::TransformPropagate).run_if(in_state(GameState::InGame)))
            .add_systems(PostUpdate, (
                // These *must* be in the main schedule currently so that they do not miss events.
                systems::sync_removals,
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackend).in_set(PhysicsSet::SyncBackend),
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::StepSimulation).in_set(PhysicsSet::StepSimulation),
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::Writeback).in_set(PhysicsSet::Writeback),
            ));
    }
}
