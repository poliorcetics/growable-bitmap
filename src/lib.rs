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
    // Named constand to clarify bit shifts in `(set|clear)_bit`.
    const BITS_IN_BYTE: usize = 8;
    // Number of bits that can be stored in one instance of the backend type.
    const BITS_BY_STORAGE: usize = 8;

    /// Creates a new, empty `GrowableBitMap`.
    ///
    /// This does not allocate anything.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::GrowableBitMap;
    ///
    /// assert!(GrowableBitMap::new().is_empty());
    /// ```
    pub const fn new() -> Self {
        Self { bits: Vec::new() }
    }

    /// Constructs a new, empty `GrowableBitMap` with the specified capacity
    /// **in bits**.
    ///
    /// When `capacity` is zero, nothing is allocated.
    ///
    /// When `capacity` is not zero, the bit `capacity - 1` can be set without
    /// any other allocation and the returned `GrowableBitMap` is guaranteed
    /// to be able to hold `capacity` bits without reallocating (and maybe more
    /// if the given `capacity` is not a multiple of the number of bits in one
    /// instance of the backing storage).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::GrowableBitMap;
    ///
    /// let mut b = GrowableBitMap::with_capacity(8);
    /// assert!(b.is_empty());
    /// assert_eq!(b.capacity(), 8);
    ///
    /// b.set_bit(7);
    /// assert_eq!(b.capacity(), 8);
    ///
    /// b.set_bit(10);
    /// assert!(b.capacity() >= 8);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }

        let div = capacity / Self::BITS_BY_STORAGE;
        // Ensures the allocated capacity is enough for values like 125 with a
        // storage of `u8`:
        //
        // - `div` is 15
        // - `capacity % Self::BITS_BY_STORAGE` is 5 so `rem` is 1.
        //
        // The final capacity will be 16 `u8`s -> 128 bits, enough for the
        // 125 bits asked for.
        let rem = (capacity % Self::BITS_BY_STORAGE != 0) as usize;

        Self {
            bits: Vec::with_capacity(div + rem),
        }
    }

    /// `true` if the GrowableBitMap is empty.
    ///
    /// # Examples
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

    /// Gets the bit at the given index and returns `true` when it is set to 1.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::GrowableBitMap;
    ///
    /// let mut b = GrowableBitMap::new();
    /// assert!(!b.get_bit(0));
    /// assert!(!b.get_bit(15));
    ///
    /// b.set_bit(15);
    /// assert!(!b.get_bit(0));
    /// assert!(b.get_bit(15));
    /// ```
    pub fn get_bit(&self, index: usize) -> bool {
        let bits_index = index / Self::BITS_BY_STORAGE;

        // Since the bits_index does not exist in the storage, the bit at
        // `index` is logically 0.
        if self.bits.len() <= bits_index {
            return false;
        }

        let elem = self.bits[bits_index];
        let mask = 1 << (index - bits_index * Self::BITS_IN_BYTE);

        (elem & mask) != 0
    }

    /// Sets the bit at the given index and returns whether the bit was set
    /// to 1 by this call or not.
    ///
    /// # Examples
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
    /// Note: This will grow the backing storage as needed to have enough
    /// storage for the given index. If you set the bit 12800 with a storage of
    /// `u8`s the backing storage will allocate 1600 `u8`s since
    /// `sizeof::<u8>() == 1` byte.
    ///
    /// See also the `Caveats` section on `GrowableBitMap`.
    pub fn set_bit(&mut self, index: usize) -> bool {
        let bits_index = index / Self::BITS_BY_STORAGE;

        // Ensure there are enough elements in the `bits` storage.
        if self.bits.len() <= bits_index {
            self.bits.resize(bits_index + 1, 0);
        }

        let elem = &mut self.bits[bits_index];

        let mask = 1 << (index - bits_index * Self::BITS_IN_BYTE);
        let prev = *elem & mask;

        *elem |= mask;

        // If prev is 0, it means the bit was set by this call.
        prev == 0
    }

    /// Clears the bit at the given index and returns whether the bit was set
    /// to 0 by this call or not.
    ///
    /// # Examples
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
    /// Note: this function will never allocate nor free memory, even when
    /// the bit being cleared is the last 1 in the value at the end of the
    /// backing storage.
    pub fn clear_bit(&mut self, index: usize) -> bool {
        let bits_index = index / Self::BITS_BY_STORAGE;

        // Since the bits_index does not exist in the storage, the bit at
        // `index` is logically 0.
        if self.bits.len() <= bits_index {
            return false;
        }

        let elem = &mut self.bits[bits_index];

        let mask = 1 << (index - bits_index * Self::BITS_IN_BYTE);
        let prev = *elem | !mask;

        *elem &= !mask;

        prev != 0
    }

    /// Clears the bitmap, removing all values.
    ///
    /// This method has no effect on the allocated capacity of the bitmap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::GrowableBitMap;
    ///
    /// let mut b = GrowableBitMap::new();
    /// b.set_bit(4);
    ///
    /// assert!(!b.is_empty());
    /// b.clear();
    /// assert!(b.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.bits.clear();
    }

    /// Counts the number of bits that are set to 1 in the whole bitmap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::GrowableBitMap;
    ///
    /// let mut b = GrowableBitMap::new();
    /// assert_eq!(b.count_ones(), 0);
    ///
    /// b.set_bit(2);
    /// assert_eq!(b.count_ones(), 1);
    ///
    /// b.set_bit(9);
    /// assert_eq!(b.count_ones(), 2);
    /// ```
    pub fn count_ones(&self) -> usize {
        self.bits
            .iter()
            .map(|&store| store.count_ones() as usize)
            .sum::<usize>()
    }

    /// Returns the number of bits the bitmap can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::GrowableBitMap;
    ///
    /// let mut b = GrowableBitMap::new();
    ///
    /// assert_eq!(b.capacity(), 0);
    /// b.set_bit(125);
    /// assert_eq!(b.capacity(), 128);
    /// ```
    pub fn capacity(&self) -> usize {
        self.bits.capacity() * Self::BITS_BY_STORAGE
    }

    /// Shrinks the capacity of the `GrowableBitMap` as much as possible.
    ///
    /// It will drop down as close as possible to the length needed to store
    /// the last bit set to 1 and not more but the allocator may still inform
    /// the bitmap that there is space for a few more elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use growable_bitmap::GrowableBitMap;
    ///
    /// let mut b = GrowableBitMap::with_capacity(125);
    ///
    /// b.set_bit(63);
    /// b.set_bit(127);
    /// // assert_eq!(b.capacity(), 128);
    ///
    /// b.clear_bit(127);
    /// b.shrink_to_fit();
    /// assert_eq!(b.capacity(), 64);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        // Ignoring the values at the end that are 0.
        let last_set_bit_index = self
            .bits
            .iter()
            .rev()
            .skip_while(|&&store| store == 0)
            .count();
        self.bits.truncate(last_set_bit_index);
        self.bits.shrink_to_fit();
    }
}
