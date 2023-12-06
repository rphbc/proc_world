use bevy::{ecs::{system::{Commands, Res, ResMut}, component::Component}, asset::{AssetServer, Assets, Handle}, sprite::{TextureAtlas, SpriteSheetBundle, TextureAtlasSprite}, render::texture::Image, math::{Vec2, Vec3, vec3}, utils::{HashSet, default}, transform::components::Transform, app::{Plugin, Startup, Update, App}};
use noise::{NoiseFn, Perlin, Fbm, MultiFractal};
use rand::Rng;

use crate::configs::*;
use crate::utils::grid_to_world;


pub struct Tile {
    pos: (i32, i32),
    sprite: usize,
    z_index: i32,
}

#[derive(Component)]
pub struct TileComponent;


pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, gen_world);
    }
}

fn gen_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut rng = rand::thread_rng();
    let perlin = Fbm::<Perlin>::new(CURRENT_SEED)
    .set_frequency(CONTINENT_FREQUENCY)
    .set_persistence(0.5)
    .set_lacunarity(CONTINENT_LACUNARITY)
    .set_octaves(14);

    // let perlin = Perlin::new(42);

    let texture_handler: Handle<Image> = asset_server.load(SPRITE_SHEET_PATH);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handler,
        Vec2::new(TILE_W as f32, TILE_H as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut tiles = Vec::new();
    let mut occupied = HashSet::new();
    for x in 0..GRID_COLS {
        for y in 0..GRID_ROWS {
            let noise_val = perlin.get([x as f64 / NOISE_SCALE, y as f64 / NOISE_SCALE]) + NOISE_BIAS;
            let choice = rng.gen_range(0.0..1.0);
            let (x, y) = (x as i32, y as i32);

            // Ground
            if noise_val > 0.1 {
                occupied.insert((x, y));
            }

            // // Mountain
            // if noise_val > 0.3 && noise_val < 0.31 {
            //     tiles.push(Tile::new((x, y), 10, 1));
            // }

            // Trees
            if noise_val > 0.35 && noise_val < 0.99 {
                if choice > 0.9 {
                    tiles.push(Tile::new((x, y), rng.gen_range(7..=9), 1));
                } else if choice > 0.8 {
                    tiles.push(Tile::new((x,y), 6 , 1))
                }
            }

            // // Bones
            // if noise_val > 0.6 && noise_val < 0.7 && choice >0.9 {
            //     tiles.push(Tile::new((x, y), rng.gen_range(18..=19), 1));
            // }

            // // Houses
            // if noise_val > 0.7 && choice > 0.97 {
            //     let house_tile = if rng.gen_range(0.0..1.0) > 0.85 { 12 } else {13};
            //     tiles.push(Tile::new((x, y), house_tile, 1));
            // }
        }
    }

    for (x, y) in occupied.iter() {
        let (tile, nei_count) = get_tile((*x, *y), &occupied);
        if nei_count == 1 {
            continue;
        }
        tiles.push(Tile::new((*x, *y), tile, 0));
    }

    for tile in tiles.iter() {
        let (x, y) = tile.pos;
        let (x, y) = grid_to_world(x as f32, y as f32);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(tile.sprite),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32))
                    .with_translation(vec3(x, y, tile.z_index as f32)),
                ..default()
            },
            TileComponent,
        ));
    }

}

fn get_tile((x, y): (i32, i32), occupied: &HashSet<(i32, i32)>) -> (usize, i32) {
    let (x, y) = (x as i32, y as i32);
    let nei_options = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut nei = [1, 1, 1, 1];
    let mut nei_count = 0;

    for (idx, (i, j)) in nei_options.iter().enumerate() {
        if occupied.contains(&(x + i, y + j)) {
            nei_count += 1;
            continue;
        }
        nei[idx] = 0;
    }

    let tile = match nei {
        [0, 1, 1, 0] => 1,
        [1, 0, 1, 0] => 2,
        [0, 1, 0, 1] => 3,
        [1, 0, 0, 1] => 4,
        _ => 0,
    };
    (tile, nei_count)
}

// fn center_to_top_left (x: f32, y: f32) -> (f32, f32) {
//     let x_center = x - (GRID_COLS as f32 * SPRITE_SCALE_FACTOR as f32 ) / 2.0;
//     let y_center = ((GRID_ROWS as f32 * SPRITE_SCALE_FACTOR as f32) / 2.0) - y;
//     (x_center,y_center)
// }

impl Tile {
    fn new(pos: (i32, i32), sprite: usize, z_index: i32) -> Self {
        Self {
            pos,
            sprite,
            z_index,
        }
    }
}