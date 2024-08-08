mod asset_loader;
mod assets;
mod plugin;
mod reexports;

pub(crate) mod ldtk;
pub(crate) mod system_params;
pub(crate) mod util;

pub mod iid;
// pub mod system_params;

pub mod prelude {
    pub use crate::assets::entity::LdtkEntityAsset;
    pub use crate::assets::project::LdtkProject;
    pub use crate::iid::Iid;
    pub use crate::iid::IidError;
    pub use crate::plugin::ShieldTankPlugin;
    // pub use crate::system_params::commands::LdtkCommands;
    pub use crate::system_params::LdtkEntityIteratorExt;
    pub use crate::system_params::LdtkQuery;

    pub use crate::reexports::field_instance::FieldInstance;
    pub use crate::reexports::tileset_rectangle::TilesetRectangle;
}
