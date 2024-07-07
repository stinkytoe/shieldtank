use bevy::prelude::*;
use bevy::sprite::Anchor;
use path_clean::PathClean;
use std::num::ParseIntError;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

pub(crate) fn ldtk_path_to_bevy_path(base_directory: &Path, ldtk_path: &Path) -> PathBuf {
    base_directory.join(ldtk_path).clean()
}

#[derive(Debug, Error)]
pub enum ColorParseError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Unable to parse given color string! expect hex color in format: #rrggbb, got: {0}")]
    UnableToParse(String),
}

// Format should be: Hex color "#rrggbb"
// from: https://ldtk.io/json-2/#ldtk-ProjectJson;bgColor
pub(crate) fn bevy_color_from_ldtk(color: &str) -> Result<Color, ColorParseError> {
    if color.len() != 7 {
        return Err(ColorParseError::UnableToParse(color.to_owned()));
    }

    let hashmark = &color[0..1];
    let r = &color[1..3];
    let g = &color[3..5];
    let b = &color[5..7];

    if hashmark != "#" {
        return Err(ColorParseError::UnableToParse(color.to_owned()));
    };

    Ok(Color::srgb_u8(
        u8::from_str_radix(r, 16)?,
        u8::from_str_radix(g, 16)?,
        u8::from_str_radix(b, 16)?,
    ))
}

#[derive(Debug, Error)]
pub enum AnchorIntoError {
    #[error("Provided array not four numbers!")]
    _BadArrayLength,
}

pub(crate) fn bevy_anchor_from_ldtk(pivot: &[f64]) -> Result<Anchor, AnchorIntoError> {
    if pivot.len() != 2 {
        return Err(AnchorIntoError::_BadArrayLength);
    }

    Ok(match (pivot[0] as f32, pivot[1] as f32) {
        (0.0, 0.0) => Anchor::TopLeft,
        (0.0, 0.5) => Anchor::CenterLeft,
        (0.0, 1.0) => Anchor::BottomLeft,
        (0.5, 0.0) => Anchor::TopCenter,
        (0.5, 0.5) => Anchor::Center,
        (0.5, 1.0) => Anchor::BottomCenter,
        (1.0, 0.0) => Anchor::TopRight,
        (1.0, 0.5) => Anchor::CenterRight,
        (1.0, 1.0) => Anchor::BottomRight,
        (x, y) => Anchor::Custom(Vec2::new(x - 0.5, 0.5 - y)),
    })
}
