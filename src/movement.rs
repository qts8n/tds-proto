use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::schedule::InGameSet;
use crate::collision_detection::Collider;


#[derive(Component, Default, Debug)]
pub struct Velocity {
    pub value: Vec3,
}


impl Velocity {
    pub fn new(value: Vec3) -> Self { Self { value } }

    pub fn rng_unit(n: Option<f32>) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(Vec3::new(
            rng.gen_range(-1.0..1.0),
            0.,
            rng.gen_range(-1.0..1.0),
        )
        .normalize_or_zero() * n.unwrap_or(1.))
    }
}


#[derive(Component, Default, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}


impl Acceleration {
    pub fn new(value: Vec3) -> Self { Self { value } }

    pub fn rng_unit(n: Option<f32>) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(Vec3::new(
            rng.gen_range(-1.0..1.0),
            0.,
            rng.gen_range(-1.0..1.0),
        )
        .normalize_or_zero() * n.unwrap_or(1.))
    }
}


#[derive(Default, Debug)]
pub struct Translation {
    pub value: Vec3,
}


impl Translation {
    pub fn new(value: Vec3) -> Self { Self { value } }

    pub fn rng_range(x_range: Range<f32>, z_range: Range<f32>) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(Vec3::new(
            rng.gen_range(x_range),
            0.,
            rng.gen_range(z_range),
        ))
    }

    pub fn get_transform(&self) -> Transform {
        Transform::from_translation(self.value)
    }
}


#[derive(Bundle, Default)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub collider: Collider,
    pub model: SceneBundle,
}


pub struct MovementPlugin;


impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_velocity,
            update_position,
        ).chain().in_set(InGameSet::EntityUpdates));
    }
}


fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}


fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
