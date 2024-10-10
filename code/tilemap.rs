use crate::*;
use bevy::ecs::system::ParamSet;
use bevy::sprite::MaterialMesh2dBundle;
use camera::HIGH_RES_LAYERS;
use rand::Rng;

const TILE_SIZE_X: f32 = 16.0;
const TILE_SIZE_Y: f32 = 8.0;
const TILE_COUNT_X: u32 = 4;
const TILE_COUNT_Y: u32 = 2;
const MAP_WIDTH: u32 = 50;
const MAP_HEIGHT: u32 = 30;
const MINIMAP_SCALE: f32 = 0.3;
const MINIMAP_SCREEN_X: f32 = 0.0;
const MINIMAP_SCREEN_Y: f32 = 0.0;
const MINIMAP_SCREEN_PERCENT_X: f32 = 0.00; // 5% from the left edge
const MINIMAP_SCREEN_PERCENT_Y: f32 = 0.00; // 5% from the top edge

const CIRCLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.5);

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct MinimapTile;

#[derive(Component)]
struct PlayerMinimapIcon;

#[derive(Component)]
struct MinimapParent;

#[derive(Resource, Default)]
pub struct Tilemap {
    pub tiles: Vec<Vec<TileType>>,
}

#[derive(Resource)]
pub struct MinimapState {
    pub visible: bool,
}

impl Default for MinimapState {
    fn default() -> Self {
        Self { visible: false }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Stone,
    Grass,
    Water,
    Dirt,
}

impl TileType {
    fn to_index(&self) -> usize {
        match self {
            TileType::Stone => 0,
            TileType::Grass => 1,
            TileType::Water => 2,
            TileType::Dirt => 3,
        }
    }

    fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => TileType::Stone,
            1 => TileType::Grass,
            2 => TileType::Water,
            _ => TileType::Dirt,
        }
    }
}

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Tilemap>()
            .init_resource::<MinimapState>()
            .add_systems(Startup, setup_tilemap)
            .add_systems(Startup, spawn_tiles.after(setup_tilemap))
            .add_systems(Startup, spawn_minimap_tiles.after(setup_tilemap))
            .add_systems(
                Update,
                (
                    toggle_minimap,
                    update_minimap_visibility,
                    update_minimap_content,
                ),
            );
    }
}

fn setup_tilemap(mut tilemap: ResMut<Tilemap>) {
    tilemap.tiles = (0..MAP_HEIGHT)
        .map(|_| (0..MAP_WIDTH).map(|_| TileType::random()).collect())
        .collect();
}

fn spawn_tiles(
    mut commands: Commands,
    tilemap: Res<Tilemap>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("tiles.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_SIZE_X as u32, TILE_SIZE_Y as u32),
        TILE_COUNT_X,
        TILE_COUNT_Y,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for (y, row) in tilemap.tiles.iter().enumerate() {
        for (x, tile_type) in row.iter().enumerate() {
            commands.spawn((
                SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            (x as f32 - y as f32) * TILE_SIZE_X / 2.0,
                            -(x as f32 + y as f32) * TILE_SIZE_Y / 2.0,
                            50.0,
                        ),
                        scale: Vec3::splat(1.0),
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_handle.clone(),
                    index: tile_type.to_index(),
                },
                Tile,
            ));
        }
    }
}

fn spawn_minimap_tiles(
    mut commands: Commands,
    tilemap: Res<Tilemap>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("tiles.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_SIZE_X as u32, TILE_SIZE_Y as u32),
        TILE_COUNT_X,
        TILE_COUNT_Y,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Create a parent entity for all minimap tiles
    let minimap_parent = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(MINIMAP_SCREEN_X, MINIMAP_SCREEN_Y, 100.0),
                visibility: Visibility::Hidden,
                ..default()
            },
            MinimapParent,
            HIGH_RES_LAYERS, // Use the high res layer for the outer camera
        ))
        .id();

    for (y, row) in tilemap.tiles.iter().enumerate() {
        for (x, tile_type) in row.iter().enumerate() {
            let child = commands
                .spawn((
                    SpriteBundle {
                        texture: texture_handle.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                (x as f32 - y as f32) * TILE_SIZE_X / 2.0 * MINIMAP_SCALE,
                                -(x as f32 + y as f32) * TILE_SIZE_Y / 2.0 * MINIMAP_SCALE,
                                0.0,
                            ),
                            scale: Vec3::splat(MINIMAP_SCALE),
                            ..default()
                        },
                        ..default()
                    },
                    TextureAtlas {
                        layout: texture_atlas_handle.clone(),
                        index: tile_type.to_index(),
                    },
                    MinimapTile,
                    HIGH_RES_LAYERS,
                ))
                .id();
            commands.entity(minimap_parent).push_children(&[child]);
        }
    }
    // player
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(5.0)).into(),
            material: materials.add(ColorMaterial::from(CIRCLE_COLOR)),
            transform: Transform::from_xyz(MINIMAP_SCREEN_X, MINIMAP_SCREEN_Y, 500.0),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayerMinimapIcon,
        HIGH_RES_LAYERS,
    ));
}

fn update_minimap_content(
    camera_resource: Res<camera::CameraResource>,
    mut transform_set: ParamSet<(
        Query<&Transform>,
        Query<&mut Transform, With<MinimapParent>>,
        Query<&mut Transform, With<PlayerMinimapIcon>>,
    )>,
) {
    // First, get the camera position
    let camera_position = transform_set
        .p0()
        .get(camera_resource.pixel_camera)
        .map(|transform| Vec2::new(transform.translation.x, transform.translation.y))
        .unwrap_or(Vec2::ZERO);

    let screen_width = camera_resource.pixel_w as f32;
    let screen_height = camera_resource.pixel_h as f32;

    // Calculate minimap position relative to screen size
    let screen_pos_x = -screen_width * MINIMAP_SCREEN_PERCENT_X;
    let screen_pos_y = screen_height * MINIMAP_SCREEN_PERCENT_Y;

    // Then, update the minimap position
    if let Ok(mut minimap_parent_transform) = transform_set.p1().get_single_mut() {
        let content_offset = Vec2::new(
            -camera_position.x * MINIMAP_SCALE,
            -camera_position.y * MINIMAP_SCALE,
        );
        minimap_parent_transform.translation.x = screen_pos_x + content_offset.x;
        minimap_parent_transform.translation.y = screen_pos_y + content_offset.y;
    }

    if let Ok(mut player_icon_transform) = transform_set.p2().get_single_mut() {
        player_icon_transform.translation.x = screen_pos_x;
        player_icon_transform.translation.y = screen_pos_y;
    }
}

fn toggle_minimap(keys: Res<ButtonInput<KeyCode>>, mut minimap_state: ResMut<MinimapState>) {
    if keys.just_pressed(KeyCode::KeyM) {
        minimap_state.visible = !minimap_state.visible;
    }
}

fn update_minimap_visibility(
    minimap_state: Res<MinimapState>,
    mut transform_set: ParamSet<(
        Query<&mut Visibility, With<MinimapParent>>,
        Query<&mut Visibility, With<PlayerMinimapIcon>>,
    )>,
) {
    if minimap_state.is_changed() {
        if let Ok(mut visibility) = transform_set.p0().get_single_mut() {
            *visibility = if minimap_state.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
        if let Ok(mut visibility) = transform_set.p1().get_single_mut() {
            *visibility = if minimap_state.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
