use core::{borrow::{Borrow, BorrowMut}, hash::Hash};

use crate::{create, OffsetVec, VecLike};

impl<V: VecLike> PartialEq for OffsetVec<V> where V::Slice: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<V: VecLike> PartialEq<V> for OffsetVec<V> where V::Slice: PartialEq {
    fn eq(&self, other: &V) -> bool {
        **self == *other.as_slice()
    }
}

impl<V: VecLike> Eq for OffsetVec<V> where V::Slice: Eq { }

impl<V: VecLike> PartialOrd for OffsetVec<V> where V::Slice: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<V: VecLike> Ord for OffsetVec<V> where V::Slice: Ord {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: PartialEq, V: VecLike<Elem = T, Slice = [T]>> PartialEq<[T]> for OffsetVec<V> {
    fn eq(&self, other: &[V::Elem]) -> bool {
        **self == *other
    }
}
impl<T: PartialEq, V: VecLike<Elem = T, Slice = [T]>> PartialEq<&[T]> for OffsetVec<V> {
    fn eq(&self, other: &&[V::Elem]) -> bool {
        **self == **other
    }
}

impl<T: PartialEq, V: VecLike<Elem = T, Slice = [T]>, const N: usize> PartialEq<[T; N]> for OffsetVec<V> {
    fn eq(&self, other: &[V::Elem; N]) -> bool {
        **self == *other
    }
}
impl<T: PartialEq, V: VecLike<Elem = T, Slice = [T]>, const N: usize> PartialEq<&[T; N]> for OffsetVec<V> {
    fn eq(&self, other: &&[V::Elem; N]) -> bool {
        **self == **other
    }
}

impl<V: VecLike<Slice = str>> PartialEq<str> for OffsetVec<V> {
    fn eq(&self, other: &str) -> bool {
        **self == *other
    }
}
impl<V: VecLike<Slice = str>> PartialEq<&str> for OffsetVec<V> {
    fn eq(&self, other: &&str) -> bool {
        **self == **other
    }
}

impl<V: VecLike + IntoIterator> IntoIterator for OffsetVec<V> {
    type Item = V::Item;
    type IntoIter = V::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter = self.vec.into_iter();

        if self.offset != 0 {
            iter.nth(self.offset-1);
        }

        iter
    }
}

impl<T, V: VecLike<Slice = [T]>> Borrow<[T]> for OffsetVec<V> {
    fn borrow(&self) -> &V::Slice {
        self
    }
}
impl<T, V: VecLike<Slice = [T]>> BorrowMut<[T]> for OffsetVec<V> {
    fn borrow_mut(&mut self) -> &mut V::Slice {
        self
    }
}

impl<V: VecLike<Slice = str>> Borrow<str> for OffsetVec<V> {
    fn borrow(&self) -> &V::Slice {
        self
    }
}
impl<V: VecLike<Slice = str>> BorrowMut<str> for OffsetVec<V> {
    fn borrow_mut(&mut self) -> &mut V::Slice {
        self
    }
}

impl<V: VecLike> Hash for OffsetVec<V> where V::Slice: Hash {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<V: VecLike> AsRef<V::Slice> for OffsetVec<V> {
    fn as_ref(&self) -> &V::Slice {
        self
    }
}

impl<V: VecLike> AsMut<V::Slice> for OffsetVec<V> {
    fn as_mut(&mut self) -> &mut V::Slice {
        self
    }
}

impl<V: VecLike> From<V> for OffsetVec<V> {
    fn from(value: V) -> Self {
        create(value, 0)
    }
}

impl<T, V: VecLike> Extend<T> for OffsetVec<V> where V::Collection: Extend<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.vec.as_mut_collection().extend(iter);
    }
}
