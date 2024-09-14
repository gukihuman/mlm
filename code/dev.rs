use crate::*;
use bevy::sprite::MaterialMesh2dBundle;

// const MIDDLE_POINT_Z_INDEX: f32 = 20.0;
// const MIDDLE_RADIUS: f32 = 2.;
// const MIDDLE_COLOR: Color = Color::srgb(1., 0.4, 0.7);

// const CROSSHAIR_Z_INDEX: f32 = 999.;
// const CROSSHAIR_RADIUS: f32 = 2.5;
// const CROSSHAIR_COLOR: Color = Color::srgb(0.7, 0.9, 0.9);

const GRID_SIZE: u32 = 100; // Adjust the size of the grid as needed
const GRID_CELL_SIZE: f32 = 10.; // Adjust the size of each cell

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_lira.after(camera::setup_cameras),
                // spawn_crosshair.after(camera::setup_cameras),
                // spawn_middle_point,
                spawn_grid,
            ),
        );
    }
}

pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let color = if (x + y) % 2 == 0 {
                Color::srgb(0.5, 0.5, 0.5)
            } else {
                Color::srgb(0.4, 0.4, 0.4)
            };

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Rectangle::from_size(Vec2::splat(GRID_CELL_SIZE)))
                        .into(),
                    material: materials.add(ColorMaterial::from(color)),
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * GRID_CELL_SIZE
                            - (GRID_SIZE as f32 * GRID_CELL_SIZE) / 2.,
                        y as f32 * GRID_CELL_SIZE
                            - (GRID_SIZE as f32 * GRID_CELL_SIZE) / 2.,
                        0., // Z-index can be adjusted as needed
                    )),
                    ..default()
                },
                camera::PIXEL_PERFECT_LAYERS,
            ));
        }
    }
}

fn spawn_lira(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_resource: ResMut<camera::CameraResource>,
    mut controlled_entity: ResMut<motion::ControlledEntity>,
) {
    let lira = commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("lira_idle.png"),
                transform: Transform::from_xyz(-40., 20., 2.),
                ..default()
            },
            motion::Movement::default(),
            camera::PIXEL_PERFECT_LAYERS,
        ))
        .id();
    camera_resource.followed_entity = Some(lira);
    controlled_entity.0 = Some(lira);
}

// pub fn spawn_crosshair(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut camera_resource: ResMut<camera::CameraResource>,
//     mut controlled_entity: ResMut<motion::ControlledEntity>,
// ) {
//     let crosshair = commands
//         .spawn((
//             MaterialMesh2dBundle {
//                 mesh: meshes.add(Circle::new(CROSSHAIR_RADIUS)).into(),
//                 material: materials.add(ColorMaterial::from(CROSSHAIR_COLOR)),
//                 transform: Transform::from_translation(Vec3::new(
//                     0.,
//                     0.,
//                     CROSSHAIR_Z_INDEX,
//                 )),
//                 ..default()
//             },
//             motion::Movement::default(),
//             camera::PIXEL_PERFECT_LAYERS,
//         ))
//         .id();
//     camera_resource.followed_entity = Some(crosshair);
//     controlled_entity.0 = Some(crosshair);
// }

// pub fn spawn_middle_point(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn((
//         MaterialMesh2dBundle {
//             mesh: meshes.add(Circle::new(MIDDLE_RADIUS)).into(),
//             material: materials.add(ColorMaterial::from(MIDDLE_COLOR)),
//             transform: Transform::from_translation(Vec3::new(
//                 0.,
//                 0.,
//                 MIDDLE_POINT_Z_INDEX,
//             )),
//             ..default()
//         },
//         motion::Movement::default(),
//         camera::PIXEL_PERFECT_LAYERS,
//     ));
// }
