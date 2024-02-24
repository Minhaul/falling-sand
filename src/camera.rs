use bevy::{prelude::*, render::camera::ScalingMode};

/// How many pixels equals one world unit
const PIXEL_SCALE: f32 = 2.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    // Manually create the default here because some inner struct defaults are
    // different than the defaults that the Camera2dBundle sets
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::WindowSize(PIXEL_SCALE);
    commands.spawn(camera);
}
