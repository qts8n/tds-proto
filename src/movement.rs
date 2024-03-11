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

    pub fn rng_polar_range(r_range: Range<f32>) -> Self {
        let mut rng = rand::thread_rng();

        let mut dir_vector = Self::rng_unit(None);
        dir_vector.value *= rng.gen_range(r_range);

        dir_vector
    }

    pub fn get_transform(&self) -> Transform {
        Transform::from_translation(self.value)
    }
}


#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub rigid_body: RigidBody,
    pub gravity_scale: GravityScale,
    pub collider: Collider,
    pub sleeping: Sleeping,
    pub ccd: Ccd,
    pub active_events: ActiveEvents,
}


impl Default for MovingObjectBundle {
    fn default() -> Self {
        Self {
            // Custom default values
            rigid_body: RigidBody::Dynamic,
            gravity_scale: GravityScale(0.),
            sleeping: Sleeping::disabled(),
            ccd: Ccd::enabled(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            // This is the part you usually want to change
            velocity: Velocity::default(),
            collider: Collider::default(),
        }
    }
}

