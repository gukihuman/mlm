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
    pub ls_x: f32,
    pub ls_y: f32,
    pub rs_x: f32,
    pub rs_y: f32,
    pub deadzone: f32,
    pub ls_deadzone_exceed: bool,
    pub rs_deadzone_exceed: bool,
}
impl Default for GamepadState {
    fn default() -> Self {
        Self {
            ls_x: 0.,
            ls_y: 0.,
            rs_x: 0.,
            rs_y: 0.,
            deadzone: 0.1,
            ls_deadzone_exceed: false,
            rs_deadzone_exceed: false,
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
                    gamepad_state.ls_deadzone_exceed = true;
                    gamepad_state.ls_x = axis_event.value
                }
                GamepadAxisType::LeftStickY => {
                    gamepad_state.ls_deadzone_exceed = true;
                    gamepad_state.ls_y = axis_event.value
                }
                GamepadAxisType::RightStickX => {
                    gamepad_state.rs_deadzone_exceed = true;
                    gamepad_state.rs_x = axis_event.value
                }
                GamepadAxisType::RightStickY => {
                    gamepad_state.rs_deadzone_exceed = true;
                    gamepad_state.rs_y = axis_event.value
                }
                _ => {}
            }
        }
    }
    let ls_magnitude = (gamepad_state.ls_x.powi(2) + gamepad_state.ls_y.powi(2)).sqrt();
    let rs_magnitude = (gamepad_state.rs_x.powi(2) + gamepad_state.rs_y.powi(2)).sqrt();
    if ls_magnitude < gamepad_state.deadzone {
        gamepad_state.ls_deadzone_exceed = false;
        gamepad_state.ls_x = 0.0;
        gamepad_state.ls_y = 0.0;
    }
    if rs_magnitude < gamepad_state.deadzone {
        gamepad_state.rs_deadzone_exceed = false;
        gamepad_state.rs_x = 0.0;
        gamepad_state.rs_y = 0.0;
    }
}
