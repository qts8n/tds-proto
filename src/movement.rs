use std::ops::Range;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;


#[derive(Default, Debug)]
pub struct DirVector {
    pub value: Vec3,
}


impl DirVector {
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
    pub rigid_body: RigidBody,
    pub gravity_scale: GravityScale,
    pub collider: Collider,
    pub sleeping: Sleeping,
    pub ccd: Ccd,
    pub active_events: ActiveEvents,
    pub model: SceneBundle,
}

