#[derive(Debug, thiserror::Error)]
pub enum ShieldtankError {
    #[error(transparent)]
    WaitForAssetError(#[from] bevy_asset::WaitForAssetError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    IntoDynamicImageError(#[from] bevy_image::IntoDynamicImageError),

    #[error(transparent)]
    QueryEntityError(#[from] bevy_ecs::query::QueryEntityError),

    #[error("ShieldtankError! {0}")]
    ShieldtankError(String),
}

#[macro_export]
macro_rules! shieldtank_error {
    ($($args:tt)*) => {
        $crate::error::ShieldtankError::ShieldtankError(format!($($args)*))
    };
}
