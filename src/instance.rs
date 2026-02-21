use bladeink::story::Story;
use godot::{classes::Json, prelude::*};

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct InkStoryInstance {
    #[export]
    ink_json: Option<Gd<Json>>,

    ink_story: Option<Story>,
}
