use crate::*;
use bevy::sprite::MaterialMesh2dBundle;

const GRID_SIZE: u32 = 100;
const GRID_CELL_SIZE: f32 = 10.;

const ANIMATION_FPS: f32 = 6.0;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_lira.after(camera::setup_cameras), spawn_grid),
        )
        .add_systems(Update, animate_sprite);
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
                camera::PIXEL_LAYER,
            ));
        }
    }
}

fn spawn_lira(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut camera_resource: ResMut<camera::CameraResource>,
    mut controlled_entity: ResMut<motion::ControlledEntity>,
) {
    let texture = asset_server.load("lira/idle.png");
    let layout =
        TextureAtlasLayout::from_grid(UVec2::new(75, 75), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices { first: 0, last: 7 };

    let lira = commands
        .spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_xyz(0., 0., 10.),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(
                1.0 / ANIMATION_FPS,
                TimerMode::Repeating,
            )),
            motion::Movement::default(),
            camera::PIXEL_LAYER,
        ))
        .id();

    camera_resource.followed_entity = Some(lira);
    controlled_entity.0 = Some(lira);
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
    )>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
