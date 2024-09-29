use bevy::prelude::*;
use bevy::window::*;

pub mod camera;
pub mod dev;
pub mod gamepad;
pub mod motion;
pub mod tilemap;
pub mod time;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    present_mode: PresentMode::AutoNoVsync,
                    title: "Spirit of Lira".into(),
                    resizable: true,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .build(),
    )
    .insert_resource(Msaa::Off)
    .init_resource::<time::WorldTime>()
    .add_plugins((
        camera::CameraPlugin,
        gamepad::GamepadPlugin,
        motion::MotionPlugin,
        dev::DevPlugin,
        tilemap::TilemapPlugin,
    ));
    app.run();
}
