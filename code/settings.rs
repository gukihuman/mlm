use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowTheme};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Resource, Deserialize, Debug, Clone)]
pub struct GameSettings {
    pub window: WindowSettings,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WindowSettings {
    pub mode: String,
    pub vsync: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            window: WindowSettings {
                mode: "BorderlessFullscreen".to_string(),
                vsync: false,
            },
        }
    }
}

pub fn load_settings() -> GameSettings {
    let config_path = Path::new("settings.toml");

    if let Ok(contents) = fs::read_to_string(config_path) {
        if let Ok(settings) = toml::from_str(&contents) {
            return settings;
        } else {
            println!("Failed to parse settings.toml, using defaults");
        }
    } else {
        println!("No settings.toml found, using defaults");
    }

    GameSettings::default()
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let settings = load_settings();
        app.insert_resource(settings);
    }
}

pub fn apply_window_settings(settings: GameSettings) -> WindowPlugin {
    let mode = match settings.window.mode.as_str() {
        "BorderlessFullscreen" => WindowMode::BorderlessFullscreen,
        "Fullscreen" => WindowMode::Fullscreen,
        "Windowed" => WindowMode::Windowed,
        _ => WindowMode::BorderlessFullscreen,
    };

    let present_mode = if settings.window.vsync {
        PresentMode::AutoVsync
    } else {
        PresentMode::AutoNoVsync
    };

    WindowPlugin {
        primary_window: Some(Window {
            mode,
            present_mode,
            title: "Mommy's Little Monsters".into(),
            resizable: true,
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    }
}
