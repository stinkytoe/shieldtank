use bevy::prelude::*;
use thiserror::Error;

use crate::ldtk;
use crate::reexports::tileset_rectangle::TilesetRectangle;

#[derive(Debug, Error)]
pub enum FieldInstanceError {
    #[error("Given unknown field instance type from LDtk project! {0}")]
    UnknownFieldInstanceType(String),
    #[error("value is None!")]
    ValueIsNone,
    #[error("Unable to parse as an integer!")]
    BadInt,
    #[error("Unable to parse as a float!")]
    BadFloat,
    #[error("Unable to parse as a string?!")]
    BadString,
    #[error("Unable to parse as a boolean?!")]
    BadBool,
    #[error("Unable to parse as a TilesetRectangle? serde_json error: {0:?}")]
    BadTile(#[from] serde_json::error::Error),
}

#[derive(Debug, Error)]
pub enum FieldInstanceValueAsTileError {
    #[error("Parse error! {0:?}")]
    ParseError(#[from] FieldInstanceError),
    #[error("Wrong type!")]
    WrongType,
}

#[derive(Clone, Debug, Reflect)]
// TODO: fill me out!
pub enum FieldInstanceValue {
    Int(i64),
    Float(f64),
    String(Option<String>),
    Multilines(String),
    Enum(String),
    Bool(bool),
    Tile(TilesetRectangle),
    ArrayTile(Vec<TilesetRectangle>),
    // from GridPoint
    // GridPoint(U64Vec2),
    // EntityReferenceInfo(ReferenceToAnEntityInstance),
    // Array(Vec<FieldInstanceValue>),
}

#[derive(Clone, Debug, Reflect)]
pub struct FieldInstance {
    pub identifier: String,
    pub tile: Option<TilesetRectangle>,
    pub value: FieldInstanceValue,
    pub def_uid: i64,
}

impl FieldInstance {
    pub fn as_tile(&self) -> Result<&TilesetRectangle, FieldInstanceValueAsTileError> {
        if let FieldInstanceValue::Tile(tile) = &self.value {
            Ok(tile)
        } else {
            Err(FieldInstanceValueAsTileError::WrongType)
        }
    }
}

impl From<FieldInstance> for FieldInstanceValue {
    fn from(val: FieldInstance) -> Self {
        val.value
    }
}

// { "__identifier": "Integer", "__type": "Int", "__value": 0, "__tile": null, "defUid": 312, "realEditorValues": [] },
// { "__identifier": "Float", "__type": "Float", "__value": 0, "__tile": null, "defUid": 313, "realEditorValues": [] },
// { "__identifier": "Boolean", "__type": "Bool", "__value": false, "__tile": null, "defUid": 316, "realEditorValues": [] },
// { "__identifier": "String", "__type": "String", "__value": null, "__tile": null, "defUid": 314, "realEditorValues": [] },
// { "__identifier": "Multilines", "__type": "String", "__value": null, "__tile": null, "defUid": 317, "realEditorValues": [] },
// { "__identifier": "Color", "__type": "Color", "__value": "#000000", "__tile": null, "defUid": 318, "realEditorValues": [] },
// { "__identifier": "File_path", "__type": "FilePath", "__value": null, "__tile": null, "defUid": 319, "realEditorValues": [] },
// { "__identifier": "Tile", "__type": "Tile", "__value": null, "__tile": null, "defUid": 320, "realEditorValues": [] },
// { "__identifier": "Entity_ref", "__type": "EntityRef", "__value": null, "__tile": null, "defUid": 321, "realEditorValues": [] },
// { "__identifier": "Point", "__type": "Point", "__value": null, "__tile": null, "defUid": 322, "realEditorValues": [] },
// { "__identifier": "Enum", "__type": "LocalEnum.Enum", "__value": null, "__tile": null, "defUid": 325, "realEditorValues": [] },
// { "__identifier": "Integer2", "__type": "Array<Int>", "__value": [], "__tile": null, "defUid": 326, "realEditorValues": [] },
// { "__identifier": "Float2", "__type": "Array<Float>", "__value": [], "__tile": null, "defUid": 327, "realEditorValues": [] }

impl FieldInstance {
    pub(crate) fn new(value: &ldtk::FieldInstance) -> Result<Self, FieldInstanceError> {
        Ok(Self {
            identifier: value.identifier.clone(),
            tile: value.tile.as_ref().map(TilesetRectangle::new),
            value: {
                let field_instance_type = value.field_instance_type.as_str();
                let value = value
                    .value
                    .as_ref()
                    .ok_or(FieldInstanceError::ValueIsNone)?;
                match field_instance_type {
                    "Int" => {
                        FieldInstanceValue::Int(value.as_i64().ok_or(FieldInstanceError::BadInt)?)
                    }
                    "Float" => FieldInstanceValue::Float(
                        value.as_f64().ok_or(FieldInstanceError::BadFloat)?,
                    ),
                    "String" => {
                        FieldInstanceValue::String(value.as_str().map(|str| str.to_string()))
                    }
                    "Multilines" => FieldInstanceValue::Multilines(
                        value
                            .as_str()
                            .ok_or(FieldInstanceError::BadString)?
                            .to_owned(),
                    ),
                    "Bool" => FieldInstanceValue::Bool(
                        value.as_bool().ok_or(FieldInstanceError::BadBool)?,
                    ),
                    "Tile" => {
                        let ldtk_tile: ldtk::TilesetRectangle =
                            serde_json::from_value(value.clone())?;
                        let tile = TilesetRectangle::new(&ldtk_tile);
                        FieldInstanceValue::Tile(tile)
                    }
                    "Array<Tile>" => {
                        let ldtk_tile_vec: Vec<ldtk::TilesetRectangle> =
                            serde_json::from_value(value.clone())?;
                        let array_tile = ldtk_tile_vec.iter().map(TilesetRectangle::new).collect();
                        FieldInstanceValue::ArrayTile(array_tile)
                    }
                    // TODO: finish me!
                    _ => {
                        return Err(FieldInstanceError::UnknownFieldInstanceType(
                            field_instance_type.to_owned(),
                        ))
                    }
                }
            },
            def_uid: value.def_uid,
        })
    }
}
