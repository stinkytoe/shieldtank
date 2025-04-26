use crate::component::layer::LdtkLayer;

use super::component::ShieldtankComponentQuery;

pub type LdtkLayerQuery<'w, 's, D, F = ()> = ShieldtankComponentQuery<'w, 's, LdtkLayer, (), D, F>;
