use bevy_app::Plugin;
use bevy_asset::Handle;
use bevy_asset::{prelude::AssetChanged, AsAssetId, Assets};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_ecs::system::{Commands, Query, Res};
use bevy_image::Image;
use bevy_ldtk_asset::entity::EntityInstance;
use bevy_ldtk_asset::entity_definition::EntityDefinition as LdtkEntityDefinition;
use bevy_ldtk_asset::entity_definition::NineSlice;
use bevy_ldtk_asset::tileset_definition::TilesetDefinition as LdtkTilesetDefinition;
use bevy_ldtk_asset::tileset_rectangle::TilesetRectangle as LdtkTilesetRectangle;
use bevy_log::{trace, warn};
use bevy_math::{Rect, Vec2};
use bevy_reflect::Reflect;
use bevy_sprite::{
    BorderRect, ScalingMode, SliceScaleMode, Sprite, SpriteImageMode, TextureSlicer,
};

use super::entity::LdtkEntity;
use super::entity_definition::EntityDefinition;
use super::shieldtank_component::ShieldtankComponentSystemSet;
use super::tileset_definition::TilesetDefinition;

#[derive(Clone, Debug, Component, Reflect)]
pub struct Tile {
    pub corner: Vec2,
    pub size: Vec2,
    // Not provided by LDtk, inserted by us for convenience
    pub flip_x: bool,
    pub flip_y: bool,
    pub tileset_definition: Handle<LdtkTilesetDefinition>,
}

impl Tile {
    pub fn new(tileset_rectangle: &LdtkTilesetRectangle) -> Self {
        let corner = tileset_rectangle.corner.as_vec2();
        let size = tileset_rectangle.size.as_vec2();
        let flip_x = false;
        let flip_y = false;
        let tileset_definition = tileset_rectangle.tileset_definition.clone();

        Self {
            corner,
            size,
            flip_x,
            flip_y,
            tileset_definition,
        }
    }

    pub fn flip_x(&mut self, flip_x: bool) {
        self.flip_x = flip_x
    }

    pub fn with_flip_x(mut self, flip_x: bool) -> Self {
        self.flip_x = flip_x;
        self
    }
}

fn sprite_mode_cover(tile: &Tile, image: Handle<Image>, asset: &EntityInstance) -> Sprite {
    let flip_x = tile.flip_x;
    let flip_y = tile.flip_y;
    let custom_size = Some(asset.size.as_vec2());
    let rect = Some(Rect::from_corners(tile.corner, tile.corner + tile.size));
    let anchor = asset.anchor;
    let scaling_mode = ScalingMode::FillCenter;
    let image_mode = SpriteImageMode::Scale(scaling_mode);

    Sprite {
        image,
        // texture_atlas: todo!(),
        // color: todo!(),
        flip_x,
        flip_y,
        custom_size,
        rect,
        anchor,
        image_mode,
        ..Default::default()
    }
}

fn sprite_mode_fit_inside(tile: &Tile, image: Handle<Image>, asset: &EntityInstance) -> Sprite {
    let flip_x = tile.flip_x;
    let flip_y = tile.flip_y;
    let custom_size = Some(asset.size.as_vec2());
    let rect = Some(Rect::from_corners(tile.corner, tile.corner + tile.size));
    let anchor = asset.anchor;
    let scaling_mode = ScalingMode::FitCenter;
    let image_mode = SpriteImageMode::Scale(scaling_mode);

    Sprite {
        image,
        // texture_atlas: todo!(),
        // color: todo!(),
        flip_x,
        flip_y,
        custom_size,
        rect,
        anchor,
        image_mode,
        ..Default::default()
    }
}

fn sprite_mode_repeat(tile: &Tile, image: Handle<Image>, asset: &EntityInstance) -> Sprite {
    let flip_x = tile.flip_x;
    let flip_y = tile.flip_y;
    let custom_size = Some(asset.size.as_vec2());
    let rect = Some(Rect::from_corners(tile.corner, tile.corner + tile.size));
    let anchor = asset.anchor;
    let image_mode = SpriteImageMode::Tiled {
        tile_x: true,
        tile_y: true,
        stretch_value: 1.0,
    };

    Sprite {
        image,
        // texture_atlas: todo!(),
        // color: todo!(),
        flip_x,
        flip_y,
        custom_size,
        rect,
        anchor,
        image_mode,
        ..Default::default()
    }
}

fn sprite_mode_stretch(tile: &Tile, image: Handle<Image>, asset: &EntityInstance) -> Sprite {
    let flip_x = tile.flip_x;
    let flip_y = tile.flip_y;
    let custom_size = Some(asset.size.as_vec2());
    let rect = Some(Rect::from_corners(tile.corner, tile.corner + tile.size));
    let anchor = asset.anchor;
    let image_mode = SpriteImageMode::Auto;

    Sprite {
        image,
        // texture_atlas: todo!(),
        // color: todo!(),
        flip_x,
        flip_y,
        custom_size,
        rect,
        anchor,
        image_mode,
        ..Default::default()
    }
}

