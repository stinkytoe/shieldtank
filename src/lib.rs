mod asset_loader;
mod assets;
mod plugin;
mod reexports;

pub(crate) mod ldtk;
pub(crate) mod util;

pub mod iid;
pub mod system_params;

pub mod prelude {
    pub use crate::assets::project::LdtkProject;
    pub use crate::iid::Iid;
    pub use crate::iid::IidError;
    pub use crate::plugin::ShieldTankPlugin;
    pub use crate::system_params::entity::commands::LdtkEntityCommands;
    pub use crate::system_params::entity::commands::LdtkEntityCommandsError;
    pub use crate::system_params::entity::item::LdtkEntity;
    pub use crate::system_params::entity::query::LdtkEntityQuery;
    pub use crate::system_params::entity::query::LdtkEntityQueryError;
    pub use crate::system_params::entity::query::LdtkEntityQueryEx;
    pub use crate::system_params::project::LdtkProjectCommandsError;
    pub use crate::system_params::project::LdtkProjectQuery;
}
