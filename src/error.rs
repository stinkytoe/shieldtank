#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    WaitForAssetError(#[from] bevy_asset::WaitForAssetError),

    #[error(transparent)]
    RonSpannedError(#[from] ron::error::SpannedError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    IntoDynamicImageError(#[from] bevy_render::texture::IntoDynamicImageError),

    //#[error(transparent)]
    //QueryEntityError(#[from] bevy::ecs::query::QueryEntityError),
    #[error("Bad Handle! {0}")]
    BadHandle(String),

    #[error("Not a tiles layer")]
    BadTilesLayer,

    #[error("Bad Int Grid! reason: {0}")]
    BadIntGrid(String),

    #[error("Bad entity! {0}")]
    BadEntity(String),
}

pub type Result<T> = core::result::Result<T, Error>;

#[macro_export]
macro_rules! bad_handle {
    ($handle:expr) => {
        $crate::Error::BadHandle(format!("{:?}", $handle.path()))
    };
}
pub use bad_handle;

#[macro_export]
macro_rules! bad_entity {
    ($entity:expr) => {
        $crate::Error::BadEntity(format!("{:?}", $entity))
    };
}
pub use bad_entity;
