use bevy_asset::Asset;
use bevy_asset::Handle;
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
    pub fn handle_matches_pattern<A: Asset>(&self, handle: &Handle<A>) -> bool {
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
