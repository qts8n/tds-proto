use bevy::prelude::*;

use crate::{spaceship::Spaceship, state::GameState};


pub struct DebugPlugin;


impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, log_debug_presence);
        app.add_systems(OnEnter(GameState::Paused), log_spaceship_components);
    }
}


fn log_debug_presence() {
    println!("[DEBUG] INFO log: Debugger is active for this session!");
}


fn log_spaceship_components(mut commands: Commands, query: Query<Entity, With<Spaceship>>) {
    let Ok(spaceship) = query.get_single() else { return };
    let Some(mut spaceship_commands) = commands.get_entity(spaceship) else { return };
    spaceship_commands.log_components();
}
