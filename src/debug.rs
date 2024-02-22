use bevy::{prelude::*, render::camera::ScalingMode, window::WindowResized};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_window_resolution);
    }
}

fn print_window_resolution(
    mut resized_event_reader: EventReader<WindowResized>,
    query: Query<&OrthographicProjection>,
) {
    for resized_event in resized_event_reader.read() {
        info!(
            "Window resized to {:?}x{:?}",
            resized_event.width, resized_event.height,
        );

        if let Ok(projection) = query.get_single() {
            info!("World area: {:?}", projection.area);
            if let ScalingMode::WindowSize(pixel_scale) = projection.scaling_mode {
                info!(
                    "World size: {:?}x{:?}",
                    resized_event.width / pixel_scale,
                    resized_event.height / pixel_scale
                );
            }
        }
    }
}
