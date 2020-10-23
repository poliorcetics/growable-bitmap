//! A crate providing a growable compact boolean array.
//!
//! See the `GrowableBitMap` type for more information.
use std::fmt;

/// A growable compact boolean array.
///
/// Bits are stored contiguously. The first value is packed into the least
/// significant bits of the first word of the backing storage.
///
/// # Caveats
///
/// - The `GrowableBitMap::set_bit` method may allocate way too much memory
///   compared to what you really need (if for example, you only plan to set
///   the bits between 1200 and 1400). In this case, storing the offset of
///   1200 somewhere else and storing the values in the range `0..=200` in the
///   `GrowableBitMap` is probably the most efficient solution.
/// - Right now the only implemented storage integer is `u8`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GrowableBitMap {
    // The storage for the bits.
    bits: Vec<u8>,
}

impl fmt::Debug for GrowableBitMap {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_list().entries(self.bits.iter()).finish()
    }
}

impl GrowableBitMap {
    // Number of bits that can be stored in one instance of the backend type.
    const BITS_BY_STORAGE: usize = 8;

    /// Creates a new GrowableBitMap.
    ///
    /// ```rust
    /// use growable_bitmap::GrowableBitMap;
    ///
    /// assert!(GrowableBitMap::new().is_empty());
    /// ```
    pub const fn new() -> Self {
        Self { bits: Vec::new() }
    }

    /// `true` if the GrowableBitMap is empty.
    ///
    /// ```rust
    /// use growable_bitmap::GrowableBitMap;
    ///
    /// assert!(GrowableBitMap::new().is_empty());
    ///
    /// let mut b = GrowableBitMap::new();
    /// b.set_bit(3);
    /// assert!(!b.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty() || self.bits.iter().all(|bits| *bits == 0)
    }

    /// Sets the bit at the given index and returns whether the bit was set
    /// to 1 by this call or not.
    ///
    /// ```rust
    /// use growable_bitmap::GrowableBitMap;
    ///
    /// let mut b = GrowableBitMap::new();
    /// assert!(b.set_bit(0)); // Bit 0 was not set before, returns true.
    /// assert!(!b.set_bit(0)); // Bit 0 was already set, returns false.
    ///
    /// assert!(b.set_bit(10)); // The bitmap will grow as needed to set the bit.
    /// ```
    ///
    /// > Note: This will grow the backing storage as needed to have enough
    /// > storage for the given index. If you set the bit 12800 with a
    /// > storage of `u8`s the backing storage will allocate 1600 `u8`s since
    /// > `sizeof::<u8>() == 1` byte.
    ///
    /// See also the `Caveats` section on `GrowableBitMap`.
    pub fn set_bit(&mut self, index: usize) -> bool {
        let bits_index = index / Self::BITS_BY_STORAGE;

        // Ensure there are enough elements in the `bits` storage.
        if self.bits.len() >= bits_index {
            self.bits.resize_with(bits_index + 1, Default::default);
        }

        let elem = &mut self.bits[bits_index];

        let mask = 1 << (index - bits_index * 8);
        let prev = *elem & mask;

        *elem |= mask;

        // If prev is 0, it means the bit was set by this call.
        prev == 0
    }

    /// Clears the bit at the given index and returns whether the bit was set
    /// to 0 by this call or not.
    ///
    /// ```rust
    /// use growable_bitmap::GrowableBitMap;
    ///
    /// let mut b = GrowableBitMap::new();
    /// assert!(!b.clear_bit(0)); // Bit 0 was not set before, returns false.
    ///
    /// b.set_bit(0);
    /// assert!(b.clear_bit(0));
    /// ```
    ///
    /// > Note: this function will never allocate nor free memory, even when
    /// > the bit being cleared is the last 1 in the value at the end of the
    /// > backing storage.
    pub fn clear_bit(&mut self, index: usize) -> bool {
        let bits_index = index / Self::BITS_BY_STORAGE;

        // Since the bits_index does not exist in the storage, the bit at
        // `index` is logically 0.
        if self.bits.len() <= bits_index {
            return false;
        }

        let elem = &mut self.bits[bits_index];

        let mask = 1 << (index - bits_index);
        let prev = *elem | !mask;

        *elem &= !mask;

        prev != 0
    }
}
