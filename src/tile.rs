//! Bitpacked tile representation. One [`GridTiles`] represents 8 tiles, each occupying 3 bits.
//! Very memory efficient, and most get/set operations are mostly composed of bitwise operations
//! hopefully making it very computationally efficient.

/// Tile
/// # Examples
/// ```
/// println!("Test")
/// ```
#[derive(Default)]
pub struct GridTiles {
    tile_data: [u8; 3],
}

/// Method builder macro for getters for individual bits within a tile.
/// `$offset` should be 0-2 otherwise you will access bits in other tiles
macro_rules! bit_getter_method_builder {
    [$name:ident, $offset:expr] => {
        /// Fetch the requested flag state from a specified tile
        pub fn $name(&self, index: usize) -> bool {
            assert!(index < 8, "Index out of range");

            let bit_position = index * 3 + $offset;
            let byte_index = bit_position / 8;
            let bit_offset = bit_position % 8;

            ((self.tile_data[byte_index].reverse_bits() >> bit_offset) & 1) != 0
        }
    };
}

/// Method builder macro for setters for individual bits within a tile.
/// `$offset` should be 0-2 otherwise you will access bits in other tiles
macro_rules! bit_setter_method_builder {
    [$name:ident, $offset:expr] => {
        /// Set the requested flag state for a specified tile
        #[inline]
        pub fn $name(&mut self, index: usize, bit: bool) -> () {
            assert!(index < 8, "Index out of range");

            let bit_position = index * 3 + $offset;
            let byte_index = bit_position / 8;
            let bit_offset = bit_position % 8;

            self.tile_data[byte_index] &= !(0b10000000 >> bit_offset);
            self.tile_data[byte_index] |= ((bit as u8) << 7) >> bit_offset;
        }
    };
}

impl GridTiles {
    /// Returns a byte containing the requested tile's data. The last 3 bits in the returned byte contain the requested data.
    pub fn get(&self, index: usize) -> u8 {
        assert!(index < 8, "Index out of range");

        let bit_position = index * 3;
        let byte_index = bit_position / 8;
        let bit_offset = bit_position % 8;

        let mut result = (self.tile_data[byte_index].reverse_bits() >> bit_offset) & 0b111;

        if bit_offset > 5 {
            result |= (self.tile_data[byte_index + 1].reverse_bits() << (8 - bit_offset)) & 0b111;
        }
        result
    }

    /// Set a specific tile's data. The last 3 bits of the given byte will be used to overwrite the tile bits.
    pub fn set(&mut self, index: usize, mut byte: u8) {
        assert!(index < 8, "Index out of range");
        byte = (byte & 0b111) << 5;

        let bit_position = index * 3;
        let byte_index = bit_position / 8;
        let bit_offset = bit_position % 8;

        self.tile_data[byte_index] &= !(0b11100000 >> bit_offset);
        self.tile_data[byte_index] |= byte >> bit_offset;

        if bit_offset > 5 {
            self.tile_data[byte_index + 1] &= !(0b11000000 << (7 - bit_offset));
            self.tile_data[byte_index + 1] |= byte << (8 - bit_offset);
        }
    }

    bit_getter_method_builder!(is_mine, 0);
    bit_getter_method_builder!(revealed, 1);
    bit_getter_method_builder!(flagged, 2);

    bit_setter_method_builder!(set_mine, 0);
    bit_setter_method_builder!(set_revealed, 1);
    bit_setter_method_builder!(set_flagged, 2);
}

#[test]
fn single_tile_flag() {
    for i in 0..8 {
        let mut tile = GridTiles::default();
        tile.set_mine(i, true);
        assert!(
            tile.is_mine(i),
            "Set mine flag, got back {}",
            tile.is_mine(i)
        );
        assert_eq!(
            tile.get(i),
            0b1,
            "Set mine flag, got back {:03b}",
            tile.get(i)
        );
    }

    for i in 0..8 {
        let mut tile = GridTiles::default();
        tile.set_revealed(i, true);
        assert!(
            tile.revealed(i),
            "Set revealed flag, got back {}",
            tile.revealed(i)
        );
        assert_eq!(
            tile.get(i),
            0b10,
            "Set revealed flag, got back {:03b}",
            tile.get(i)
        );
    }

    for i in 0..8 {
        let mut tile = GridTiles::default();
        tile.set_flagged(i, true);
        assert!(
            tile.flagged(i),
            "Set flagged flag, got back {}",
            tile.flagged(i)
        );
        assert_eq!(
            tile.get(i),
            0b100,
            "Set flagged flag, got back {:03b}",
            tile.get(i)
        );
    }
}

#[test]
fn mass_tile_flag() {
    for i in 0..8 {
        let mut tile = GridTiles::default();
        tile.set(i, 255);
        assert_eq!(
            tile.get(i),
            0b111,
            "Set all tile flags, got back {:03b}",
            tile.get(i)
        );
    }
}

#[test]
fn tile_flag_combinations() {
    let mut tile = GridTiles::default();

    tile.set(0, 0b111);
    tile.set_flagged(1, true);
    tile.set_mine(2, true);
    tile.set_mine(2, false);
    tile.set_revealed(2, true);
    tile.set_mine(3, true);
    tile.set_revealed(4, true);
    tile.set_flagged(5, true);
    tile.set(6, 0b101);
    tile.set(7, 0b110);

    assert_eq!(tile.tile_data, [0b11100101, 0b01000100, 0b01101110]);
}
