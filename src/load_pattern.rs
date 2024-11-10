use bevy_asset::Handle;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;
use bevy_log::debug;
use bevy_reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, Reflect, Serialize, Deserialize)]
pub enum LoadPattern {
    #[default]
    All,
    None,
    Pattern(String),
}

impl LoadPattern {
    pub fn handle_matches_pattern<Asset: LdtkAsset>(&self, handle: &Handle<Asset>) -> bool {
        match self {
            LoadPattern::All => true,
            LoadPattern::None => false,
            LoadPattern::Pattern(pattern) => match handle.path() {
                Some(path) => {
                    let path = path.to_string();

                    let re = match regex::Regex::new(pattern) {
                        Ok(re) => re,
                        Err(e) => {
                            debug!("failed to compile regular expression! {e}");
                            return false;
                        }
                    };

                    re.is_match(&path)
                }
                None => false,
            },
        }
    }
}
