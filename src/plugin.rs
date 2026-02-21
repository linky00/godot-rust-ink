use godot::{
    classes::{EditorPlugin, IEditorPlugin},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=EditorPlugin, init, tool)]
pub struct GodotRustInkPlugin {
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for GodotRustInkPlugin {
    fn enter_tree(&mut self) {}
}
