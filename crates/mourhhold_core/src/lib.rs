//! Core gameplay layer for **Mourhhold**.
//!
//! This crate is deliberately renderer-agnostic: it owns the components,
//! resources, states, and pure logic that the game is built from, so it can be
//! unit-tested headlessly without spinning up a window or GPU.

use bevy::prelude::*;

/// Side length, in world units (pixels), of one logical tile.
///
/// Mourhhold renders as a **pure top-down** game, so one world unit is one
/// screen pixel and a grid cell maps to a `TILE_SIZE`×`TILE_SIZE` square. The
/// value matches the 48 px winter-forest tile art and the 48 px-wide character
/// frames, so tiles and actors line up on the same grid.
pub const TILE_SIZE: f32 = 48.0;

/// Base Z for grid-anchored objects (actors, props), kept above ground tiles.
pub const OBJECT_DEPTH_BASE: f32 = 10.0;
/// Z added per grid row so lower rows render in front of higher ones.
pub const DEPTH_PER_ROW: f32 = 1.0;

/// High-level application states that drive system scheduling.
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    /// Assets are loading; nothing interactive yet.
    #[default]
    Loading,
    /// Title / main menu.
    Menu,
    /// Active gameplay in the world of Ravenhollow.
    Playing,
}

/// A position on the logical tile grid.
///
/// Stored as integer column/row coordinates. Convert to/from top-down world
/// space with [`GridPosition::to_world`] and [`GridPosition::from_world`]:
/// grid X maps to world X (rightward) and grid Y maps to world Y, negated, so
/// that **increasing rows move *down* the screen** (Bevy's world Y points up).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GridPosition {
    /// Column on the grid (maps to world X; larger is further right).
    pub x: i32,
    /// Row on the grid (maps to world Y; larger is further *down* the screen).
    pub y: i32,
}

impl GridPosition {
    /// Creates a grid position from column/row coordinates.
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Projects this grid cell to its top-down world-space position (in pixels).
    ///
    /// The renderer adds the Z (depth) coordinate; see [`GridPosition::depth`].
    #[must_use]
    pub fn to_world(self) -> Vec2 {
        Vec2::new(self.x as f32 * TILE_SIZE, -(self.y as f32) * TILE_SIZE)
    }

    /// Recovers the nearest grid cell from a top-down world-space position.
    #[must_use]
    pub fn from_world(world: Vec2) -> Self {
        Self {
            x: (world.x / TILE_SIZE).round() as i32,
            y: (-world.y / TILE_SIZE).round() as i32,
        }
    }

    /// Painter's-order depth (Z) for an object standing on this cell.
    ///
    /// In a top-down view, things lower on the screen (greater row `y`) should
    /// draw *in front of* things higher up. Returning a Z that grows with `y`
    /// gives that ordering when used as the sprite's translation Z; ground tiles
    /// sit at Z 0, below every object.
    #[must_use]
    pub const fn depth(self) -> f32 {
        (self.y as f32).mul_add(DEPTH_PER_ROW, OBJECT_DEPTH_BASE)
    }
}

/// Wires the core gameplay layer into a Bevy [`App`].
///
/// Registers the [`GameState`] state machine. Rendering, input, and window
/// plugins are the responsibility of the binary.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_maps_to_world_origin() {
        assert_eq!(GridPosition::new(0, 0).to_world(), Vec2::ZERO);
    }

    #[test]
    fn rows_increase_downward_columns_rightward() {
        // +X column is to the right; +Y row is *down* (negative world Y).
        assert_eq!(
            GridPosition::new(1, 0).to_world(),
            Vec2::new(TILE_SIZE, 0.0)
        );
        assert_eq!(
            GridPosition::new(0, 1).to_world(),
            Vec2::new(0.0, -TILE_SIZE)
        );
    }

    #[test]
    fn world_round_trips_through_grid() {
        for x in -8..8 {
            for y in -8..8 {
                let cell = GridPosition::new(x, y);
                let world = cell.to_world();
                assert_eq!(GridPosition::from_world(world), cell, "round-trip {cell:?}");
            }
        }
    }

    #[test]
    fn lower_rows_render_in_front() {
        assert!(GridPosition::new(0, 5).depth() > GridPosition::new(0, 4).depth());
    }

    #[test]
    fn default_state_is_loading() {
        assert_eq!(GameState::default(), GameState::Loading);
    }
}
