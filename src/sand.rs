use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    collision_detection::Collider,
    physics::{Acceleration, MovingObjectBundle, Velocity},
    schedule::RunningSet,
    world_grid::{AliasAxis, GlobalAlias, LocalAlias},
};

/// Grains of sand are squares, how big is an edge?
///
/// Pretty sure this will break if not 1.0
const SAND_GRAIN_EDGE_SIZE: f32 = 1.0;

/// Marker component for a grain of sand.
#[derive(Component, Debug)]
pub struct SandGrain;

/// Event that will spawn sand when received
#[derive(Event, Debug)]
pub struct SpawnSandEvent {
    pub location: Vec2,
}

/// Plugin for spawning sand and defining its behavior
pub struct SandPlugin;

impl Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSandEvent>()
            .add_systems(Update, spawn_sand.in_set(RunningSet::SpawnEntities));
    }
}

fn spawn_sand(
    mut spawn_sand_event_reader: EventReader<SpawnSandEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for spawn_sand_event in spawn_sand_event_reader.read() {
        let location = spawn_sand_event.location;
        commands
            .spawn((
                MovingObjectBundle {
                    velocity: Velocity::new(Vec2::ZERO),
                    acceleration: Acceleration::new(Vec2::new(0.0, -1.0)),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                        location.x, location.y, 0.0,
                    ))),
                },
                SandGrain,
                GlobalAlias::new(AliasAxis::All),
            ))
            .with_children(|parent| {
                parent.spawn((
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(
                            meshes.add(Rectangle::new(SAND_GRAIN_EDGE_SIZE, SAND_GRAIN_EDGE_SIZE)),
                        ),
                        material: materials.add(Color::rgb(0.5, 1.0, 0.5)),
                        transform: Transform::from_translation(Vec3::ZERO),
                        ..default()
                    },
                    // Collision will happen based on the visual mesh's location,
                    // not the underlying moving physics object's location.
                    Collider::new(SAND_GRAIN_EDGE_SIZE),
                    LocalAlias,
                ));
            });
    }
}
