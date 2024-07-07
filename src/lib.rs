mod asset_loader;
mod assets;
mod components;
mod iid;
mod ldtk;
mod plugin;

pub(crate) mod util;

pub mod system_params;

pub mod prelude {
    // pub use crate::iid::Iid;
    // pub use crate::iid::IidError;
    pub use crate::assets::project::LdtkProject;
    pub use crate::plugin::ShieldTankPlugin;
}
