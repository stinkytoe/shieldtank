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

    #[error("ShieldtankError! {0}")]
    ShieldtankError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! shieldtank_error {
    ($($args:tt)*) => {
        $crate::error::Error::ShieldtankError(format!($($args)*))
    };
}
