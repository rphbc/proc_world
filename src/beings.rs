use bevy::{app::{Plugin, App, Startup, Update}, ecs::{system::{Commands, Res, ResMut, Query}, component::Component, query::With}, asset::{AssetServer, Assets, Handle}, sprite::{TextureAtlas, SpriteSheetBundle, TextureAtlasSprite}, render::texture::Image, math::{Vec2, Vec3, vec3}, transform::components::Transform, prelude::default};
use rand::Rng;

use crate::{*};
use crate::utils::{grid_to_world, limit_var};


#[derive(Component)]
struct Being;

#[derive(Component)]
struct Health(usize);

#[derive(Component)]
struct Vel{
    x: f32,
    y: f32
}

#[derive(Component)]
struct Acc{
    x: f32,
    y: f32
}


pub struct BeingsPlugin;

impl Plugin for BeingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, gen_beings)
            .add_systems(Update, movement);
    }
}

fn gen_beings(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let mut rng = rand::thread_rng();
    
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

    for _ in 0..BEINGS_NUMBER {

        let (x,y) = grid_to_world(rng.gen_range(0..GRID_COLS) as f32, rng.gen_range(0..GRID_ROWS) as f32);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(27),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32))
                    .with_translation(vec3(x, y, 10.0)),
                ..default()
            },
            Being,
            Health(100),
            Vel{x: 0.0, y: 0.0},
            Acc{x: 0.0, y: 0.0},
        ));
    }
}

fn movement (
    mut being_query: Query<(&mut Transform, &mut Vel,&mut Acc), With<Being>>,
) {
    let mut rng = rand::thread_rng();

    for (mut t,mut v,mut a) in being_query.iter_mut() {

        a.x += rng.gen_range(-1.0..1.0);
        a.y += rng.gen_range(-1.0..1.0);

        v.x += a.x;
        v.y += a.y;

        v.x = limit_var(v.x, MAX_VEL);
        v.y = limit_var(v.y, MAX_VEL);

        t.translation.x += v.x;
        t.translation.y += v.y;

        let limits = grid_to_world(GRID_COLS as f32, GRID_ROWS as f32);

        // println!("limits - {:?}", limits);

        if t.translation.x > limits.0 {
            t.translation.x = t.translation.x - limits.0;
        }

        if t.translation.x < 0.0 {
            t.translation.x = t.translation.x + limits.0;
        }

        if t.translation.y > limits.1 {
            t.translation.y = t.translation.y - limits.1;
        }

        if t.translation.y < 0.0 {
            t.translation.y = t.translation.y + limits.1;
        }


    }
}