use bevy::{prelude::Handle, sprite::TextureAtlas, text::Font};

mod assets_plugin;
pub mod startup_plugin;

pub struct InternalAssets {
    pub ground: Handle<TextureAtlas>,
    pub fish: Handle<TextureAtlas>,
    pub text_font: Handle<Font>,
}
