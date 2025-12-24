use bevy_math::Vec2;
use itertools::ExactlyOneError;

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

    #[error("{0:?}")]
    SingleError(SingleError),

    #[error("ShieldtankError! {0}")]
    ShieldtankError(String),
}

impl ShieldtankError {
    pub(crate) fn from_exactly_one<I: std::iter::Iterator>(
        e: ExactlyOneError<I>,
        location: Vec2,
    ) -> Self {
        match e.count() {
            0 => Self::SingleError(SingleError::NoItems(location)),
            _ => Self::SingleError(SingleError::MultipleItems(location)),
        }
    }
}

#[derive(Debug)]
pub enum SingleError {
    NoItems(Vec2),
    MultipleItems(Vec2),
}

#[macro_export]
macro_rules! shieldtank_error {
    ($($args:tt)*) => {
        $crate::error::ShieldtankError::ShieldtankError(format!($($args)*))
    };
}
