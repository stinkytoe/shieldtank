use bevy_app::Plugin;
use bevy_derive::Deref;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_reflect::Reflect;

#[derive(Debug, Component, Deref, Reflect)]
#[relationship(relationship_target = ShieldtankEntities)]
pub struct ParentShieldtankLayer(pub Entity);

#[derive(Debug, Component, Deref, Reflect)]
#[relationship(relationship_target = ShieldtankLayers)]
pub struct ParentShieldtankLevel(pub Entity);

#[derive(Debug, Component, Deref, Reflect)]
#[relationship(relationship_target = ShieldtankLevels)]
pub struct ParentShieldtankWorld(pub Entity);

#[derive(Debug, Component, Reflect)]
#[relationship_target(relationship = ParentShieldtankLayer, linked_spawn)]
pub struct ShieldtankEntities(Vec<Entity>);

#[derive(Debug, Component, Reflect)]
#[relationship_target(relationship = ParentShieldtankLevel, linked_spawn)]
pub struct ShieldtankLayers(Vec<Entity>);

#[derive(Debug, Component, Reflect)]
#[relationship_target(relationship = ParentShieldtankWorld, linked_spawn)]
pub struct ShieldtankLevels(Vec<Entity>);

pub struct RelationsPlugin;
impl Plugin for RelationsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app //
            .register_type::<ParentShieldtankLayer>()
            .register_type::<ParentShieldtankLevel>()
            .register_type::<ParentShieldtankWorld>()
            .register_type::<ShieldtankEntities>()
            .register_type::<ShieldtankLayers>()
            .register_type::<ShieldtankLevels>();
    }
}
