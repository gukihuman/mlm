use crate::*;
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

pub const PIXEL_LAYER: RenderLayers = RenderLayers::layer(0);
pub const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

const PIXEL_SIZE: f32 = 2.5;

const TEST_COLOR: Color = Color::srgb(0.7, 0.3, 0.5);

#[derive(Resource)]
pub struct CameraResource {
    pub pixel_camera: Entity,
    pub outer_camera: Entity,
    pub canvas: Entity,
    pub canvas_image: Handle<Image>,
    // pub zoom: f32,
    pub pixel_size: f32,
    pub followed_entity: Option<Entity>,
    pub pixel_w: u32,
    pub pixel_h: u32,
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
            (follow.after(motion::update_position), fit_canvas),
        );
    }
}

pub fn setup_cameras(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&mut Window>,
) {
    let window_w = windows.single().resolution.width();
    let window_h = windows.single().resolution.height();

    let pixel_w = (window_w / PIXEL_SIZE).floor() as u32;
    let pixel_h = (window_h / PIXEL_SIZE).floor() as u32;

    let canvas_size = Extent3d {
        width: pixel_w,
        height: pixel_h,
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

    let pixel_camera = commands
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
            PIXEL_LAYER,
        ))
        .id();

    let canvas_entity = commands
        .spawn((
            SpriteBundle {
                texture: image_handle.clone(),
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
        pixel_camera,
        outer_camera,
        canvas: canvas_entity,
        canvas_image: image_handle.clone(),
        // zoom: ZOOM,
        pixel_size: PIXEL_SIZE,
        followed_entity: None,
        pixel_w,
        pixel_h,
    });
}

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
            transforms.get_mut(camera.pixel_camera)
        {
            camera_transform.translation.x = transform.translation.x;
            camera_transform.translation.y = transform.translation.y;
        }
    }
}

fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut transforms: Query<&mut Transform, With<Canvas>>,
    mut images: ResMut<Assets<Image>>,
    mut camera_resource: ResMut<CameraResource>,
    mut in_game_camera_query: Query<
        &mut OrthographicProjection,
        With<InGameCamera>,
    >,
) {
    for event in resize_events.read() {
        let new_pixel_width =
            (event.width / camera_resource.pixel_size as f32).floor() as u32;
        let new_pixel_height =
            (event.height / camera_resource.pixel_size as f32).floor() as u32;

        // Update the canvas size
        if let Some(canvas_image) =
            images.get_mut(&camera_resource.canvas_image)
        {
            canvas_image.resize(Extent3d {
                width: new_pixel_width,
                height: new_pixel_height,
                ..default()
            });
        }

        // Update the camera resource
        camera_resource.pixel_w = new_pixel_width;
        camera_resource.pixel_h = new_pixel_height;

        // Adjust the canvas transform to fill the window
        if let Ok(mut transform) = transforms.get_single_mut() {
            transform.scale = Vec3::new(
                camera_resource.pixel_size as f32,
                camera_resource.pixel_size as f32,
                1.0,
            );
        }

        // Adjust the in-game camera's orthographic projection
        if let Ok(mut projection) = in_game_camera_query.get_single_mut() {
            projection.area = Rect::from_center_size(
                Vec2::ZERO,
                Vec2::new(new_pixel_width as f32, new_pixel_height as f32),
            );
        }
    }
}
