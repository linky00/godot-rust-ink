use godot::{
    classes::{EditorImportPlugin, IEditorImportPlugin},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=EditorImportPlugin, init, tool)]
pub struct StoryImportPlugin {
    base: Base<EditorImportPlugin>,
}

#[godot_api]
impl IEditorImportPlugin for StoryImportPlugin {
    fn get_importer_name(&self) -> GString {
        "godotrustink.story".into()
    }

    fn get_visible_name(&self) -> GString {
        "Ink Story".into()
    }

    fn get_recognized_extensions(&self) -> PackedStringArray {
        ["json".into()].into()
    }

    fn get_save_extension(&self) -> GString {
        "json".into()
    }

    fn get_resource_type(&self) -> GString {
        "JSON".into()
    }

    fn get_import_options(&self, _path: GString, _preset_index: i32) -> Array<VarDictionary> {
        Array::new()
    }
}
