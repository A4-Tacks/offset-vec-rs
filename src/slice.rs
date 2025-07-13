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

    fn transform_index(&self, index: usize) -> usize;
}

impl<T> Slice for [T] {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    fn transform_index(&self, index: usize) -> usize {
        index
    }
}
impl Slice for str {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    fn transform_index(&self, index: usize) -> usize {
        crate::util::transform_char_index(self, index)
    }
}
