use bevy::{math::vec3, prelude::*, utils::HashSet, window::{close_on_esc, PrimaryWindow}, core_pipeline::deferred::DEFERRED_LIGHTING_PASS_ID_DEPTH_FORMAT};
use bevy::input::common_conditions::input_toggle_active;
use bevy_editor_pls::{EditorPlugin, AddEditorWindow};
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use noise::{NoiseFn, Perlin, Fbm, MultiFractal};
use rand::Rng;

use procedural::{*, world::GroundComponent};
use procedural::editor_mode::{*};
use procedural::world::WorldPlugin;
use procedural::beings::BeingsPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgba_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 255,
        )))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PanCamPlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Key1)),
        )
        // .add_plugins(EditorPlugin::default())
        // .add_editor_window::<MyEditorWindow>()
        .add_plugins(WorldPlugin)
        .add_plugins(BeingsPlugin)
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(Update, close_on_esc)
        // .add_systems(Update, cursor_position)
        .run();
}

fn handle_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    tiles_query: Query<Entity, With<GroundComponent>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if !keys.just_pressed(KeyCode::Tab) {
        return;
    }

    for entity in tiles_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    commands
        .spawn(Camera2dBundle{
            transform: Transform::from_xyz(GEN_W as f32 / 2.0, GEN_H as f32 / 2.0, 0.0),
            ..Default::default()
        })
        .insert(PanCam::default());
}


