//! Asset-loading scaffold.
//!
//! On entering [`GameState::Loading`] this loads the tile atlas and the player's
//! paperdoll layer sheets, builds their [`TextureAtlasLayout`]s, and stores every
//! handle in [`GameAssets`]. Once all images have finished loading it advances to
//! [`GameState::Playing`], where [`crate::world`] and [`crate::character`] consume
//! the handles. Geometry constants for both atlases live here as the single source
//! of truth.

use bevy::prelude::*;
use mourhhold_core::GameState;

/// Edge length, in pixels, of one winter-forest tile (`tiles/winter_forest/48x48.png`).
pub(crate) const TILE_PX: u32 = 48;
/// Columns in the winter-forest tile atlas.
pub(crate) const FOREST_COLS: u32 = 21;
/// Rows in the winter-forest tile atlas.
pub(crate) const FOREST_ROWS: u32 = 21;

/// Size of one character frame. Every paperdoll layer sheet is 1584×1984 px,
/// i.e. a 33×31 grid of 48×64 cells, so one atlas index addresses the same pose
/// across every layer.
pub(crate) const CHAR_FRAME: UVec2 = UVec2::new(48, 64);
/// Columns in each character layer sheet.
pub(crate) const CHAR_COLS: u32 = 33;
/// Rows in each character layer sheet.
pub(crate) const CHAR_ROWS: u32 = 31;

/// Atlas index of the idle, facing-down player pose (row 1, column 2 — the first
/// bracket-free "down" frame). `row * CHAR_COLS + col` = `1 * 33 + 2`.
pub(crate) const PLAYER_IDLE_DOWN: usize = CHAR_COLS as usize + 2;

/// Player paperdoll layers, ordered **back-to-front** to match the art pack's
/// "HOW TO USE" layering: the cape sits behind the body and hair draws last.
const PLAYER_LAYER_PATHS: &[&str] = &[
    "characters/adventurer_set/cape/adventurer_cape.png",
    "characters/base_set/base/skin_1.png",
    "characters/adventurer_set/boots/leather_boots.png",
    "characters/adventurer_set/pants/brown_pants.png",
    "characters/adventurer_set/shirt/blue_shirt.png",
    "characters/adventurer_set/gloves/leather_gloves.png",
    "characters/adventurer_set/hair/adventurer_hair.png",
];

/// Loaded image and atlas-layout handles, shared across gameplay systems.
#[derive(Resource)]
pub(crate) struct GameAssets {
    /// The winter-forest tile sheet.
    pub(crate) tiles: Handle<Image>,
    /// Grid layout over [`GameAssets::tiles`].
    pub(crate) tile_layout: Handle<TextureAtlasLayout>,
    /// Grid layout shared by every player paperdoll layer.
    pub(crate) char_layout: Handle<TextureAtlasLayout>,
    /// Player paperdoll layer images, ordered back (drawn first) to front.
    pub(crate) player_layers: Vec<Handle<Image>>,
}

/// Loads assets during [`GameState::Loading`] and transitions to
/// [`GameState::Playing`] once they are ready.
pub(crate) struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(Update, finish_loading.run_if(in_state(GameState::Loading)));
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let tile_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_PX),
        FOREST_COLS,
        FOREST_ROWS,
        None,
        None,
    ));
    let char_layout = layouts.add(TextureAtlasLayout::from_grid(
        CHAR_FRAME, CHAR_COLS, CHAR_ROWS, None, None,
    ));

    let player_layers = PLAYER_LAYER_PATHS
        .iter()
        .map(|path| asset_server.load(*path))
        .collect();

    commands.insert_resource(GameAssets {
        tiles: asset_server.load("tiles/winter_forest/48x48.png"),
        tile_layout,
        char_layout,
        player_layers,
    });
}

fn finish_loading(
    assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let ready = asset_server.is_loaded_with_dependencies(assets.tiles.id())
        && assets
            .player_layers
            .iter()
            .all(|layer| asset_server.is_loaded_with_dependencies(layer.id()));

    if ready {
        next_state.set(GameState::Playing);
    }
}
