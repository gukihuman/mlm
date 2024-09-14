use bevy::input::gamepad::{GamepadAxisType, GamepadEvent};
use bevy::prelude::*;
pub struct GamepadPlugin;
impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GamepadState>()
            .add_systems(Update, update_gamepad_state);
    }
}
#[derive(Resource, Debug)]
pub struct GamepadState {
    pub deadzone: f32,
    pub left_stick_deadzone_exceed: bool,
    pub left_stick_x: f32,
    pub left_stick_y: f32,
    pub right_stick_deadzone_exceed: bool,
    pub right_stick_x: f32,
    pub right_stick_y: f32,
}
impl Default for GamepadState {
    fn default() -> Self {
        Self {
            deadzone: 0.1,
            left_stick_deadzone_exceed: false,
            left_stick_x: 0.,
            left_stick_y: 0.,
            right_stick_deadzone_exceed: false,
            right_stick_x: 0.,
            right_stick_y: 0.,
        }
    }
}
pub fn update_gamepad_state(
    mut gamepad_state: ResMut<GamepadState>,
    mut gamepad_event_reader: EventReader<GamepadEvent>,
) {
    for event in gamepad_event_reader.read() {
        if let GamepadEvent::Axis(axis_event) = event {
            match axis_event.axis_type {
                GamepadAxisType::LeftStickX => {
                    gamepad_state.left_stick_deadzone_exceed = true;
                    gamepad_state.left_stick_x = axis_event.value
                }
                GamepadAxisType::LeftStickY => {
                    gamepad_state.left_stick_deadzone_exceed = true;
                    gamepad_state.left_stick_y = axis_event.value
                }
                GamepadAxisType::RightStickX => {
                    gamepad_state.right_stick_deadzone_exceed = true;
                    gamepad_state.right_stick_x = axis_event.value
                }
                GamepadAxisType::RightStickY => {
                    gamepad_state.right_stick_deadzone_exceed = true;
                    gamepad_state.right_stick_y = axis_event.value
                }
                _ => {}
            }
        }
    }
    let left_stick_magnitude = (gamepad_state.left_stick_x.powi(2)
        + gamepad_state.left_stick_y.powi(2))
    .sqrt();
    let right_stick_magnitude = (gamepad_state.right_stick_x.powi(2)
        + gamepad_state.right_stick_y.powi(2))
    .sqrt();
    if left_stick_magnitude < gamepad_state.deadzone {
        gamepad_state.left_stick_deadzone_exceed = false;
        gamepad_state.left_stick_x = 0.0;
        gamepad_state.left_stick_y = 0.0;
    }
    if right_stick_magnitude < gamepad_state.deadzone {
        gamepad_state.right_stick_deadzone_exceed = false;
        gamepad_state.right_stick_x = 0.0;
        gamepad_state.right_stick_y = 0.0;
    }
}
