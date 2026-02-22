use bladeink::story::Story;
use godot::{classes::Json, prelude::*};

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct InkStoryInstance {
    #[export]
    ink_json: Option<Gd<Json>>,

    story: Option<Story>,
}

#[godot_api]
impl INode for InkStoryInstance {
    fn ready(&mut self) {
        if let Some(ink_json) = &self.ink_json {
            let new_story = Story::new(&ink_json.get_parsed_text().to_string());

            match new_story {
                Ok(new_story) => {
                    self.story = Some(new_story);
                }
                Err(e) => godot_error!("Story could not be loaded: {e}"),
            }
        } else {
            godot_error!("ink_json is not set")
        }
    }
}

#[godot_api]
impl InkStoryInstance {
    #[func]
    fn cont(&mut self) -> GString {
        if let Some(story) = &mut self.story {
            match story.cont() {
                Ok(line) => GString::from(&line),
                Err(_) => GString::default(),
            }
        } else {
            GString::default()
        }
    }
}
