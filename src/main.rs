mod schedule;
mod state;
mod asset_loader;
mod debug;
mod field;
mod camera;
mod menu;
mod movement;
mod health;
mod asteroids;
mod spaceship;
mod collision_detection;
mod despawn_routine;

use bevy::prelude::*;

use schedule::SchedulePlugin;
use state::StatePlugin;
use asset_loader::AssetLoaderPlugin;
use debug::DebugPlugin;
use field::FieldPlugin;
use camera::CameraPlugin;
use menu::MenuPlugin;
use movement::MovementPlugin;
use asteroids::AsteroidPlugin;
use spaceship::SpaceshipPlugin;
use collision_detection::CollisionDetectionPlugin;
use despawn_routine::DespawnPlugin;


fn main() {
    App::new()
        // -- Bevy configuration
        // Built-ins
        .add_plugins(DefaultPlugins)
        // -- Custom user configuration
        // Misc
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(DebugPlugin)
        // World defaults
        .add_plugins(FieldPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(MovementPlugin)
        // Game logic
        .add_plugins(AsteroidPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        // Run the magic
        .run();
}
