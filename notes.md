# Notes

```rust
impl LdtkQuery {
  pub fn get_projects(&self) -> impl Iterator<Item = ProjectItem>;
  pub fn get_worlds(&self) -> impl Iterator<Item = WorldItem>;
  pub fn get_levels(&self) -> impl Iterator<Item = LevelItem>;
  pub fn get_layers(&self) -> impl Iterator<Item = LayerItem>;
  pub fn get_entities(&self) -> impl Iterator<Item = EntityItem>;
}

impl ProjectItem {
  pub fn get_worlds(&self) -> impl Iterator<Item = WorldItem>;

  pub fn location(&self) -> Vec2;
}

impl WorldItem {
  pub fn get_project(&self) -> Option<ProjectItem>;
  pub fn get_levels(&self) -> impl Iterator<Item = LevelItem>;

  pub fn levels_at(&self, location: Vec2) -> impl Iterator<Item = LevelItem>;
  pub fn layers_at(&self, location: Vec2) -> impl Iterator<Item = LayerItem>;
  pub fn entities_at(&self, location: Vec2) -> impl Iterator<Item = EntityItem>;

  pub fn location(&self) -> Vec2;
  pub fn location_in_project(&self) -> Vec2;

  pub fn int_grid_values_at(&self, location: Vec2) -> impl Iterator<Item = IntGridValue>;
  pub fn int_grid_value_at(&self, location: Vec2) -> Option<IntGridValue>;

}

impl LevelItem {
  pub fn get_project(&self) -> Option<ProjectItem>;
  pub fn get_world(&self) -> Option<WorldItem>;
  pub fn get_layers(&self) -> impl Iterator<Item = LayerItem>;

  pub fn layers_at(&self, location: Vec2) -> impl Iterator<Item = LayerItem>;
  pub fn entities_at(&self, location: Vec2) -> impl Iterator<Item = EntityItem>;

  pub fn location(&self) -> Vec2;
  pub fn location_in_project(&self) -> Vec2;
  pub fn location_in_world(&self) -> Vec2;

  pub fn contains_project_location(&self, location: Vec2) -> bool;
  pub fn contains_world_location(&self, location: Vec2) -> bool;
}

impl LayerItem {
  pub fn get_project(&self) -> Option<ProjectItem>;
  pub fn get_world(&self) -> Option<WorldItem>;
  pub fn get_level(&self) ->  Option<LevelItem>;
  pub fn get_entities(&self) -> Option<EntityItem>;

  pub fn entities_at(&self, location: Vec2) -> impl Iterator<Item = EntityItem>;

  pub fn location(&self) -> Vec2;
  pub fn location_in_project(&self) -> Vec2;
  pub fn location_in_world(&self) -> Vec2;
  pub fn location_in_level(&self) -> Vec2;

  pub fn contains_project_location(&self, location: Vec2) -> bool;
  pub fn contains_world_location(&self, location: Vec2) -> bool;
  pub fn contains_level_location(&self, location: Vec2) -> bool;
}

impl EntityItem {
  pub fn get_project(&self) -> Option<ProjectItem>;
  pub fn get_world(&self) -> Option<WorldItem>;
  pub fn get_level(&self) ->  Option<LevelItem>;
  pub fn get_layer(&self) ->  Option<LayerItem>;

  pub fn location(&self) -> Vec2;
  pub fn location_in_project(&self) -> Vec2;
  pub fn location_in_world(&self) -> Vec2;
  pub fn location_in_level(&self) -> Vec2;
  pub fn location_in_layer(&self) -> Vec2;

  pub fn contains_project_location(&self, location: Vec2) -> bool;
  pub fn contains_world_location(&self, location: Vec2) -> bool;
  pub fn contains_level_location(&self, location: Vec2) -> bool;
  pub fn contains_layer_location(&self, location: Vec2) -> bool;
}
```
