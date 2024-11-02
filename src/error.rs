#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    WaitForAssetError(#[from] bevy::asset::WaitForAssetError),

    #[error(transparent)]
    RonSpannedError(#[from] ron::error::SpannedError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    IntoDynamicImageError(#[from] bevy::render::texture::IntoDynamicImageError),

    #[error("Bad Handle!")]
    BadHandle,

    #[error("Not a tiles layer")]
    BadTilesLayer,

    #[error("Bad Int Grid! reason: {0}")]
    BadIntGrid(String),
}

pub type Result<T> = core::result::Result<T, Error>;
