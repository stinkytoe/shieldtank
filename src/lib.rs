mod asset_loader;
mod assets;
mod plugin;
mod reexports;

pub(crate) mod ldtk;
pub(crate) mod util;

pub mod commands;
pub mod iid;
pub mod system_params;

pub mod prelude {
    pub use crate::assets::project::LdtkProject;
    pub use crate::commands::ldtk_commands::LdtkCommands;
    pub use crate::iid::Iid;
    pub use crate::iid::IidError;
    pub use crate::plugin::ShieldTankPlugin;
    pub use crate::system_params::entity::item::LdtkEntity;
    pub use crate::system_params::entity::query::LdtkEntityQuery;
    pub use crate::system_params::project::LdtkProjectQuery;
    pub use crate::system_params::project::LdtkProjectQueryError;
    pub use crate::system_params::traits::LdtkItem;
    pub use crate::system_params::traits::LdtkQuery;
    pub use crate::system_params::traits::LdtkQueryError;
    pub use crate::system_params::traits::LdtkQueryEx;
}
