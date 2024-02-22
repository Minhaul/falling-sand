use bevy::{prelude::*, window::WindowResized};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_window_resolution);
    }
}

fn print_window_resolution(mut resized_event_reader: EventReader<WindowResized>) {
    for resized_event in resized_event_reader.read() {
        info!(
            "Window resized to {:?}x{:?}",
            resized_event.width, resized_event.height
        );
    }
}
