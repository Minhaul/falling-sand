use bevy::{prelude::*, render::camera::ScalingMode, window::WindowResized};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(feature = "gizmos"))]
        app.add_systems(Startup, disable_gizmos);
        #[cfg(feature = "gizmos")]
        app.add_systems(Startup, configure_gizmos);
        #[cfg(feature = "debug")]
        app.add_systems(Update, print_window_resolution);
    }
}

#[allow(unused)]
fn disable_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();

    config.enabled = false;
}

#[allow(unused)]
fn configure_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();

    config.line_width = 1.0;
}

#[allow(unused)]
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
