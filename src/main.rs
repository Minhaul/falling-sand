mod camera;
mod collision_detection;
mod debug;
mod physics;
mod sand;
mod schedule;
mod state;
mod world_grid;

use bevy::prelude::*;

use camera::CameraPlugin;
use debug::DebugPlugin;
use physics::PhysicsPlugin;
use sand::SandPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;
use world_grid::WorldGridPlugin;

fn main() {
    let mut app = App::new();

    // Bevy builtins
    app.add_plugins(DefaultPlugins);

    // User defined
    app.add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(SandPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(WorldGridPlugin);

    app.run();
}
