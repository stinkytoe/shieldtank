use bevy_ldtk_asset::entity::Entity as EntityAsset;

use crate::component::entity::EntityComponentQueryData;

use super::ShieldtankItemCommands;

pub type EntityCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, EntityAsset, EntityComponentQueryData<'w>>;
