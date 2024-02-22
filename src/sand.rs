use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

// Marker component for a grain of sand.
#[derive(Component, Debug)]
pub struct SandGrain;

pub struct SandPlugin;

impl Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sand_grain);
    }
}

fn spawn_sand_grain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(1.0, 1.0))),
            material: materials.add(Color::rgb(0.2, 0.5, 0.2)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        SandGrain,
    ));
}
