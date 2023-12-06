
// BEINGS
pub const BEINGS_NUMBER: usize = 20;
pub const MAX_VEL: f32 = 5.0;

// Perlin
pub const NOISE_SCALE: f64 = 20.0;
pub const NOISE_BIAS: f64 = 0.1;

// Sprite
pub const SPRITE_SHEET_PATH: &str = "sheet_1.png";
pub const SPRITE_SCALE_FACTOR: usize = 5;
pub const TILE_W: usize = 6;
pub const TILE_H: usize = 8;
pub const SPRITE_SHEET_W: usize = 36 / TILE_W;
pub const SPRITE_SHEET_H: usize = 40 / TILE_H;

// Window
pub const BG_COLOR: (u8, u8, u8) = (50, 195, 199);
pub const GRID_COLS: usize = 200;
pub const GRID_ROWS: usize = 100;
pub const GEN_W: usize = GRID_COLS * TILE_W * SPRITE_SCALE_FACTOR;
pub const GEN_H: usize = GRID_ROWS * TILE_H * SPRITE_SCALE_FACTOR;

// WORLD
pub const CURRENT_SEED: u32 = 42;
pub const CONTINENT_FREQUENCY: f64 = 1.0;
pub const CONTINENT_LACUNARITY: f64 = 2.208984375;