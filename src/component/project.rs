use bevy_app::Plugin;
use bevy_ldtk_asset::project::Project as ProjectAsset;

use super::ShieldtankComponent;

pub type ProjectComponent = ShieldtankComponent<ProjectAsset>;

pub type ProjectComponentQueryData<'a> = ();

pub struct ProjectPlugin;
impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<ProjectComponent>();
    }
}
