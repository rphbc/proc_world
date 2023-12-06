use bevy::ecs::world::World;
use bevy_editor_pls::{editor_window::{EditorWindow, EditorWindowContext}};

pub struct MyEditorWindow;

#[derive(Debug, Default)]
pub struct MyEditorWindowState {
}
impl EditorWindow for MyEditorWindow {
    type State = MyEditorWindowState;
    const NAME: &'static str = "Another editor panel";

    fn ui(world: &mut World, cx: EditorWindowContext, ui: &mut egui::Ui) {
        let currently_inspected = &cx.state::<MyEditorWindow>().unwrap();

        ui.label("Anything can go here");
    }
}