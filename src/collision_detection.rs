use bevy::prelude::*;

// TODO
#[derive(Component, Debug)]
pub struct Collider {
    pub half_extents: f32,
}

impl Collider {
    pub fn new(half_extents: f32) -> Self {
        Self { half_extents }
    }
}
