use noise::{NoiseFn, Perlin};
use rand::Rng;

pub const MAP_WIDTH: u32 = 250;
pub const MAP_HEIGHT: u32 = 250;

// Noise parameters
const NOISE_SCALE: f64 = 100.0;
const RIVER_THRESHOLD: f64 = 0.05; // Adjust this to control river width
const RIVER_EDGE_SOFTNESS: f64 = 0.02; // Adjust for softer/harder river edges

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Stone,
    Grass,
    Water,
    Dirt,
}

impl TileType {
    pub fn to_index(&self) -> usize {
        match self {
            TileType::Stone => 0,
            TileType::Grass => 1,
            TileType::Water => 2,
            TileType::Dirt => 3,
        }
    }
}

pub fn generate_tilemap() -> Vec<Vec<TileType>> {
    let mut rng = rand::thread_rng();
    let perlin = Perlin::new(rng.gen());

    let mut heightmap = vec![vec![0.0; MAP_WIDTH as usize]; MAP_HEIGHT as usize];

    // Generate initial noise
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let nx = x as f64 / NOISE_SCALE;
            let ny = y as f64 / NOISE_SCALE;

            // Generate two noise values and subtract them to create "ridges"
            let noise1 = perlin.get([nx, ny]);
            let noise2 = perlin.get([nx + 5.0, ny + 5.0]); // Offset for different pattern

            heightmap[y as usize][x as usize] = noise1 - noise2;
        }
    }

    // Convert heightmap to tiles
    (0..MAP_HEIGHT)
        .map(|y| {
            (0..MAP_WIDTH)
                .map(|x| {
                    let height = heightmap[y as usize][x as usize];

                    // Create rivers where the height difference is near zero
                    if height.abs() < RIVER_THRESHOLD + RIVER_EDGE_SOFTNESS {
                        TileType::Water
                    } else {
                        TileType::Grass
                    }
                })
                .collect()
        })
        .collect()
}

// Helper function to get neighboring tiles (useful for future enhancements)
fn get_neighbors(x: i32, y: i32, map: &Vec<Vec<TileType>>) -> Vec<TileType> {
    let directions = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    directions
        .iter()
        .filter_map(|(dx, dy)| {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x >= 0 && new_x < MAP_WIDTH as i32 && new_y >= 0 && new_y < MAP_HEIGHT as i32 {
                Some(map[new_y as usize][new_x as usize])
            } else {
                None
            }
        })
        .collect()
}
