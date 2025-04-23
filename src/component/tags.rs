use bevy_app::Plugin;
use bevy_derive::Deref;
use bevy_ecs::component::Component;
use bevy_reflect::Reflect;

#[derive(Debug, Deref, Component, Reflect)]
pub struct Tags {
    #[deref]
    tags: Vec<String>,
}

impl Tags {
    pub fn new<T: AsRef<str> + std::fmt::Display>(tags: &[T]) -> Self {
        let tags = tags.iter().map(|tag| tag.to_string()).collect();
        Self { tags }
    }
}

pub struct TagsPlugin;
impl Plugin for TagsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<Tags>();
    }
}
