#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    WaitForAssetError(#[from] bevy_asset::WaitForAssetError),

    #[error(transparent)]
    RonSpannedError(#[from] ron::error::SpannedError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    IntoDynamicImageError(#[from] bevy_image::IntoDynamicImageError),

    #[error("Bad Handle! {0}")]
    BadHandle(String),

    #[error("Not a tiles layer")]
    BadTilesLayer,

    #[error("Bad Int Grid! reason: {0}")]
    BadIntGrid(String),

    #[error("Bad ecs entity! {0}")]
    BadEcsEntity(String),
}

pub type Result<T> = core::result::Result<T, Error>;

#[macro_export]
macro_rules! bad_handle {
    ($($args:tt)*) => {
        $crate::error::Error::BadHandle(format!($($args)*))
    };
}

#[macro_export]
macro_rules! bad_ecs_entity {
    ($($args:tt)*) => {
        $crate::error::Error::BadEcsEntity(format!($($args)*))
    };
}
