mod camera;
mod debug;
mod sand;

use bevy::prelude::*;

use camera::CameraPlugin;
#[cfg(feature = "debug")]
use debug::DebugPlugin;
use sand::SandPlugin;

fn main() {
    let mut app = App::new();

    // Bevy builtins
    app.add_plugins(DefaultPlugins);

    // User defined
    app.add_plugins(CameraPlugin).add_plugins(SandPlugin);

    #[cfg(feature = "debug")]
    app.add_plugins(DebugPlugin);

    app.run();
}
