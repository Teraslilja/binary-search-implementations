pub trait DynamicBinarySearch<D, I>
where
    D: std::cmp::PartialOrd,
    I: num::Integer,
{
    fn r#impl(&self, data: &[D], value: &D) -> Option<I>;
}

pub trait StaticBinarySearch<D, I, const N: usize>
where
    D: std::cmp::PartialOrd,
    I: num::Integer,
{
    fn r#impl(&self, data: &[D; N], value: &D) -> Option<I>;
}

pub mod alternative;
pub mod power;
pub mod range;
pub mod traditional;
