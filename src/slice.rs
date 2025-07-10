use core::ops::{IndexMut, Range, RangeFrom, RangeTo};

pub trait Slice
    : IndexMut<Range<usize>, Output = Self>
    + IndexMut<RangeTo<usize>, Output = Self>
    + IndexMut<RangeFrom<usize>, Output = Self>
{
    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Slice for [T] {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}
impl Slice for str {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}
