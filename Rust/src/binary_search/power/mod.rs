pub mod utility {
    // Some helper functions
    #[inline]
    pub const fn is_power_of_two(n: usize) -> bool {
        return if n > 0 { (n & (n - 1)) == 0 } else { true };
    }

    #[inline]
    pub const fn log2(n: usize) -> Option<u32> {
        if n != 0 {
            return Some(63u32 - n.leading_zeros());
        } else {
            return None;
        }
    }

    #[inline]
    pub fn previous_power_of_two(n: usize) -> usize {
        if is_power_of_two(n) {
            return n >> 1;
        } else {
            return 1 << log2(n).unwrap();
        }
    }
}

use crate::binary_search::{DataTypes, IndexTypes, UnsignedIndexType};
use crate::binary_search::{DynamicBinarySearch, StaticBinarySearch};

trait DynamicPower<D, I>: DataTypes<D> + UnsignedIndexType<I>
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned + num::Integer,
{
    fn without_bound_check(data: &[D], value: D) -> Option<I>;
    fn with_bound_check(data: &[D], value: D) -> Option<I>;
}

trait StaticPower<D, I, const N: usize>: DataTypes<D> + UnsignedIndexType<I>
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned + num::Integer,
{
    fn without_bound_check(data: &[D; N], value: D) -> Option<I>;
    fn with_bound_check(data: &[D; N], value: D) -> Option<I>;
}

pub struct DynamicImplementation;

pub struct StaticImplementation;

impl<D: std::cmp::PartialOrd> DataTypes<D> for DynamicImplementation {}
impl<I: num::Integer> IndexTypes<I> for DynamicImplementation {}
impl<I: num::Integer + num_traits::Unsigned> UnsignedIndexType<I> for DynamicImplementation {}

impl<D: std::cmp::PartialOrd> DataTypes<D> for StaticImplementation {}
impl<I: num::Integer> IndexTypes<I> for StaticImplementation {}
impl<I: num::Integer + num_traits::Unsigned> UnsignedIndexType<I> for StaticImplementation {}

impl<D, I> DynamicPower<D, I> for DynamicImplementation
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num_traits::NumCast
        + std::ops::BitOr<Output = I>
        + std::ops::ShrAssign<u16>
        + std::marker::Copy,
{
    #[inline]
    fn without_bound_check(data: &[D], value: D) -> Option<I> {
        use num_traits::cast::cast;

        let half_power: usize = self::utility::previous_power_of_two(data.len());
        let mut low: I = cast(0).unwrap();
        let mut width: I = cast(half_power).unwrap();
        while width > cast(0).unwrap() {
            let mid: I = low | width;
            let index: usize = cast(mid).unwrap();
            if data[index] <= value {
                low = mid;
            }
            width >>= 1u16;
        }
        let index: usize = cast(low).unwrap();
        if data[index] == value {
            return Some(low);
        }
        return None;
    }

    #[inline]
    fn with_bound_check(data: &[D], value: D) -> Option<I> {
        use num_traits::cast::cast;

        let half_power: usize = self::utility::previous_power_of_two(data.len());
        let mut low: I = cast(0).unwrap();
        let mut width: I = cast(half_power).unwrap();
        while width > cast(0).unwrap() {
            let mid: I = low | width;
            let index: usize = cast(mid).unwrap();
            if (index < data.len()) && (data[index] <= value) {
                low = mid;
            }
            width >>= 1u16;
        }
        let index: usize = cast(low).unwrap();
        if data[index] == value {
            return Some(low);
        }
        return None;
    }
}

impl<D, I, const N: usize> StaticPower<D, I, N> for StaticImplementation
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num_traits::NumCast
        + std::ops::BitOr<Output = I>
        + std::ops::ShrAssign<u16>
        + std::marker::Copy,
{
    #[inline]
    fn without_bound_check(data: &[D; N], value: D) -> Option<I> {
        use self::utility::{log2, previous_power_of_two};
        use num_traits::cast::cast;

        //// const HALF_POWER: usize = previous_power_of_two(N);
        //// const P: u32 = log2(HALF_POWER).unwrap_or(cast(0).unwrap());
        let half_power: usize = previous_power_of_two(N);
        let p: u32 = log2(half_power).unwrap_or(cast(0).unwrap());
        let mut low: I = cast(0).unwrap();
        let mut width: I = cast(half_power).unwrap();
        for _i in 0..=p {
            let mid: I = low | width;
            let index: usize = cast(mid).unwrap();
            if data[index] <= value {
                low = mid;
            }
            width >>= 1u16;
        }
        let index: usize = cast(low).unwrap();
        if data[index] == value {
            return Some(low);
        }
        return None;
    }

    #[inline]
    fn with_bound_check(data: &[D; N], value: D) -> Option<I> {
        use self::utility::{log2, previous_power_of_two};
        use num_traits::cast::cast;

        //// const HALF_POWER: usize = previous_power_of_two(N);
        //// const P: u32 = log2(HALF_POWER).unwrap_or(cast(0).unwrap());
        let half_power: usize = previous_power_of_two(N);
        let p: u32 = log2(half_power).unwrap_or(cast(0).unwrap());
        let mut low: I = cast(0).unwrap();
        let mut width: I = cast(half_power).unwrap();
        for _i in 0..=p {
            let mid: I = low | width;
            let index: usize = cast(mid).unwrap();
            if (index < data.len()) && (data[index] <= value) {
                low = mid;
            }
            width >>= 1u16;
        }
        let index: usize = cast(low).unwrap();
        if data[index] == value {
            return Some(low);
        }
        return None;
    }
}

impl<D, I> DynamicBinarySearch<D, I> for DynamicImplementation
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num_traits::NumCast
        + std::ops::BitOr<Output = I>
        + std::ops::ShrAssign<u16>
        + std::marker::Copy,
{
    #[inline]
    fn r#impl(&self, data: &[D], value: D) -> Option<I> {
        if data.len() > 0 {
            if self::utility::is_power_of_two(data.len()) {
                return Self::without_bound_check(data, value);
            } else {
                return Self::with_bound_check(data, value);
            }
        }
        return None;
    }
}

impl<D, I, const N: usize> StaticBinarySearch<D, I, N> for StaticImplementation
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num_traits::NumCast
        + std::ops::BitOr<Output = I>
        + std::ops::ShrAssign<u16>
        + std::marker::Copy,
{
    #[inline]
    fn r#impl(&self, data: &[D; N], value: D) -> Option<I> {
        if N > 0 {
            if self::utility::is_power_of_two(N) {
                return Self::without_bound_check(data, value);
            } else {
                return Self::with_bound_check(data, value);
            }
        }
        return None;
    }
}
