# Shieldtank

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/stinkytoe/shieldtank/tree/main#license)
[![docs.rs](https://img.shields.io/docsrs/shieldtank)](https://docs.rs/shieldtank/latest/shieldtank/)
[![Crates.io Version](https://img.shields.io/crates/v/shieldtank)](https://crates.io/crates/shieldtank/)
[![CI](https://github.com/stinkytoe/shieldtank/workflows/CI/badge.svg)](https://github.com/stinkytoe/shieldtank/actions)

<!--toc:start-->
- [Shieldtank](#shieldtank)
  - [Upcoming Features](#upcoming-features)
<!--toc:end-->

## Upcoming Features

- Custom Renderer for components
  - Level
    - Currently uses a [Sprite](https://docs.rs/bevy/latest/bevy/sprite/struct.Sprite.html)
    - Consider replacing with a custom renderer/shader
  - Layer
    - Currently uses a [Sprite](https://docs.rs/bevy/latest/bevy/sprite/struct.Sprite.html)
    - Strong candidate for a custom renderer
      - Use its own render mode, optionally independent of Bevy's ImagePlugin
      - Guarantee correct pixel render
      - Support user controlled parallax based on the LDtk hints
        - What will the interface look like?
  - Entity
    - Currently uses a [Sprite](https://docs.rs/bevy/latest/bevy/sprite/struct.Sprite.html)
      - Unlike other components, might be advantageous to continue to use Bevy's
      Sprite
    - 9-slice render mode

- Vast improvement of spawning of hierarchies
  - Finer control of asset sub loading
    - Currently only have an option of `All`/`None`/match against a regex
      - !!! Only `All` tested!
