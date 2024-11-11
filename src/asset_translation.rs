use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;
use bevy_math::Vec3;

use crate::project_config::ProjectConfig;

pub trait LdtkAssetTranslation: LdtkAsset {
    fn get_translation(&self, project_config: &ProjectConfig) -> Vec3;
}

use bevy_ldtk_asset::entity::Entity as EntityAsset;
impl LdtkAssetTranslation for EntityAsset {
    fn get_translation(&self, _project_config: &ProjectConfig) -> Vec3 {
        self.location.extend(0.0)
    }
}

use bevy_ldtk_asset::layer::Layer as LayerAsset;
impl LdtkAssetTranslation for LayerAsset {
    fn get_translation(&self, project_config: &ProjectConfig) -> Vec3 {
        let z = (self.index + 1) as f32;
        let z = z * project_config.layer_z_scale;
        self.location.extend(z)
    }
}

use bevy_ldtk_asset::level::Level as LevelAsset;
impl LdtkAssetTranslation for LevelAsset {
    fn get_translation(&self, project_config: &ProjectConfig) -> Vec3 {
        let z = (self.index + 1) as f32;
        let z = z * project_config.layer_z_scale;
        self.location.extend(z)
    }
}

use bevy_ldtk_asset::world::World as WorldAsset;
impl LdtkAssetTranslation for WorldAsset {
    fn get_translation(&self, _project_config: &ProjectConfig) -> Vec3 {
        Vec3::ZERO
    }
}

use bevy_ldtk_asset::project::Project as ProjectAsset;
impl LdtkAssetTranslation for ProjectAsset {
    fn get_translation(&self, _project_config: &ProjectConfig) -> Vec3 {
        Vec3::ZERO
    }
}
