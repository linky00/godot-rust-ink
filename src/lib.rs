use godot::prelude::*;

mod import;
mod instance;
mod plugin;

struct GodotRustInk;

#[gdextension]
unsafe impl ExtensionLibrary for GodotRustInk {}
