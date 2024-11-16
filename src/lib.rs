//! Hypersweeper is a library and component of a passion project (and mild obsession) of mine to build the
//! best version of minesweeper possible. This crate contains implementations (and hopefully traits in the
//! near future) for efficient, infinitely expandable minesweeper boards. Infinitely expandable as in any
//! size board, in any dimension (hence the name, *hyper*sweeper).
//!
//! # Hypersweeper contents
//! Hypersweeper contains implementations (and traits Soon) for high efficiency, multi-dimensional boards
//! as well as hopefully easy to use APIs for interacting with them. This crate serves as a backend for
//! minesweeper games, so all the logic and computations for running a game are provided in this crate.
//! Hypersweeper does not contain any code for a user interface. It only contains the logical and computational
//! aspects of a minesweeper game, leaving any graphical design choices up to the user of this crate.
//!
//! # Events system
//! I honestly don't know why but literally every version of minesweeper I have ever encountered updates the
//! entire damn board every frame and that pisses me off. So, this crate comes with a basic events system
//! that is triggered on interaction with the board. This allows for any important events or updates to be
//! detected by the user of this crate without having to check the entire state of the board for changes.
//! Check out the [events] module for the provided events and how to use them.
//!
//! # Why did you make this literally no one cares
//! I don't think anyone will ever see this to ask me this but like I said this is a passion project built
//! for fun. However, I did start this project with the hopes that I could make a better version of minesweeper
//! with all my favourite features (I couldn't find a good singular version to play) that *ran efficiently*.
//! I really liked [4D Minesweeper](https://store.steampowered.com/app/787980/4D_Minesweeper/) on Steam, but
//! it is not that well optimized from what I can tell, and it also lacked some features I really wish it had.
//! So, that's why I did this. To make a fully featured, highly efficient, multi-dimensional minesweeper clone.

pub mod events;
pub mod models;
pub mod tile;
