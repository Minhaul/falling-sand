use bevy::prelude::*;

use crate::state::GameState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum RunningSet {
    UserInput,
    SpawnEntities,
    EntityUpdates,
    CollisionDetection,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                RunningSet::UserInput,
                RunningSet::SpawnEntities,
                RunningSet::EntityUpdates,
                RunningSet::CollisionDetection,
            )
                .chain()
                .run_if(in_state(GameState::Running)),
        );
    }
}
