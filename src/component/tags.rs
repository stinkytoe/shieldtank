use bevy_app::Plugin;
use bevy_ecs::component::Component;
use bevy_reflect::Reflect;

#[derive(Debug, Component, Reflect)]
pub struct LdtkTags {
    tags: Vec<String>,
}

impl LdtkTags {
    pub fn new<T: AsRef<str> + std::fmt::Display>(tags: &[T]) -> Self {
        let tags = tags.iter().map(|tag| tag.to_string()).collect();
        Self { tags }
    }

    pub fn contains(&self, tag: &str) -> bool {
        self.tags.iter().any(|inner_tag| *inner_tag == tag)
    }
}

pub struct TagsPlugin;
impl Plugin for TagsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<LdtkTags>();
    }
}
