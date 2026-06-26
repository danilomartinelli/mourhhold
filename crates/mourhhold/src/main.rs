//! Mourhhold — game binary entry point.
//!
//! Wires Bevy's [`DefaultPlugins`] together with the renderer-agnostic
//! [`mourhhold_core::CorePlugin`] and the binary's own gameplay plugins
//! (asset loading, the winter-forest world, and the player character), then
//! boots a windowed program.

// On Windows, don't pop up a console window alongside the game in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod character;
mod world;

use bevy::prelude::*;
use mourhhold_core::CorePlugin;

use crate::assets::AssetsPlugin;
use crate::character::CharacterPlugin;
use crate::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Mourhhold".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.07, 0.09, 0.13)))
        .add_plugins(CorePlugin)
        .add_plugins((AssetsPlugin, WorldPlugin, CharacterPlugin))
        .run();
}
