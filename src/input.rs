use bevy::{prelude::*, window::PrimaryWindow};

use crate::{sand::SpawnSandEvent, schedule::RunningSet};

const SPAWN_SAND_EVENT_CD_SECONDS: f32 = 0.05;

#[derive(Resource, Debug, Default)]
struct SpawnSandEventCdTimer {
    timer: Timer,
}

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnSandEventCdTimer {
            timer: Timer::from_seconds(SPAWN_SAND_EVENT_CD_SECONDS, TimerMode::Once),
        })
        .add_systems(Update, mouse_input.in_set(RunningSet::UserInput));
    }
}

fn mouse_input(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    input: Res<ButtonInput<MouseButton>>,
    mut spawn_sand_event_writer: EventWriter<SpawnSandEvent>,
    mut cd_timer: ResMut<SpawnSandEventCdTimer>,
    time: Res<Time>,
) {
    cd_timer.timer.tick(time.delta());

    if input.pressed(MouseButton::Left) {
        if !(cd_timer.timer.finished() || input.just_pressed(MouseButton::Left)) {
            return;
        }

        let Ok(window) = window_query.get_single() else {
            return;
        };

        let Some(window_position) = window.cursor_position() else {
            return;
        };

        let Ok((camera, camera_transform)) = camera_query.get_single() else {
            return;
        };

        let Some(world_position) = camera.viewport_to_world_2d(camera_transform, window_position)
        else {
            return;
        };

        spawn_sand_event_writer.send(SpawnSandEvent {
            location: world_position,
        });
        cd_timer.timer.reset();
    }
}
