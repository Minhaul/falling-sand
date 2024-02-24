mod camera;
mod collision_detection;
mod debug;
mod physics;
mod sand;
mod schedule;
mod state;

use bevy::prelude::*;

use camera::CameraPlugin;
#[cfg(feature = "debug")]
use debug::DebugPlugin;
use physics::PhysicsPlugin;
use sand::SandPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;

fn main() {
    let mut app = App::new();

    // Bevy builtins
    app.add_plugins(DefaultPlugins);

    // User defined
    app.add_plugins(CameraPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(SandPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin);

    #[cfg(feature = "debug")]
    app.add_plugins(DebugPlugin);

    app.run();
}