fn sprite_mode_full_size_uncropped(
    tile: &Tile,
    image: Handle<Image>,
    asset: &EntityInstance,
) -> Sprite {
    let flip_x = tile.flip_x;
    let flip_y = tile.flip_y;
    let rect = Some(Rect::from_corners(tile.corner, tile.corner + tile.size));
    let anchor = asset.anchor;
    let image_mode = bevy_sprite::SpriteImageMode::Auto;

    Sprite {
        image,
        flip_x,
        flip_y,
        rect,
        anchor,
        image_mode,
        ..Default::default()
    }
}

// FIXME: This doesn't render correctly when the width or height is too small
fn sprite_mode_nine_slice(
    tile: &Tile,
    image: Handle<Image>,
    asset: &EntityInstance,
    nine_slice: &NineSlice,
) -> Sprite {
    let flip_x = tile.flip_x;
    let flip_y = tile.flip_y;
    let custom_size = Some(asset.size.as_vec2());
    let rect = Some(Rect::from_corners(tile.corner, tile.corner + tile.size));
    let anchor = asset.anchor;
    let border = BorderRect {
        left: nine_slice.left as f32,
        right: nine_slice.right as f32,
        top: nine_slice.up as f32,
        bottom: nine_slice.down as f32,
    };
    let center_scale_mode = SliceScaleMode::Tile { stretch_value: 1.0 };
    let sides_scale_mode = SliceScaleMode::Tile { stretch_value: 1.0 };
    let texture_slicer = TextureSlicer {
        border,
        center_scale_mode,
        sides_scale_mode,
        //max_corner_scale: (),
        ..Default::default()
    };
    let image_mode = bevy_sprite::SpriteImageMode::Sliced(texture_slicer);

    Sprite {
        image,
        flip_x,
        flip_y,
        custom_size,
        rect,
        anchor,
        image_mode,
        ..Default::default()
    }
}

#[allow(clippy::type_complexity)]
fn insert_sprite_system(
    query: Query<
        (
            Entity,
            &LdtkEntity,
            &EntityDefinition,
            &TilesetDefinition,
            &Tile,
        ),
        Or<(
            Changed<LdtkEntity>,
            AssetChanged<LdtkEntity>,
            Changed<TilesetDefinition>,
            AssetChanged<TilesetDefinition>,
            Changed<Tile>,
        )>,
    >,
    entity_assets: Res<Assets<EntityInstance>>,
    entity_definitions: Res<Assets<LdtkEntityDefinition>>,
    tileset_definitions: Res<Assets<LdtkTilesetDefinition>>,
    mut commands: Commands,
) {
    query
        .iter()
        .filter_map(
            |(entity, asset, entity_definition, tileset_definition, tile)| {
                Some((
                    entity,
                    entity_assets.get(asset.as_asset_id())?,
                    entity_definitions.get(entity_definition.as_asset_id())?,
                    tileset_definitions.get(tileset_definition.as_asset_id())?,
                    tile,
                ))
            },
        )
        .for_each(
            |(entity, asset, entity_definition, tileset_definition, tile)| {
                // This is where we would render a mesh instead of a sprite
                let Some(image) = tileset_definition.tileset_image.clone() else {
                    todo!()
                };

                let sprite = match &entity_definition.render_mode {
                    bevy_ldtk_asset::prelude::TileRenderMode::Cover => {
                        sprite_mode_cover(tile, image, asset)
                    }
                    bevy_ldtk_asset::prelude::TileRenderMode::FitInside => {
                        sprite_mode_fit_inside(tile, image, asset)
                    }
                    bevy_ldtk_asset::prelude::TileRenderMode::Repeat => {
                        sprite_mode_repeat(tile, image, asset)
                    }
                    bevy_ldtk_asset::prelude::TileRenderMode::Stretch => {
                        sprite_mode_stretch(tile, image, asset)
                    }
                    bevy_ldtk_asset::prelude::TileRenderMode::FullSizeCropped => {
                        warn!("FullSizeCropped not supported yet!");
                        return;
                    }
                    bevy_ldtk_asset::prelude::TileRenderMode::FullSizeUncropped => {
                        sprite_mode_full_size_uncropped(tile, image, asset)
                    }
                    bevy_ldtk_asset::prelude::TileRenderMode::NineSlice(nine_slice) => {
                        sprite_mode_nine_slice(tile, image, asset, nine_slice)
                    }
                };

                trace!("Inserting Sprite");
                commands.entity(entity).insert(sprite);
            },
        );
}

fn insert_tileset_definition(query: Query<(Entity, &Tile), Changed<Tile>>, mut commands: Commands) {
    query.iter().for_each(|(entity, tile)| {
        let tileset_definition = tile.tileset_definition.clone();
        let tileset_definition = TilesetDefinition::new(tileset_definition);

        commands.entity(entity).insert(tileset_definition);
    });
}

pub struct TilePlugin;
impl Plugin for TilePlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<Tile>();
        app.add_systems(
            ShieldtankComponentSystemSet,
            (
                insert_sprite_system.after(insert_tileset_definition),
                insert_tileset_definition,
            ),
        );
    }
}
