pub mod utility {
    use num::PrimInt;

    //
    // Some helper functions
    //

    fn recursive_eytzinger_layout<D: std::marker::Copy>(
        eytzinger: &mut [D],
        monotonic: &[D],
        mut i: usize,
        k: usize,
    ) -> usize {
        if k <= eytzinger.len() {
            i = recursive_eytzinger_layout(eytzinger, monotonic, i, k << 1usize);
            eytzinger[k - 1] = monotonic[i];
            i += 1usize;
            i = recursive_eytzinger_layout(eytzinger, monotonic, i, (k << 1usize) + 1usize);
        }

        return i;
    }

    #[inline]
    pub fn eytzinger_layout<D: std::marker::Copy>(eytzinger: &mut [D], monotonic: &[D]) -> bool {
        if eytzinger.len() != monotonic.len() {
            return false;
        }

        let length: usize = recursive_eytzinger_layout(eytzinger, monotonic, 0, 1);

        return length == monotonic.len();
    }

    #[inline]
    pub fn ffs<I: PrimInt>(value: I) -> u16 {
        use num_traits::cast::cast;
        return if value == cast(0).unwrap() {
            016
        } else {
            1u16 + value.trailing_zeros() as u16
        };
    }
}

use crate::binary_search::{DynamicBinarySearch, StaticBinarySearch};

trait Eytzinger<D, I>
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num::PrimInt
        + std::ops::Not
        + std::ops::ShrAssign<u16>,
{
    fn eytzinger_layout_search(data: &[D], value: &D) -> Option<I>;
}

pub struct ImplementationWithoutHints;
pub struct ImplementationBranchless;
pub struct ImplementationPrefetching;

impl<D, I> Eytzinger<D, I> for ImplementationWithoutHints
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num::PrimInt
        + std::ops::Not
        + std::ops::ShlAssign<u16>
        + std::ops::ShrAssign<u16>
        + std::ops::BitOrAssign,
{
    #[inline]
    fn eytzinger_layout_search(data: &[D], value: &D) -> Option<I> {
        use num_traits::cast::cast;

        let n: I = cast(data.len()).unwrap(); //< This fails, if data length don't fit into type I
        let mut index: I = cast(1).unwrap();
        while index <= n {
            if data[cast::<I, usize>(index).unwrap() - 1] < *value {
                index <<= 1;
                index |= cast(1).unwrap();
            } else {
                index <<= 1;
            }
        }
        index >>= utility::ffs(!index);
        return if (index == cast(0).unwrap())
            || (data[cast::<I, usize>(index).unwrap() - 1] != *value)
        {
            None
        } else {
            Some(index - cast(1).unwrap())
        };
    }
}

impl<D, I> Eytzinger<D, I> for ImplementationBranchless
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num::PrimInt
        + std::ops::Not
        + std::ops::ShrAssign<u16>
        + std::convert::From<bool>,
{
    #[inline]
    fn eytzinger_layout_search(data: &[D], value: &D) -> Option<I> {
        use num_traits::cast::cast;

        let n: I = cast(data.len()).unwrap(); //< This fails, if data length don't fit into type I
        let mut index: I = cast(1).unwrap();
        while index <= n {
            index = (index << 1) | (data[cast::<I, usize>(index).unwrap() - 1] < *value).into();
        }
        index >>= utility::ffs(!index);
        return if (index == cast(0).unwrap())
            || (data[cast::<I, usize>(index).unwrap() - 1] != *value)
        {
            None
        } else {
            Some(index - cast(1).unwrap())
        };
    }
}

impl<D, I> Eytzinger<D, I> for ImplementationPrefetching
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num::PrimInt
        + std::ops::Not
        + std::ops::ShrAssign<u16>
        + std::convert::From<bool>,
{
    #[inline]
    fn eytzinger_layout_search(data: &[D], value: &D) -> Option<I> {
        use num_traits::cast::cast;
        use std::arch::x86_64::{_mm_prefetch, _MM_HINT_T2};
        use std::ptr;

        let n: I = cast(data.len()).unwrap(); //< This fails, if data length don't fit into type I
        let mut index: I = cast(1).unwrap();
        unsafe { _mm_prefetch(ptr::addr_of!(data[0]) as *const i8, _MM_HINT_T2) };
        while index <= n {
            unsafe {
                _mm_prefetch(
                    ptr::addr_of!(data[cast::<I, usize>(index << 1).unwrap()]) as *const i8,
                    _MM_HINT_T2,
                )
            };
            index = (index << 1) | (data[cast::<I, usize>(index).unwrap() - 1] < *value).into()
        }
        index >>= utility::ffs(!index);
        return if (index == cast(0).unwrap())
            || (data[cast::<I, usize>(index).unwrap() - 1] != *value)
        {
            None
        } else {
            Some(index - cast(1).unwrap())
        };
    }
}

impl<D, I> DynamicBinarySearch<D, I> for ImplementationWithoutHints
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num::PrimInt
        + std::ops::Not
        + std::ops::ShlAssign<u16>
        + std::ops::ShrAssign<u16>
        + std::ops::BitOrAssign,
{
    #[inline]
    fn r#impl(&self, data: &[D], value: &D) -> Option<I> {
        return if data.len() > 0 {
            Self::eytzinger_layout_search(data, value)
        } else {
            None
        };
    }
}

impl<D, I, const N: usize> StaticBinarySearch<D, I, N> for ImplementationWithoutHints
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num::PrimInt
        + std::ops::Not
        + std::ops::ShlAssign<u16>
        + std::ops::ShrAssign<u16>
        + std::ops::BitOrAssign,
{
    #[inline]
    fn r#impl(&self, data: &[D; N], value: &D) -> Option<I> {
        return if N > 0 {
            Self::eytzinger_layout_search(data, value)
        } else {
            None
        };
    }
}

impl<D, I> DynamicBinarySearch<D, I> for ImplementationBranchless
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num::PrimInt
        + std::ops::Not
        + std::ops::ShrAssign<u16>
        + std::convert::From<bool>,
{
    #[inline]
    fn r#impl(&self, data: &[D], value: &D) -> Option<I> {
        return if data.len() > 0 {
            Self::eytzinger_layout_search(data, value)
        } else {
            None
        };
    }
}

impl<D, I, const N: usize> StaticBinarySearch<D, I, N> for ImplementationBranchless
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num::PrimInt
        + std::ops::Not
        + std::ops::ShrAssign<u16>
        + std::convert::From<bool>,
{
    #[inline]
    fn r#impl(&self, data: &[D; N], value: &D) -> Option<I> {
        return if N > 0 {
            Self::eytzinger_layout_search(data, value)
        } else {
            None
        };
    }
}

impl<D, I> DynamicBinarySearch<D, I> for ImplementationPrefetching
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num::PrimInt
        + std::ops::Not
        + std::ops::ShrAssign<u16>
        + std::convert::From<bool>,
{
    #[inline]
    fn r#impl(&self, data: &[D], value: &D) -> Option<I> {
        return if data.len() > 0 {
            Self::eytzinger_layout_search(data, value)
        } else {
            None
        };
    }
}

impl<D, I, const N: usize> StaticBinarySearch<D, I, N> for ImplementationPrefetching
where
    D: std::cmp::PartialOrd,
    I: num_traits::Unsigned
        + num::Integer
        + num::PrimInt
        + std::ops::Not
        + std::ops::ShrAssign<u16>
        + std::convert::From<bool>,
{
    #[inline]
    fn r#impl(&self, data: &[D; N], value: &D) -> Option<I> {
        return if N > 0 {
            Self::eytzinger_layout_search(data, value)
        } else {
            None
        };
    }
}
