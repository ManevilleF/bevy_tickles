use crate::particle::Particle;
use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Component, Handle, Image, Reflect, TextureAtlas};
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::sprite::Rect;
use rand::Rng;

/// Defines the looping behaviour of the animated sheet
#[derive(Debug, Copy, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum TextureSheetLoopingMode {
    /// The texture sheet is not animated
    None,
    /// The sheet loops by increasing the cell index and then going back to the first one
    Loop,
    // TODO: Implement additional looping modes
    // /// The sheet loops by decreasing the cell index and then going back to the first one
    // Reverse,
    // /// The sheet cell index goes forth and back, changing direction when reaching either the minimum index or the maximum index
    // PingPong,
    // /// The sheet cell index goes back and forth, changing direction when reaching either the minimum index or the maximum index
    // ReversePingPong,
}

/// Animation params for particle texture sheets
#[derive(Debug, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct TextureSheetAnimation {
    /// Start cell (usually 0)
    pub start_index: usize,
    /// Is the animation looping, meaning we can go back to `start_index` after we reached the end
    /// of the texture sheet
    pub looping_mode: TextureSheetLoopingMode,
    /// Cell transition ratio
    // TODO: Add doc example
    pub ratio: f32,
}

/// Texture Sheet resolve mode for particle systems
#[derive(Debug, Clone, Reflect)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum TextureSheetMode {
    /// Use a single cell of the texture sheet
    FixedIndex(usize),
    /// Use a random index of the texture sheet
    RandomIndex,
    /// Change the cell over time
    AnimateOverTime(TextureSheetAnimation),
    /// Change the cell over the particle lifetime
    AnimateOverLifetime(TextureSheetAnimation),
    /// Change the cell over the particle speed
    AnimateOverSpeed(TextureSheetAnimation),
}

/// Texture Sheet params for particle systems
#[derive(Debug, Clone, Reflect, Default)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub struct ParticleTextureSheet {
    /// Texture atlas handle
    pub texture_atlas: Handle<TextureAtlas>,
    /// Texture sheet resolve mode
    pub mode: TextureSheetMode,
}

/// The material of the particle, can be a texture or an animated texture sheet
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
pub enum ParticleMaterial {
    /// Single image material
    Image(Handle<Image>),
    /// Texture sheet material
    TextureSheet(ParticleTextureSheet),
}

impl Default for TextureSheetAnimation {
    fn default() -> Self {
        Self {
            start_index: 0,
            looping_mode: TextureSheetLoopingMode::None,
            ratio: 1.0,
        }
    }
}

impl Default for TextureSheetMode {
    fn default() -> Self {
        Self::AnimateOverLifetime(TextureSheetAnimation::default())
    }
}

impl Default for ParticleMaterial {
    fn default() -> Self {
        Self::Image(DEFAULT_IMAGE_HANDLE.typed())
    }
}

impl TextureSheetAnimation {
    /// Retrieves the texture cell bounds (`Rect`) of the texture sheet related to the given `particle`
    /// after computing the animation index
    pub fn rect(
        &self,
        texture_atlas: &TextureAtlas,
        particle: &Particle,
        delta: impl Fn(&Particle) -> f32,
    ) -> Rect {
        let delta = delta(particle) * self.ratio;
        let index = (texture_atlas.textures.len() as f32 * delta) as usize;
        match self.looping_mode {
            TextureSheetLoopingMode::None => {
                texture_atlas.textures[index.clamp(0, texture_atlas.textures.len() - 1)]
            }
            TextureSheetLoopingMode::Loop => {
                texture_atlas.textures[index % (texture_atlas.textures.len() - 1)]
            }
        }
    }
}

impl TextureSheetMode {
    /// Retrieves the texture cell bounds (`Rect`) of the texture sheet related to the given `particle`
    pub fn rect(
        &self,
        texture_atlas: &TextureAtlas,
        particle: &Particle,
        rng: &mut impl Rng,
    ) -> Rect {
        match self {
            TextureSheetMode::FixedIndex(i) => texture_atlas.textures[*i],
            TextureSheetMode::RandomIndex => {
                let index = rng.gen_range(0..texture_atlas.textures.len());
                texture_atlas.textures[index]
            }
            TextureSheetMode::AnimateOverLifetime(animation) => {
                animation.rect(texture_atlas, particle, Particle::alive_time_ratio)
            }
            TextureSheetMode::AnimateOverTime(animation) => {
                animation.rect(texture_atlas, particle, Particle::alive_time)
            }
            TextureSheetMode::AnimateOverSpeed(animation) => {
                animation.rect(texture_atlas, particle, Particle::speed)
            }
        }
    }
}
