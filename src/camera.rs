use bevy::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 80., 0.);


pub struct CameraPlugin;


impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(STARTING_TRANSLATION).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}
