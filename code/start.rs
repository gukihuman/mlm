use bevy::prelude::*;

pub mod camera;
pub mod dev;
pub mod gamepad;
pub mod movement;
pub mod settings;
pub mod tiles {
    pub mod tile_generation;
    pub mod tilemap;
}
pub mod time;

fn main() {
    let mut app = App::new();

    app.add_plugins(settings::SettingsPlugin);

    let settings = app.world().resource::<settings::GameSettings>();
    let window_plugin = settings::apply_window_settings(settings.clone());

    app.add_plugins(
        DefaultPlugins
            .set(window_plugin)
            .set(ImagePlugin::default_nearest())
            .build(),
    )
    .insert_resource(Msaa::Off)
    .init_resource::<time::WorldTime>()
    .add_plugins((
        camera::CameraPlugin,
        gamepad::GamepadPlugin,
        movement::MovementPlugin,
        dev::DevPlugin,
        tiles::tilemap::TilemapPlugin,
    ));
    app.run();
}
