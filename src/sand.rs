use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::physics::{Acceleration, MovingObjectBundle, Velocity};

/// Grains of sand are squares, how big is an edge?
const SAND_GRAIN_EDGE_SIZE: f32 = 2.0;

// Marker component for a grain of sand.
#[derive(Component, Debug)]
pub struct SandGrain;

pub struct SandPlugin;

impl Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_sand_grain);
    }
}

fn spawn_sand_grain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            MovingObjectBundle {
                velocity: Velocity::new(Vec2::ZERO),
                acceleration: Acceleration::new(Vec2::new(0.0, -1.0)),
                spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
            },
            SandGrain,
        ))
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(
                    SAND_GRAIN_EDGE_SIZE / 2.0,
                    SAND_GRAIN_EDGE_SIZE / 2.0,
                ))),
                material: materials.add(Color::rgb(0.5, 1.0, 0.5)),
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            });
        });
}
