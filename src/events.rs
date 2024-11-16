//! Event and additional [`EventDispatcher`] implementations.

pub use crate::models::events::*;
use std::sync::Arc;

/// Event emitted when the first tile of the board is revealed (when the game starts)
pub struct GameStartEvent;
impl Event for GameStartEvent {
    type Context = ();
}

/// Event emitted when a game ends. The last tile last flagged or revealed is passed as context
pub struct GameEndEvent;
impl Event for GameEndEvent {
    type Context = Vec<usize>;
}

/// Event emitted when mines are placed on a board. The amount of mines successfully placed is
/// passed as context.
pub struct MinesPlacedEvent;
impl Event for MinesPlacedEvent {
    type Context = usize;
}

/// Event emitted when a tile is revealed on the board. The position of the tile revealed is
/// passed as context.
pub struct TileRevealedEvent;
impl Event for TileRevealedEvent {
    type Context = Vec<usize>;
}

/// Event emitted when a tile is flagged on the board. The position of the tile flagged is passed
/// as context.
pub struct TileFlaggedEvent;
impl Event for TileFlaggedEvent {
    type Context = Vec<usize>;
}

impl EventDispatcher {
    /// Register a callback for [`GameStartEvent`]
    pub fn on_start(&mut self, callback: impl Fn(()) + Send + Sync + 'static) {
        self.subscribe::<GameStartEvent>(Arc::new(callback));
    }

    /// Register a callback for [`GameEndEvent`]
    pub fn on_end(&mut self, callback: impl Fn(Vec<usize>) + Send + Sync + 'static) {
        self.subscribe::<GameEndEvent>(Arc::new(callback));
    }

    /// Register a callback for [`MinesPlacedEvent`]
    pub fn on_placed_mines(&mut self, callback: impl Fn(usize) + Send + Sync + 'static) {
        self.subscribe::<MinesPlacedEvent>(Arc::new(callback));
    }

    /// Register a callback for [`TileRevealedEvent`]
    pub fn on_tile_revealed(&mut self, callback: impl Fn(Vec<usize>) + Send + Sync + 'static) {
        self.subscribe::<TileRevealedEvent>(Arc::new(callback));
    }

    /// Register a callback for [`TileFlaggedEvent`]
    pub fn on_tile_flagged(&mut self, callback: impl Fn(Vec<usize>) + Send + Sync + 'static) {
        self.subscribe::<TileFlaggedEvent>(Arc::new(callback));
    }
}
