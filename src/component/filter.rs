use bevy_ecs::component::Component;
use bevy_log::error;
use regex::Regex;

#[derive(Clone, Default, Component)]
pub enum ChildAutoloadFilter {
    #[default]
    All,
    None,
    ByPattern(String),
    ByList(&'static [&'static str]),
}

impl ChildAutoloadFilter {
    pub(crate) fn should_load(&self, label: &str) -> bool {
        match self {
            ChildAutoloadFilter::All => true,
            ChildAutoloadFilter::None => false,
            ChildAutoloadFilter::ByPattern(pattern) => {
                let Ok(re) = Regex::new(pattern) else {
                    error!("Could not compile regex! {pattern}");
                    return false;
                };
                re.captures(label).is_some()
            }
            ChildAutoloadFilter::ByList(list) => list.contains(&label),
        }
    }
}
