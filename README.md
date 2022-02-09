<!-- cargo-sync-readme start -->

# bevy particles

[![workflow](https://github.com/ManevilleF/bevy_particles/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/bevy_particles/actions/workflows/rust.yml)

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Particle systems plugin for [bevy](https://bevyengine.org)

> This is a work in progress with many missing features, it is not suitable for production

<!-- cargo-sync-readme end -->

## Usage

Add `ParticlesPlugin` to your bevy `App`

```rust
use bevy::prelude::*;
use bevy_particles::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticlesPlugin)
        .run();
}
```

You can then use `ParticleSystemBundle` to spawn particle systems.

> Note: The particle modifiers are not included in the bundle, `insert` the modifiers you want to the particle system entity. (See the [example](examples/basic_example.rs))

## Features

1. `inspector`

This feature enables [bevy_inspector_egui](https://github.com/jakobhellermann/bevy-inspector-egui) integration, allowing dynamic customization of your particle systems

## Examples

1. [Basic example](examples/basic_example.rs)

Run with `cargo run --example basic_example --features inspector`

![Alt](./docs/basic_example.gif "basic_example gif")

2. [Animated example](examples/animated_example.rs)

Run with `cargo run --example animated_example --features inspector`

![Alt](./docs/animated_example.gif "animated_example gif")

3. [Explosion example](examples/explosion_example.rs)

Run with `cargo run --example explosion_example --features inspector`

![Alt](./docs/explosion_example.gif "explosion_example gif")

## TODO:

- [x] computed visibility with AAB
- [x] Duration/Looping/Infinite particle emission
- [ ] Curves and gradients implementation
- [ ] Complete modifier list
- [ ] Sub Emitters and callbacks (trails/death)
- [ ] Curves
- [x] Color gradients
- [ ] Multi camera support
- [ ] Lit particles
- [x] Perlin noise
- [ ] Uniform random emission
- [ ] examples of classic particle systems:
  - [ ] Fire
  - [x] Explosion
  - [ ] Sci Fi Shield

Contributions welcome !