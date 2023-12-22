use bevy::prelude::*;

#[derive(Resource)]
pub struct UserSettings {
    pub show_fps: bool,
    pub orbit_sensitivity: f32,
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,

    // TODO camera keybinds
}