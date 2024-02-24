use bevy::{
    prelude::*, transform::systems::propagate_transforms, utils::hashbrown::HashMap,
    window::WindowResized,
};

use crate::schedule::RunningSet;

#[derive(Debug, Hash, PartialEq, Eq, Default, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct GridSlot {
    entity: Option<Entity>,
}

// For now, (0, 0) will always be a valid slot, and grid slots will only expand on resize if
// there's room for two more, one on each side of the existing grid, keeping the middle grid slot
// at (0,0). Also the grid will be 1:1 with world units, no scaling (for now at least).
#[derive(Resource, Debug, Default)]
struct WorldGrid {
    grid: HashMap<Coord, GridSlot>,
    extents: (Coord, Coord),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AliasAxis {
    #[default]
    All,
    X,
    Y,
}

#[derive(Component, Debug)]
pub struct GlobalAlias {
    pub axis: AliasAxis,
}

impl GlobalAlias {
    pub fn new(axis: AliasAxis) -> Self {
        Self { axis }
    }
}

#[derive(Component, Debug)]
pub struct LocalAlias;

pub struct WorldGridPlugin;

impl Plugin for WorldGridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldGrid>()
            .add_systems(PostStartup, create_grid)
            .add_systems(
                Update,
                (
                    update_grid,
                    alias_global_location,
                    propagate_transforms,
                    alias_local_location,
                    propagate_transforms,
                )
                    .chain()
                    .in_set(RunningSet::EntityUpdates),
            );
    }
}

fn create_grid(mut world_grid: ResMut<WorldGrid>, query: Query<&OrthographicProjection>) {
    let Ok(projection) = query.get_single() else {
        panic!("NO PROJECTION??????");
    };

    let (neg_extent, pos_extent) = prv_get_extents(&projection.area);

    for y in neg_extent.y..=pos_extent.y {
        for x in neg_extent.x..=pos_extent.x {
            world_grid
                .grid
                .insert(Coord { x, y }, GridSlot { entity: None });
        }
    }

    world_grid.extents = (neg_extent, pos_extent);
}

fn update_grid(
    mut resized_event_reader: EventReader<WindowResized>,
    mut world_grid: ResMut<WorldGrid>,
    query: Query<&OrthographicProjection>,
    mut _gizmos: Gizmos,
) {
    #[cfg(feature = "gizmos")]
    {
        for (coord, _) in world_grid.grid.iter() {
            _gizmos.rect_2d(
                Vec2::new(coord.x as f32, coord.y as f32),
                0.0,
                Vec2::splat(1.0),
                Color::GREEN,
            );
        }
    }
    if resized_event_reader.is_empty() {
        return;
    }
    resized_event_reader.clear();

    let Ok(projection) = query.get_single() else {
        panic!("NO PROJECTION??????");
    };

    let (neg_extent, pos_extent) = prv_get_extents(&projection.area);

    // Nothing to be done if the extents haven't changed
    if world_grid.extents == (neg_extent, pos_extent) {
        return;
    }

    let min_neg_extent = Coord {
        x: neg_extent.x.min(world_grid.extents.0.x),
        y: neg_extent.y.min(world_grid.extents.0.y),
    };

    let max_pos_extent = Coord {
        x: pos_extent.x.max(world_grid.extents.1.x),
        y: pos_extent.y.max(world_grid.extents.1.y),
    };

    for y in min_neg_extent.y..=max_pos_extent.y {
        for x in min_neg_extent.x..=max_pos_extent.x {
            if x < neg_extent.x || x > pos_extent.x || y < neg_extent.y || y > pos_extent.y {
                // TODO: despawn entity if there is one
                world_grid.grid.remove(&Coord { x, y });
            } else {
                let _ = world_grid
                    .grid
                    .try_insert(Coord { x, y }, GridSlot { entity: None });
            }
        }
    }

    world_grid.extents = (neg_extent, pos_extent);
}

fn alias_global_location(
    mut commands: Commands,
    mut world_grid: ResMut<WorldGrid>,
    mut query: Query<(Entity, &mut Transform, &GlobalAlias)>,
) {
    for (entity, mut transform, GlobalAlias { axis }) in query.iter_mut() {
        match axis {
            AliasAxis::All => transform.translation = transform.translation.round(),
            AliasAxis::X => transform.translation.x = transform.translation.x.round(),
            AliasAxis::Y => transform.translation.y = transform.translation.y.round(),
        }

        let curr_location = Coord {
            x: transform.translation.x as isize,
            y: transform.translation.y as isize,
        };

        if prv_location_valid(&world_grid, &curr_location) {
            // TODO: check if this spot is already taken and do ...something?
            world_grid.grid.insert(
                curr_location,
                GridSlot {
                    entity: Some(entity),
                },
            );
        } else {
            // Outside the bounds of the world grid, TODO: despawn?
        }

        commands.entity(entity).remove::<GlobalAlias>();
    }
}

fn alias_local_location(
    world_grid: Res<WorldGrid>,
    parent_query: Query<&Transform, (With<Children>, Without<LocalAlias>)>,
    mut child_query: Query<(&mut Transform, &Parent), With<LocalAlias>>,
) {
    for (mut transform, parent) in child_query.iter_mut() {
        let Ok(parent_transform) = parent_query.get(parent.get()) else {
            panic!("INVALID USE OF LocalAlias");
        };

        let closest_slot = Coord {
            x: parent_transform.translation.x.round() as isize,
            y: parent_transform.translation.y.round() as isize,
        };

        if prv_location_valid(&world_grid, &closest_slot) {
            transform.translation.x = closest_slot.x as f32 - parent_transform.translation.x;
            transform.translation.y = closest_slot.y as f32 - parent_transform.translation.y;
        } else {
            // Outside the bounds of the world grid, TODO: despawn?
        }
    }
}

fn prv_get_extents(area: &Rect) -> (Coord, Coord) {
    let neg_extent = Coord {
        x: (area.min.x + 0.5).ceil() as isize,
        y: (area.min.y + 0.5).ceil() as isize,
    };

    let pos_extent = Coord {
        x: (area.max.x - 0.5).floor() as isize,
        y: (area.max.y - 0.5).floor() as isize,
    };

    (neg_extent, pos_extent)
}

fn prv_location_valid(grid: &WorldGrid, coord: &Coord) -> bool {
    coord.x >= grid.extents.0.x
        && coord.y >= grid.extents.0.y
        && coord.x <= grid.extents.1.x
        && coord.y <= grid.extents.1.y
}
