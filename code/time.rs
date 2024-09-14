use bevy::prelude::*;

#[derive(Resource)]
pub struct WorldTime {
    pub delta_multiplier: f32,
    pub paused: bool,
}
impl Default for WorldTime {
    fn default() -> Self {
        Self {
            delta_multiplier: 1.,
            paused: false,
        }
    }
}
