#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

#[cfg(doc)]
use alloc::{vec::Vec, string::String};

use core::{
    iter::once,
    ops::{Bound, Deref, DerefMut, Index, IndexMut, Range, RangeBounds},
    slice::SliceIndex,
};

mod util;
mod slice;
mod offset;
mod vec_like;

pub use slice::*;
pub use offset::*;
pub use vec_like::*;

mod externs {
    mod core_impls;
}

/// Packer for [`Vec`] and [`String`] etc
///
/// For all methods index add a `offset`
///
/// Create from [`Offset::offset`] or [`create`]
///
/// # Examples
///
/// ```
/// use offset_vec::Offset;
///
/// let mut vec = vec![0, 1, 2, 3, 4];
/// let mut vec1 = vec.offset_mut(2);
///
/// assert_eq!(vec1, [2, 3, 4]);
/// assert_eq!(vec1[1], 3);
///
/// vec1[1] += 2;
/// assert_eq!(vec, [0, 1, 2, 5, 4]);
/// ```
#[derive(Debug, Clone, Default)]
pub struct OffsetVec<V: VecLike> {
    vec: V,
    offset: usize,
}

impl<V: VecLike> Deref for OffsetVec<V> {
    type Target = V::Slice;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let offset = self.offset;
        let slice = self.vec.as_slice();
        &slice[offset..]
    }
}

impl<V: VecLike> DerefMut for OffsetVec<V> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let offset = self.offset;
        let slice_mut = self.vec.as_mut_slice();
        &mut slice_mut[offset..]
    }
}

impl<V, I> Index<I> for OffsetVec<V>
where V: VecLike,
      I: SliceIndex<V::Slice>,
      V::Slice: Index<I>,
{
    type Output = <V::Slice as Index<I>>::Output;

    #[track_caller]
    fn index(&self, index: I) -> &Self::Output {
        let slice = &**self;
        &slice[index]
    }
}

impl<V, I> IndexMut<I> for OffsetVec<V>
where V: VecLike,
      I: SliceIndex<V::Slice>,
      V::Slice: IndexMut<I>,
{
    #[track_caller]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let slice = &mut **self;
        &mut slice[index]
    }
}

