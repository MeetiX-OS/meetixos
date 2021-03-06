/*! Bit flags wrapper */

use core::{
    convert::TryFrom,
    fmt,
    fmt::{
        Binary,
        Debug
    },
    marker::PhantomData,
    ops::{
        BitAnd,
        BitAndAssign,
        BitOr,
        BitOrAssign,
        BitXor,
        BitXorAssign,
        Not
    }
};

use crate::bit_fields::TBitFields;

/**
 * Safe wrapper for a bit flags
 */
#[repr(transparent)]
pub struct BitFlags<B, T>
    where B: TBitFields + Default + Copy,
          T: TBitFlagsValues {
    m_raw_bits: B,
    _unused: PhantomData<T>
}

impl<B, T> BitFlags<B, T>
    where B: TBitFields + Default + Copy,
          T: TBitFlagsValues /* Constructors */
{
    /**
     * Constructs a zeroed `BitFlags`
     */
    pub fn new_zero() -> Self {
        Self { m_raw_bits: B::default(),
               _unused: PhantomData }
    }

    /**
     * Constructs a `BitFlags` interpreting the raw value given.
     *
     * Fails if `raw_bits` contains invalid bits
     */
    pub fn from_raw(raw_bits: B) -> Option<Self> {
        for bit_index in 0..B::BIT_LEN {
            if let Err(_) = T::try_from(bit_index) {
                return None;
            }
        }
        Some(Self { m_raw_bits: raw_bits,
                    _unused: PhantomData })
    }

    /**
     * Constructs a `BitFlags` interpreting the raw value given and
     * truncating out invalid bits
     */
    pub fn from_raw_truncate(mut raw_bits: B) -> Self {
        for bit_index in 0..B::BIT_LEN {
            if let Err(_) = T::try_from(bit_index) {
                raw_bits.set_bit(bit_index, false);
            }
        }

        Self { m_raw_bits: raw_bits,
               _unused: PhantomData }
    }
}

impl<B, T> BitFlags<B, T>
    where B: TBitFields + Default + Copy,
          T: TBitFlagsValues /* Getters */
{
    /**
     * Returns whether the bit corresponding to `bit` is enabled
     */
    #[inline]
    pub fn is_enabled(&self, bit: T) -> bool {
        let bit_index = bit.into();
        assert!(bit_index < B::BIT_LEN);

        self.m_raw_bits.bit_at(bit_index)
    }

    /**
     * Returns whether the bit corresponding to `value` is disabled
     */
    #[inline]
    pub fn is_disabled(&self, bit: T) -> bool {
        !self.is_enabled(bit)
    }

    /**
     * Returns whether any of the given bits is enabled
     */
    pub fn is_any_of(&self, bits: &[T]) -> bool {
        for &bit in bits {
            if self.is_enabled(bit) {
                return true;
            }
        }
        false
    }

    /**
     * Returns whether all the given bits are enabled
     */
    pub fn is_all_of(&self, bits: &[T]) -> bool {
        for &bit in bits {
            if self.is_disabled(bit) {
                return false;
            }
        }
        true
    }

    /**
     * Returns this `BitFlags` as raw value
     */
    #[inline]
    pub fn raw_bits(&self) -> B {
        self.m_raw_bits
    }
}

impl<B, T> BitFlags<B, T>
    where B: TBitFields + Default + Copy,
          T: TBitFlagsValues /* Setters */
{
    /**
     * Enables the bit corresponding to the given `bit`
     */
    #[inline]
    pub fn set_enabled(&mut self, bit: T) -> &mut Self {
        self.set(bit, true)
    }

    /**
     * Disables the bit corresponding to the given `bit`
     */
    #[inline]
    pub fn set_disabled(&mut self, bit: T) -> &mut Self {
        self.set(bit, false)
    }

    /**
     * Sets for the bit corresponding to `bit` the `bit_value` value
     */
    #[inline]
    pub fn set(&mut self, bit: T, bit_value: bool) -> &mut Self {
        let bit_index = bit.into();
        assert!(bit_index < B::BIT_LEN);

        self.m_raw_bits.set_bit(bit_index, bit_value);
        self
    }
}

impl<B, T> Clone for BitFlags<B, T>
    where B: TBitFields + Default + Copy,
          T: TBitFlagsValues
{
    #[inline]
    fn clone(&self) -> Self {
        Self { m_raw_bits: self.m_raw_bits,
               _unused: PhantomData }
    }
}

impl<B, T> Copy for BitFlags<B, T>
    where B: TBitFields + Default + Copy,
          T: TBitFlagsValues
{
    /* No methods to implement */
}

impl<B, T> Default for BitFlags<B, T>
    where B: TBitFields + Default + Copy,
          T: TBitFlagsValues
{
    fn default() -> Self {
        Self::new_zero()
    }
}

impl<B, T> From<B> for BitFlags<B, T>
    where B: TBitFields + Default + Copy,
          T: TBitFlagsValues
{
    fn from(raw_bits: B) -> Self {
        Self::from_raw_truncate(raw_bits)
    }
}

impl<B, T> BitOr for BitFlags<B, T>
    where B: TBitFields + Default + Copy + BitOr<Output = B>,
          T: TBitFlagsValues
{
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { m_raw_bits: self.m_raw_bits | rhs.m_raw_bits,
               _unused: PhantomData }
    }
}

