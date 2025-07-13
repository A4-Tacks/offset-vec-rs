use unique_rc::{UniqArc, UniqRc};
use super::*;

impl<V: VecLike> VecLike for UniqRc<V> {
    type Elem = V::Elem;
    type ElemRef<'a> = V::ElemRef<'a> where Self: 'a;
    type Slice = V::Slice;
    type Collection = V::Collection;
    type Drain<'a> = V::Drain<'a> where Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }

    fn as_slice(&self) -> &Self::Slice {
        (**self).as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut Self::Slice {
        (**self).as_mut_slice()
    }

    fn as_mut_collection(&mut self) -> &mut Self::Collection {
        (**self).as_mut_collection()
    }

    fn capacity(&self) -> usize {
        (**self).capacity()
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        (**self).pop()
    }

    #[track_caller]
    fn push(&mut self, value: Self::Elem) {
        (**self).push(value);
    }

    #[track_caller]
    fn remove(&mut self, index: usize) -> Self::Elem {
        (**self).remove(index)
    }

    #[track_caller]
    fn insert(&mut self, index: usize, element: Self::Elem) {
        (**self).insert(index, element);
    }

    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        (**self).reserve(additional)
    }

    #[track_caller]
    fn reserve_exact(&mut self, additional: usize) {
        (**self).reserve_exact(additional)
    }

    #[track_caller]
    fn shrink_to(&mut self, min_capacity: usize) {
        (**self).shrink_to(min_capacity)
    }

    #[track_caller]
    fn shrink_to_fit(&mut self) {
        (**self).shrink_to_fit()
    }

    fn truncate(&mut self, len: usize) {
        (**self).truncate(len);
    }

    #[track_caller]
    fn split_off(&mut self, at: usize) -> Self::Collection {
        (**self).split_off(at)
    }

    #[track_caller]
    fn resize(&mut self, new_len: usize, value: Self::Elem)
    where Self::Elem: Clone,
    {
        (**self).resize(new_len, value);
    }

    #[track_caller]
    fn resize_with<F>(&mut self, new_len: usize, f: F)
    where F: FnMut() -> Self::Elem,
    {
        (**self).resize_with(new_len, f);
    }

    fn drain<R>(&mut self, range: R) -> Self::Drain<'_>
    where R: RangeBounds<usize>,
    {
        (**self).drain(range)
    }

    fn clear(&mut self) {
        (**self).clear();
    }

    #[track_caller]
    fn append(&mut self, other: &mut Self::Collection) {
        (**self).append(other);
    }

    fn retain<F>(&mut self, f: F)
    where F: FnMut(V::ElemRef<'_>) -> bool,
    {
        (**self).retain(f);
    }
}
impl<V: VecLikeSolid> VecLikeSolid for UniqRc<V> {
    fn swap_remove(&mut self, index: usize) -> Self::Elem {
        (**self).swap_remove(index)
    }

    fn retain_mut<F>(&mut self, f: F)
    where F: FnMut(&mut Self::Elem) -> bool,
    {
        (**self).retain_mut(f);
    }

    fn pop_if<F>(&mut self, predicate: F) -> Option<Self::Elem>
    where F: FnOnce(&mut Self::Elem) -> bool,
    {
        (**self).pop_if(predicate)
    }
}

impl<V: VecLike> VecLike for UniqArc<V> {
    type Elem = V::Elem;
    type ElemRef<'a> = V::ElemRef<'a> where Self: 'a;
    type Slice = V::Slice;
    type Collection = V::Collection;
    type Drain<'a> = V::Drain<'a> where Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }

    fn as_slice(&self) -> &Self::Slice {
        (**self).as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut Self::Slice {
        (**self).as_mut_slice()
    }

    fn as_mut_collection(&mut self) -> &mut Self::Collection {
        (**self).as_mut_collection()
    }

    fn capacity(&self) -> usize {
        (**self).capacity()
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        (**self).pop()
    }

    #[track_caller]
    fn push(&mut self, value: Self::Elem) {
        (**self).push(value);
    }

    #[track_caller]
    fn remove(&mut self, index: usize) -> Self::Elem {
        (**self).remove(index)
    }

    #[track_caller]
    fn insert(&mut self, index: usize, element: Self::Elem) {
        (**self).insert(index, element);
    }

    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        (**self).reserve(additional)
    }

    #[track_caller]
    fn reserve_exact(&mut self, additional: usize) {
        (**self).reserve_exact(additional)
    }

    #[track_caller]
    fn shrink_to(&mut self, min_capacity: usize) {
        (**self).shrink_to(min_capacity)
    }

    #[track_caller]
    fn shrink_to_fit(&mut self) {
        (**self).shrink_to_fit()
    }

    fn truncate(&mut self, len: usize) {
        (**self).truncate(len);
    }

    #[track_caller]
    fn split_off(&mut self, at: usize) -> Self::Collection {
        (**self).split_off(at)
    }

    #[track_caller]
    fn resize(&mut self, new_len: usize, value: Self::Elem)
    where Self::Elem: Clone,
    {
        (**self).resize(new_len, value);
    }

    #[track_caller]
    fn resize_with<F>(&mut self, new_len: usize, f: F)
    where F: FnMut() -> Self::Elem,
    {
        (**self).resize_with(new_len, f);
    }

    fn drain<R>(&mut self, range: R) -> Self::Drain<'_>
    where R: RangeBounds<usize>,
    {
        (**self).drain(range)
    }

    fn clear(&mut self) {
        (**self).clear();
    }

    #[track_caller]
    fn append(&mut self, other: &mut Self::Collection) {
        (**self).append(other);
    }

    fn retain<F>(&mut self, f: F)
    where F: FnMut(V::ElemRef<'_>) -> bool,
    {
        (**self).retain(f);
    }
}
impl<V: VecLikeSolid> VecLikeSolid for UniqArc<V> {
    fn swap_remove(&mut self, index: usize) -> Self::Elem {
        (**self).swap_remove(index)
    }

    fn retain_mut<F>(&mut self, f: F)
    where F: FnMut(&mut Self::Elem) -> bool,
    {
        (**self).retain_mut(f);
    }

    fn pop_if<F>(&mut self, predicate: F) -> Option<Self::Elem>
    where F: FnOnce(&mut Self::Elem) -> bool,
    {
        (**self).pop_if(predicate)
    }
}
