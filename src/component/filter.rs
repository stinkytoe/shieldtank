use bevy_ecs::component::Component;
use bevy_log::error;
use regex::Regex;

#[derive(Clone, Default, Component)]
pub enum ShieldtankComponentFilter {
    #[default]
    All,
    None,
    ByPattern(String),
    ByList(&'static [&'static str]),
}

impl ShieldtankComponentFilter {
    pub(crate) fn should_load(&self, label: &str) -> bool {
        match self {
            ShieldtankComponentFilter::All => true,
            ShieldtankComponentFilter::None => false,
            ShieldtankComponentFilter::ByPattern(pattern) => {
                let Ok(re) = Regex::new(pattern) else {
                    error!("Could not compile regex! {pattern}");
                    return false;
                };
                re.captures(label).is_some()
            }
            ShieldtankComponentFilter::ByList(list) => list.contains(&label),
        }
    }
}
