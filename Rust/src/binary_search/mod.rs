pub trait DataTypes<D>
where
    D: std::cmp::PartialOrd,
{
}

pub trait IndexTypes<I>
where
    I: num::Integer,
{
}

pub trait SignedIndexType<I>: IndexTypes<I>
where
    I: num_traits::Signed + num::Integer,
{
}

pub trait UnsignedIndexType<I>: IndexTypes<I>
where
    I: num_traits::Unsigned + num::Integer,
{
}

pub trait DynamicBinarySearch<D, I>: DataTypes<D> + IndexTypes<I>
where
    D: std::cmp::PartialOrd,
    I: num::Integer,
{
    fn r#impl(&self, data: &[D], value: D) -> Option<I>;
}

pub trait StaticBinarySearch<D, I, const N: usize>: DataTypes<D> + IndexTypes<I>
where
    D: std::cmp::PartialOrd,
    I: num::Integer,
{
    fn r#impl(&self, data: &[D; N], value: D) -> Option<I>;
}