impl<V: VecLike> OffsetVec<V> {
    /// Get original vector
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4];
    /// let mut vec1 = vec.offset_mut(2);
    ///
    /// assert_eq!(vec1, [2, 3, 4]);
    /// assert_eq!(vec1.origin_vec(), &&mut vec![0, 1, 2, 3, 4]);
    /// ```
    #[inline]
    pub fn origin_vec(&self) -> &V {
        &self.vec
    }

    /// Get mutable original vector
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4];
    /// let mut vec1 = vec.offset_mut(2);
    ///
    /// assert_eq!(vec1, [2, 3, 4]);
    /// assert_eq!(vec1.origin_vec_mut(), &mut &mut vec![0, 1, 2, 3, 4]);
    ///
    /// vec1.origin_vec_mut()[3] += 2;
    /// assert_eq!(vec1, [2, 5, 4]);
    /// ```
    #[inline]
    pub fn origin_vec_mut(&mut self) -> &mut V {
        &mut self.vec
    }

    /// Consume [`OffsetVec`] into packed original vector
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let vec = vec![0, 1, 2, 3, 4];
    /// let mut vec1 = vec.offset(2);
    ///
    /// assert_eq!(vec1, [2, 3, 4]);
    /// assert_eq!(vec1.into_origin_vec(), vec![0, 1, 2, 3, 4]);
    /// ```
    #[inline]
    pub fn into_origin_vec(self) -> V {
        self.vec
    }

    /// Extracts a slice containing the offset vector.
    ///
    /// Equivalent to &s[..].
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let vec = vec![0, 1, 2, 3, 4];
    /// let mut vec1 = vec.offset(2);
    ///
    /// assert_eq!(vec1.as_slice(), &[2, 3, 4]);
    /// assert_eq!(&vec1[..], &[2, 3, 4]);
    /// ```
    #[inline]
    pub fn as_slice(&self) -> &V::Slice {
        self
    }

    /// Extracts a mutable slice containing the offset vector.
    ///
    /// Equivalent to &mut s[..].
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let vec = vec![0, 1, 2, 3, 4];
    /// let mut vec1 = vec.offset(2);
    ///
    /// assert_eq!(vec1.as_mut_slice(), &mut [2, 3, 4]);
    /// assert_eq!(&mut vec1[..], &mut [2, 3, 4]);
    /// ```
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut V::Slice {
        self
    }

    pub fn iter<'a>(&'a self) -> <&'a V::Slice as IntoIterator>::IntoIter
    where &'a V::Slice: IntoIterator
    {
        self.as_slice().into_iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> <&'a mut V::Slice as IntoIterator>::IntoIter
    where &'a mut V::Slice: IntoIterator
    {
        self.as_mut_slice().into_iter()
    }

    /// `self.len() == 0`
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get offset vector length
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let vec = vec![0, 1, 2, 3, 4];
    /// let mut vec1 = vec.offset(2);
    ///
    /// assert_eq!(vec1.len(), 3);
    /// assert_eq!(vec1.origin_vec().len(), 5);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len() - self.offset
    }

    /// Get offset vector capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let mut vec = Vec::with_capacity(5);
    /// vec.extend([0, 1, 2, 3, 4]);
    /// let mut vec1 = vec.offset(2);
    ///
    /// assert_eq!(3, vec1.capacity());
    /// assert_eq!(5, vec1.origin_vec().capacity());
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.vec.capacity() - self.offset
    }

    pub fn reserve(&mut self, additional: usize) {
        self.vec.reserve(additional);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.vec.reserve_exact(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.vec.shrink_to(min_capacity);
    }

    /// Get offset
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let vec = vec![0, 1, 2, 3, 4];
    /// let vec1 = vec.offset(2);
    ///
    /// assert_eq!(vec1.origin_offset(), 2);
    ///
    /// let vec2 = vec1.offset(1);
    /// assert_eq!(vec2.origin_offset(), 3);
    /// ```
    pub fn origin_offset(&self) -> usize {
        self.offset
    }

    /// Push a value
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4];
    /// let mut vec1 = vec.offset_mut(2);
    ///
    /// assert_eq!(vec1, [2, 3, 4]);
    /// vec1.push(5);
    /// assert_eq!(vec1, [2, 3, 4, 5]);
    /// assert_eq!(vec, [0, 1, 2, 3, 4, 5]);
    /// ```
    pub fn push(&mut self, value: V::Elem) {
        self.vec.push(value);
    }

    /// Pop a value
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let mut vec = vec![0, 1, 2, 3];
    /// let mut vec1 = vec.offset_mut(2);
    ///
    /// assert_eq!(vec1, [2, 3]);
    /// assert_eq!(vec1.pop(), Some(3));
    /// assert_eq!(vec1, [2]);
    /// assert_eq!(vec1.pop(), Some(2));
    /// assert_eq!(vec1, []);
    /// assert_eq!(vec1.pop(), None);
    /// assert_eq!(vec1, []);
    ///
    /// assert_eq!(vec1.origin_vec(), &&mut [0, 1]);
    /// assert_eq!(vec, [0, 1]);
    /// ```
    pub fn pop(&mut self) -> Option<V::Elem> {
        if self.is_empty() {
            return None;
        }

        self.vec.pop()
    }

    /// Remove a value at index, shifting all elements after it to the left.
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4, 5];
    /// let mut vec1 = vec.offset_mut(2);
    ///
    /// assert_eq!(vec1, [2, 3, 4, 5]);
    /// assert_eq!(vec1.remove(1), 3);
    /// assert_eq!(vec1, [2, 4, 5]);
    ///
    /// assert_eq!(vec, [0, 1, 2, 4, 5]);
    /// ```
    #[track_caller]
    pub fn remove(&mut self, index: usize) -> V::Elem {
        let len = self.len();
        if index > len {
            index_out_of_range(index, self.offset, len)
        }
        self.vec.remove(index + self.offset)
    }

    /// Insert a value before index, shifting all elements after it to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let mut vec = vec![0, 1, 2, 4, 5];
    /// let mut vec1 = vec.offset_mut(2);
    ///
    /// assert_eq!(vec1, [2, 4, 5]);
    /// vec1.insert(1, 3);
    /// assert_eq!(vec1, [2, 3, 4, 5]);
    /// vec1.insert(4, 6);
    /// assert_eq!(vec1, [2, 3, 4, 5, 6]);
    ///
    /// assert_eq!(vec, [0, 1, 2, 3, 4, 5, 6]);
    /// ```
    #[track_caller]
    pub fn insert(&mut self, index: usize, elem: V::Elem) {
        let len = self.len();
        if index > len {
            index_out_of_range(index, self.offset, len)
        }
        self.vec.insert(index + self.offset, elem)
    }

    /// Truncate to length
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4, 5];
    /// let mut vec1 = vec.offset_mut(2);
    ///
    /// assert_eq!(vec1, [2, 3, 4, 5]);
    ///
    /// vec1.truncate(2);
    ///
    /// assert_eq!(vec1, [2, 3]);
    /// assert_eq!(vec, [0, 1, 2, 3]);
    /// ```
    pub fn truncate(&mut self, len: usize) {
        self.vec.truncate(len + self.offset);
    }

    /// Append and clear other collection
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let mut vec = vec![0, 1, 2, 3];
    /// let mut vec1 = vec.offset_mut(2);
    /// let mut other = vec![4, 5];
    ///
    /// assert_eq!(vec1, [2, 3]);
    /// vec1.append(&mut other);
    ///
    /// assert_eq!(other, []);
    /// assert_eq!(vec1, [2, 3, 4, 5]);
    /// assert_eq!(vec, [0, 1, 2, 3, 4, 5]);
    /// ```
    pub fn append(&mut self, other: &mut V::Collection) {
        self.vec.append(other);
    }

    /// Clear all elements (offset)
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4];
    /// let mut vec1 = vec.offset_mut(2);
    ///
    /// assert_eq!(vec1, [2, 3, 4]);
    ///
    /// vec1.clear();
    ///
    /// assert_eq!(vec1, []);
    /// assert_eq!(vec, [0, 1]);
    /// ```
    pub fn clear(&mut self) {
        self.vec.truncate(self.offset);
    }

    #[track_caller]
    fn map_range<R: RangeBounds<usize>>(&self, range: R) -> Range<usize> {
        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n.checked_add(1).expect("start range overflow"),
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&n) => n.checked_add(1).expect("end range overflow"),
            Bound::Excluded(&n) => n,
            Bound::Unbounded => self.len(),
        };
        if start > end {
            #[cold]
            #[track_caller]
            #[inline(never)]
            fn fail(index: usize, end: usize) -> ! {
                panic!("range index starts at {index} but ends at {end}");
            }
            fail(start, end)
        }
        if end > self.len() {
            #[cold]
            #[track_caller]
            #[inline(never)]
            fn fail(index: usize, len: usize) -> ! {
                panic!("range end index {index} out of range for slice of length {len}");
            }
            fail(end, self.len())
        }
        let offset = self.offset;
        Range { start: start+offset, end: end+offset }
    }

    /// Drain range elements
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4, 5, 6];
    /// let mut vec1 = vec.offset_mut(2);
    ///
    /// assert_eq!(vec1, [2, 3, 4, 5, 6]);
    ///
    /// let drain = vec1.drain(2..4).collect::<Vec<_>>();
    ///
    /// assert_eq!(drain, [4, 5]);
    /// assert_eq!(vec1, [2, 3, 6]);
    /// assert_eq!(vec, [0, 1, 2, 3, 6]);
    /// ```
    #[track_caller]
    pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> V::Drain<'_> {
        self.vec.drain(self.map_range(range))
    }

    /// Splits the collection into two at the given index.
    ///
    /// # Examples
    ///
    /// ```
    /// use offset_vec::Offset;
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4, 5, 6];
    /// let mut vec1 = vec.offset_mut(2);
    ///
    /// assert_eq!(vec1, [2, 3, 4, 5, 6]);
    ///
    /// let other = vec1.split_off(3);
    ///
    /// assert_eq!(other, [5, 6]);
    /// assert_eq!(vec1, [2, 3, 4]);
    /// assert_eq!(vec, [0, 1, 2, 3, 4]);
    /// ```
    #[track_caller]
    #[must_use = "use `.truncate()` if you don't need the other half"]
    pub fn split_off(&mut self, at: usize) -> V::Collection {
        let len = self.len();
        if at > len {
            index_out_of_range(at, self.offset, len)
        }
        self.vec.split_off(at + self.offset)
    }

    pub fn resize(&mut self, new_len: usize, value: V::Elem)
    where V::Elem: Clone,
    {
        self.vec.resize(new_len+self.offset, value);
    }

    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where F: FnMut() -> V::Elem,
    {
        self.vec.resize_with(new_len+self.offset, f);
    }

    pub fn retain<F>(&mut self, mut f: F)
    where F: FnMut(V::ElemRef<'_>) -> bool,
    {
        let i = self.vec.as_slice().transform_index(self.offset);

        let mut i = 0..i;
        self.vec.retain(|elem| {
            i.next().is_some() || f(elem)
        });
    }
}

