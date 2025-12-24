use bevy_math::Vec2;

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

    #[error("")]
    SingleError(Vec2),

    #[error("ShieldtankError! {0}")]
    ShieldtankError(String),
}

#[macro_export]
macro_rules! shieldtank_error {
    ($($args:tt)*) => {
        $crate::error::ShieldtankError::ShieldtankError(format!($($args)*))
    };
}