impl<B, T> BitOrAssign for BitFlags<B, T>
    where B: TBitFields + Default + Copy + BitOrAssign,
          T: TBitFlagsValues
{
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.m_raw_bits |= rhs.m_raw_bits
    }
}

impl<B, T> BitOr<T> for BitFlags<B, T>
    where B: TBitFields + Default + Copy,
          T: TBitFlagsValues
{
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: T) -> Self::Output {
        let mut self_clone = self.clone();

        self_clone.set_enabled(rhs);
        self_clone
    }
}

impl<B, T> BitOrAssign<T> for BitFlags<B, T>
    where B: TBitFields + Default + Copy,
          T: TBitFlagsValues
{
    #[inline]
    fn bitor_assign(&mut self, rhs: T) {
        self.set_enabled(rhs);
    }
}

impl<B, T> BitAnd for BitFlags<B, T>
    where B: TBitFields + Default + Copy + BitAnd<Output = B>,
          T: TBitFlagsValues
{
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { m_raw_bits: self.m_raw_bits & rhs.m_raw_bits,
               _unused: PhantomData }
    }
}

impl<B, T> BitAndAssign for BitFlags<B, T>
    where B: TBitFields + Default + Copy + BitAndAssign,
          T: TBitFlagsValues
{
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.m_raw_bits &= rhs.m_raw_bits
    }
}

impl<B, T> BitAnd<T> for BitFlags<B, T>
    where B: TBitFields + Default + Copy,
          T: TBitFlagsValues
{
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: T) -> Self::Output {
        let mut self_zero = Self::new_zero();

        if self.is_enabled(rhs) {
            self_zero.set_enabled(rhs);
        }

        self_zero
    }
}

impl<B, T> BitAndAssign<T> for BitFlags<B, T>
    where B: TBitFields + Default + Copy,
          T: TBitFlagsValues
{
    #[inline]
    fn bitand_assign(&mut self, rhs: T) {
        self.set(rhs, self.is_enabled(rhs));
    }
}

impl<B, T> BitXor for BitFlags<B, T>
    where B: TBitFields + Default + Copy + BitXor<Output = B>,
          T: TBitFlagsValues
{
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { m_raw_bits: self.m_raw_bits ^ rhs.m_raw_bits,
               _unused: PhantomData }
    }
}

impl<B, T> BitXorAssign for BitFlags<B, T>
    where B: TBitFields + Default + Copy + BitXorAssign,
          T: TBitFlagsValues
{
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.m_raw_bits ^= rhs.m_raw_bits;
    }
}

impl<B, T> BitXor<T> for BitFlags<B, T>
    where B: TBitFields + Default + Copy + BitXor<Output = B>,
          T: TBitFlagsValues
{
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: T) -> Self::Output {
        let mut rhs_bf = Self::new_zero();
        rhs_bf.set_enabled(rhs);

        Self { m_raw_bits: self.m_raw_bits ^ rhs_bf.m_raw_bits,
               _unused: PhantomData }
    }
}

impl<B, T> BitXorAssign<T> for BitFlags<B, T>
    where B: TBitFields + Default + Copy + BitXorAssign,
          T: TBitFlagsValues
{
    #[inline]
    fn bitxor_assign(&mut self, rhs: T) {
        let mut rhs_bf = Self::new_zero();
        rhs_bf.set_enabled(rhs);

        self.m_raw_bits ^= rhs_bf.m_raw_bits;
    }
}

impl<B, T> Not for BitFlags<B, T>
    where B: TBitFields + Default + Copy + Not<Output = B>,
          T: TBitFlagsValues
{
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self { m_raw_bits: !self.m_raw_bits,
               _unused: PhantomData }
    }
}

impl<B, T> PartialEq for BitFlags<B, T>
    where B: TBitFields + Default + Copy + PartialEq,
          T: TBitFlagsValues
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.m_raw_bits == other.m_raw_bits
    }
}

impl<B, T> Eq for BitFlags<B, T>
    where B: TBitFields + Default + Copy + PartialEq,
          T: TBitFlagsValues
{
    /* No methods to implement */
}

impl<B, T> Debug for BitFlags<B, T>
    where B: TBitFields + Default + Copy + Binary,
          T: TBitFlagsValues + Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitFlags {{ m_bits: ")?;

        let mut is_first = true;
        for bit_index in 0..B::BIT_LEN {
            if self.m_raw_bits.bit_at(bit_index) {
                /* here we safe construct the value using <TryFrom> but if we come at
                 * this point is ensured that each bit enabled in <m_bits> corresponds
                 * to a valid variant of <T>
                 */
                if let Ok(bit_value) = T::try_from(bit_index) {
                    /* write the pipe before only if is not the first */
                    if !is_first {
                        write!(f, " | ")?;
                    } else {
                        is_first = false;
                    }

                    /* write the current variant with his value now */
                    write!(f, "{:?}", bit_value)?;
                }
            }
        }

        /* if <is_first> is still true means that no bits are present */
        if is_first {
            write!(f, "0")?;
        }
        write!(f, " }}")
    }
}

/**
 * Interface which must be implemented by the enumeration values
 */
pub trait TBitFlagsValues: Copy + Clone + Into<usize> + TryFrom<usize> {
    /* No additional methods are requested */
}
