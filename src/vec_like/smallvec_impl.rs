use smallvec::{Array, SmallVec};
use super::*;

impl<A: Array> VecLike for SmallVec<A> {
    type Elem = A::Item;
    type ElemRef<'a> = &'a A::Item where Self: 'a;
    type Slice = [A::Item];
    type Collection = SmallVec<A>;
    type Drain<'a> = smallvec::Drain<'a, A> where Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn as_slice(&self) -> &[Self::Elem] {
        self
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        self
    }

    fn as_mut_collection(&mut self) -> &mut Self::Collection {
        self
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        self.pop()
    }

    #[track_caller]
    fn push(&mut self, value: Self::Elem) {
        self.push(value);
    }

    #[track_caller]
    fn remove(&mut self, index: usize) -> Self::Elem {
        self.remove(index)
    }

    fn insert(&mut self, index: usize, element: Self::Elem) {
        self.insert(index, element);
    }

    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional)
    }

    #[track_caller]
    fn reserve_exact(&mut self, additional: usize) {
        self.reserve_exact(additional)
    }

    #[track_caller]
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit()
    }

    fn truncate(&mut self, len: usize) {
        self.truncate(len);
    }

    #[track_caller]
    fn resize(&mut self, new_len: usize, value: Self::Elem)
    where Self::Elem: Clone,
    {
        self.resize(new_len, value);
    }

    #[track_caller]
    fn resize_with<F>(&mut self, new_len: usize, f: F)
    where F: FnMut() -> Self::Elem,
    {
        self.resize_with(new_len, f);
    }

    fn drain<R>(&mut self, range: R) -> Self::Drain<'_>
    where R: RangeBounds<usize>,
    {
        self.drain(range)
    }

    fn clear(&mut self) {
        self.clear();
    }

    #[track_caller]
    fn append(&mut self, other: &mut Self::Collection) {
        self.append(other);
    }

    fn retain<F>(&mut self, mut f: F)
    where F: FnMut(&A::Item) -> bool,
    {
        self.retain(|elem| f(&*elem));
    }
}
impl<A: Array> VecLikeSolid for SmallVec<A> {
    fn swap_remove(&mut self, index: usize) -> Self::Elem {
        self.swap_remove(index)
    }

    fn retain_mut<F>(&mut self, f: F)
    where F: FnMut(&mut Self::Elem) -> bool,
    {
        self.retain_mut(f);
    }

    fn pop_if<F>(&mut self, predicate: F) -> Option<Self::Elem>
    where F: FnOnce(&mut Self::Elem) -> bool,
    {
        let last = self.last_mut()?;
        if predicate(last) { self.pop() } else { None }
    }
}
