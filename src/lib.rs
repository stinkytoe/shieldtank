mod asset_loader;
mod assets;
mod plugin;
mod reexports;

pub(crate) mod ldtk;
pub(crate) mod query;
pub(crate) mod util;

pub mod iid;

pub mod prelude {
    pub use crate::assets::entity::LdtkEntityAsset;
    pub use crate::assets::level::LdtkLevelAsset;
    pub use crate::assets::project::LdtkProject;
    pub use crate::iid::Iid;
    pub use crate::iid::IidError;
    pub use crate::plugin::ShieldTankPlugin;
    pub use crate::query::entities::LdtkEntitiesQuery;
    pub use crate::query::entities::LdtkEntity;
    pub use crate::query::levels::LdtkLevel;
    pub use crate::query::levels::LdtkLevelsQuery;
    pub use crate::query::projects::LdtkProjectsQuery;
    pub use crate::query::traits::LdtkItemIteratorExt;
    pub use crate::query::traits::LdtkItemIteratorUniqueIdentifierExt;
    pub use crate::reexports::field_instance::FieldInstance;
    pub use crate::reexports::tileset_rectangle::TilesetRectangle;
}
