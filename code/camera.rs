use crate::*;
// use bevy::input::mouse::MouseWheel;
use bevy::{
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
    window::WindowResized,
};

const TEST_COLOR: Color = Color::srgb(0.7, 0.3, 0.5);

const RES_SCALE: u32 = 24;
const RES_WIDTH: u32 = 16 * RES_SCALE;
const RES_HEIGHT: u32 = 9 * RES_SCALE;

const ZOOM: f32 = 1.0;
// const ZOOM_MIN: f32 = 0.5;
// const ZOOM_MAX: f32 = 2.0;
// const ZOOM_SPEED: f32 = 1.0;

pub const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
pub const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

#[derive(Resource)]
pub struct CameraResource {
    pub in_game_camera: Entity,
    pub outer_camera: Entity,
    pub canvas: Entity,
    pub zoom: f32,
    pub followed_entity: Option<Entity>,
}

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct OuterCamera;

#[derive(Component)]
struct Canvas;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cameras).add_systems(
            Update,
            (
                // zoom,
                follow.after(motion::update_position),
                fit_canvas,
            ),
        );
    }
}

pub fn setup_cameras(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    let in_game_camera = commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(image_handle.clone()),
                    ..default()
                },
                ..default()
            },
            InGameCamera,
            PIXEL_PERFECT_LAYERS,
        ))
        .id();

    let canvas_entity = commands
        .spawn((
            SpriteBundle {
                texture: image_handle,
                ..default()
            },
            Canvas,
            HIGH_RES_LAYERS,
        ))
        .id();

    let outer_camera = commands
        .spawn((Camera2dBundle::default(), OuterCamera, HIGH_RES_LAYERS))
        .id();

    // Spawn a circle for the outer camera (high-res layer)
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(10.0)).into(),
            material: materials.add(ColorMaterial::from(TEST_COLOR)),
            transform: Transform::from_translation(Vec3::new(
                100.0, 100.0, 0.0,
            )),
            ..default()
        },
        HIGH_RES_LAYERS,
    ));

    commands.insert_resource(CameraResource {
        in_game_camera,
        outer_camera,
        canvas: canvas_entity,
        zoom: ZOOM,
        followed_entity: None,
    });
}

// fn zoom(
//     mut camera_resource: ResMut<CameraResource>,
//     mut transforms: Query<&mut Transform>,
//     mut mouse_wheel_events: EventReader<MouseWheel>,
//     time: Res<Time>,
// ) {
//     let adjusted_zoom_speed = ZOOM_SPEED * time.delta_seconds();
//     for event in mouse_wheel_events.read() {
//         camera_resource.zoom *= 1.0 - event.y * 2.0 * adjusted_zoom_speed;
//     }
//     camera_resource.zoom = camera_resource.zoom.clamp(ZOOM_MIN, ZOOM_MAX);
//     if let Ok(mut transform) =
//         transforms.get_mut(camera_resource.in_game_camera)
//     {
//         transform.scale = Vec3::splat(camera_resource.zoom);
//     }
// }

fn follow(
    camera: ResMut<CameraResource>,
    mut transforms: Query<&mut Transform>,
) {
    let mut followed_transform: Option<Transform> = None;
    if let Some(followed_entity) = camera.followed_entity {
        if let Ok(transform) = transforms.get_mut(followed_entity) {
            followed_transform = Some(transform.clone());
        }
    }
    if let Some(transform) = followed_transform {
        if let Ok(mut camera_transform) =
            transforms.get_mut(camera.in_game_camera)
        {
            camera_transform.translation.x = transform.translation.x;
            camera_transform.translation.y = transform.translation.y;
        }
    }
}

fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut transforms: Query<&mut Transform, With<Canvas>>,
) {
    for event in resize_events.read() {
        let h_scale = event.width / RES_WIDTH as f32;
        let v_scale = event.height / RES_HEIGHT as f32;
        let scale = h_scale.min(v_scale).floor();
        if let Ok(mut transform) = transforms.get_single_mut() {
            transform.scale = Vec3::splat(scale);
        }
    }
}
