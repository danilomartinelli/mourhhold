//! The player character, drawn as a layered paperdoll.
//!
//! The character is one parent entity carrying its [`mourhhold_core::GridPosition`] and world
//! transform, with one child sprite per equipment layer. Every layer shares the
//! same atlas layout and frame index, so they line up pixel-for-pixel; their draw
//! order comes from a tiny per-layer Z offset (back-to-front, matching the order
//! the layers were loaded in [`crate::assets`]).

use bevy::prelude::*;
use mourhhold_core::GameState;

use crate::assets::{GameAssets, PLAYER_IDLE_DOWN};
use crate::world::PLAYER_SPAWN;

/// Z gap between consecutive paperdoll layers. Small enough that all layers stay
/// grouped within the actor's own depth slice (< one grid row apart).
const LAYER_STEP: f32 = 0.01;

/// Marks the player-controlled character (the paperdoll root).
#[derive(Component)]
pub(crate) struct Player;

/// Spawns the player when gameplay begins.
pub(crate) struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
    }
}

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    let translation = PLAYER_SPAWN.to_world().extend(PLAYER_SPAWN.depth());

    commands
        .spawn((
            Player,
            PLAYER_SPAWN,
            Transform::from_translation(translation),
            Visibility::default(),
        ))
        .with_children(|parent| {
            for (layer, image) in assets.player_layers.iter().enumerate() {
                parent.spawn((
                    Sprite::from_atlas_image(
                        image.clone(),
                        TextureAtlas {
                            layout: assets.char_layout.clone(),
                            index: PLAYER_IDLE_DOWN,
                        },
                    ),
                    Transform::from_xyz(0.0, 0.0, layer as f32 * LAYER_STEP),
                ));
            }
        });
}
