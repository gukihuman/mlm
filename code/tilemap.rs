use crate::*;
use rand::Rng;

const TILE_SIZE_X: f32 = 16.0;
const TILE_SIZE_Y: f32 = 8.0;
const TILE_COUNT_X: u32 = 4;
const TILE_COUNT_Y: u32 = 2;
const MAP_WIDTH: u32 = 50;
const MAP_HEIGHT: u32 = 30;

#[derive(Component)]
pub struct Tile;

#[derive(Resource, Default)]
pub struct Tilemap {
    pub tiles: Vec<Vec<TileType>>,
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
            .add_systems(Startup, setup_tilemap)
            .add_systems(Startup, spawn_tiles.after(setup_tilemap));
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
                    index: tile_type.to_index() % TILE_COUNT_X as usize
                        + (tile_type.to_index() / TILE_COUNT_X as usize)
                            * TILE_COUNT_X as usize,
                },
                Tile,
            ));
        }
    }
}
