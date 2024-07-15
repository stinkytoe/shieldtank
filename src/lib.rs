mod asset_loader;
mod assets;
mod ldtk;
mod plugin;

pub(crate) mod util;

pub mod iid;
pub mod system_params;

pub mod prelude {
    pub use crate::assets::project::LdtkProject;
    pub use crate::iid::Iid;
    pub use crate::iid::IidError;
    pub use crate::plugin::ShieldTankPlugin;
}