impl<V: VecLikeSolid> OffsetVec<V> {
    pub fn retain_mut<F: FnMut(&mut V::Elem) -> bool>(&mut self, mut f: F) {
        let mut i = 0..self.offset;
        self.vec.retain_mut(|elem| {
            i.next().is_some() || f(elem)
        });
    }

    #[track_caller]
    pub fn swap_remove(&mut self, index: usize) -> V::Elem {
        let len = self.len();
        if index > len {
            index_out_of_range(index, self.offset, len)
        }
        self.vec.swap_remove(index + self.offset)
    }

    pub fn pop_if<F>(&mut self, predicate: F) -> Option<V::Elem>
    where F: FnOnce(&mut V::Elem) -> bool,
    {
        if self.is_empty() {
            return None;
        }

        self.vec.pop_if(predicate)
    }
}

impl<V: VecLike<Slice = str>> OffsetVec<V> {
    #[inline]
    pub fn as_str(&self) -> &str {
        self
    }

    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        self
    }

    pub fn push_str<'a>(&mut self, s: &'a str)
    where V::Collection: Extend<&'a str>,
    {
        self.vec.as_mut_collection().extend(once(s));
    }
}

#[cold]
#[track_caller]
#[inline(never)]
fn index_out_of_range(index: usize, offset: usize, len: usize) -> ! {
    panic!("offset index ({index} -> {}) out of length (is {len} -> {})",
           index+offset,
           len+offset);
}
