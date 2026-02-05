use bevy_asset::AssetPath;
use bevy_ecs::component::Component;

#[derive(Clone, Default, Component)]
pub enum ShieldtankComponentFilter {
    #[default]
    All,
    None,
    ByPattern(String),
}

impl ShieldtankComponentFilter {
    pub(crate) fn should_load<'a>(&self, _asset_path: &AssetPath<'a>) -> bool {
        match self {
            ShieldtankComponentFilter::All => true,
            ShieldtankComponentFilter::None => false,
            ShieldtankComponentFilter::ByPattern(_) => todo!(),
        }
    }
}
