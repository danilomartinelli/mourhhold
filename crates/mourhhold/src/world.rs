//! The playable world: a winter-forest tile map plus the 2D camera.
//!
//! On entering [`GameState::Playing`] this fills the map with snow ground, then
//! deterministically scatters props (bushes, rocks, stumps, branches) and pine
//! trees from the winter-forest atlas. Placement is driven by a position hash, so
//! the map is identical every run without pulling in an RNG dependency.

use bevy::prelude::*;
use mourhhold_core::{GameState, GridPosition};

use crate::assets::{FOREST_COLS, GameAssets};

/// Map width in tiles.
pub(crate) const MAP_W: i32 = 25;
/// Map height in tiles.
pub(crate) const MAP_H: i32 = 17;
/// Where the player starts (map centre).
pub(crate) const PLAYER_SPAWN: GridPosition = GridPosition::new(MAP_W / 2, MAP_H / 2);

/// Atlas index of the seamless snow floor tile.
const GROUND_SNOW: usize = 0;

/// Single-tile decorations scattered on the snow.
const PROPS: &[usize] = &[
    69,  // snowy bush
    70,  // snowy bush (alt)
    1,   // small snow rock
    23,  // snow rock cluster
    24,  // larger snow rock
    82,  // tree stump
    133, // fallen branch
    153, // fallen log
];

/// A multi-tile prop stamped as a rectangular block of atlas tiles.
struct TreePrefab {
    /// Atlas index of the block's top-left tile.
    top_left: usize,
    /// Block width / height in tiles.
    cols: i32,
    rows: i32,
    /// Anchor cell within the block (the trunk base), as column/row offsets.
    anchor_col: i32,
    anchor_row: i32,
}

const SNOWY_PINE: TreePrefab = TreePrefab {
    top_left: 92,
    cols: 3,
    rows: 4,
    anchor_col: 1,
    anchor_row: 3,
};
const GREEN_PINE: TreePrefab = TreePrefab {
    top_left: 95,
    cols: 3,
    rows: 4,
    anchor_col: 1,
    anchor_row: 3,
};

/// Spawns the camera and map when gameplay begins.
pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (setup_camera, spawn_map));
    }
}

fn setup_camera(mut commands: Commands) {
    let centre = PLAYER_SPAWN.to_world();
    commands.spawn((Camera2d, Transform::from_translation(centre.extend(1000.0))));
}

fn spawn_map(mut commands: Commands, assets: Res<GameAssets>) {
    for y in 0..MAP_H {
        for x in 0..MAP_W {
            let cell = GridPosition::new(x, y);
            spawn_tile(&mut commands, &assets, cell, GROUND_SNOW, 0.0);

            match decoration_at(cell) {
                Decoration::None => {}
                Decoration::Prop(index) => {
                    spawn_tile(&mut commands, &assets, cell, index, cell.depth());
                }
                Decoration::Tree(prefab) => spawn_tree(&mut commands, &assets, cell, prefab),
            }
        }
    }
}

/// What, if anything, decorates a given cell.
enum Decoration {
    None,
    Prop(usize),
    Tree(&'static TreePrefab),
}

fn decoration_at(cell: GridPosition) -> Decoration {
    // Keep the spawn area and the 1-tile map border clear.
    if cell.x == 0
        || cell.y == 0
        || cell.x == MAP_W - 1
        || cell.y == MAP_H - 1
        || chebyshev(cell, PLAYER_SPAWN) <= 2
    {
        return Decoration::None;
    }

    let roll = cell_hash(cell.x, cell.y) % 100;
    if roll < 7 && tree_fits(cell) {
        let prefab = if roll.is_multiple_of(2) {
            &SNOWY_PINE
        } else {
            &GREEN_PINE
        };
        Decoration::Tree(prefab)
    } else if roll < 20 {
        let index = PROPS[(cell_hash(cell.y, cell.x) as usize) % PROPS.len()];
        Decoration::Prop(index)
    } else {
        Decoration::None
    }
}

const fn tree_fits(anchor: GridPosition) -> bool {
    let prefab = &SNOWY_PINE;
    let left = anchor.x - prefab.anchor_col;
    let right = anchor.x + (prefab.cols - 1 - prefab.anchor_col);
    let top = anchor.y - prefab.anchor_row;
    let bottom = anchor.y + (prefab.rows - 1 - prefab.anchor_row);
    left >= 0 && right < MAP_W && top >= 0 && bottom < MAP_H
}

fn spawn_tree(
    commands: &mut Commands,
    assets: &GameAssets,
    anchor: GridPosition,
    prefab: &TreePrefab,
) {
    // The whole tree sorts as one object at its trunk-base depth, so an actor a
    // row lower draws in front of it and a row higher draws behind the canopy.
    let depth = anchor.depth();
    for drow in 0..prefab.rows {
        for dcol in 0..prefab.cols {
            let offset = drow * FOREST_COLS.cast_signed() + dcol;
            let Ok(block) = usize::try_from(offset) else {
                continue;
            };
            let cell = GridPosition::new(
                anchor.x + dcol - prefab.anchor_col,
                anchor.y + drow - prefab.anchor_row,
            );
            spawn_tile(commands, assets, cell, prefab.top_left + block, depth);
        }
    }
}

fn spawn_tile(
    commands: &mut Commands,
    assets: &GameAssets,
    cell: GridPosition,
    index: usize,
    z: f32,
) {
    commands.spawn((
        cell,
        Sprite::from_atlas_image(
            assets.tiles.clone(),
            TextureAtlas {
                layout: assets.tile_layout.clone(),
                index,
            },
        ),
        Transform::from_translation(cell.to_world().extend(z)),
    ));
}

fn chebyshev(a: GridPosition, b: GridPosition) -> i32 {
    (a.x - b.x).abs().max((a.y - b.y).abs())
}

/// Deterministic per-cell hash (SplitMix64-style finaliser) used for scatter.
fn cell_hash(x: i32, y: i32) -> u32 {
    let key = (u64::from(x.cast_unsigned()) << 32) | u64::from(y.cast_unsigned());
    let mut h = key.wrapping_add(0x9E37_79B9_7F4A_7C15);
    h = (h ^ (h >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h = (h ^ (h >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    h ^= h >> 31;
    h as u32
}
