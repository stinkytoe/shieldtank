//use bevy::asset::Handle;
//use bevy_ldtk_asset::ldtk_asset_traits::LdtkAsset;
//
//use crate::load_pattern::LoadPattern;

//pub trait LdtkComponent: bevy::ecs::component::Component {
//    type Asset: LdtkAsset + std::fmt::Debug;
//
//    fn new(load_pattern: LoadPattern, handle: Handle<Self::Asset>) -> Self;
//    fn get_handle(&self) -> &Handle<Self::Asset>;
//    fn get_load_pattern(&self) -> &LoadPattern;
//}
//
//pub trait HasChildren: LdtkComponent {
//    type Child: LdtkComponent;
//
//    fn new_child(
//        &self,
//        //load_pattern: LoadPattern,
//        handle: Handle<<Self::Child as LdtkComponent>::Asset>,
//    ) -> Self::Child {
//        Self::Child::new(self.get_load_pattern().clone(), handle)
//    }
//}
