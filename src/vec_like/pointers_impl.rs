use alloc::{borrow::{Cow, ToOwned}, boxed::Box, rc::Rc, sync::Arc};
use super::*;

#[cfg(feature = "unique-rc")]
#[cfg_attr(docsrs, doc(cfg(feature = "unique-rc")))]
mod unique_rc_impl;

impl<V: VecLike> VecLike for &mut V {
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
impl<V: VecLikeSolid> VecLikeSolid for &mut V {
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

impl<V: VecLike> VecLike for Box<V> {
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
impl<V: VecLikeSolid> VecLikeSolid for Box<V> {
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

impl<V: VecLike + Clone> VecLike for Rc<V> {
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
        Self::make_mut(self).as_mut_slice()
    }

    fn as_mut_collection(&mut self) -> &mut Self::Collection {
        Self::make_mut(self).as_mut_collection()
    }

    fn capacity(&self) -> usize {
        (**self).capacity()
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        Self::make_mut(self).pop()
    }

    #[track_caller]
    fn push(&mut self, value: Self::Elem) {
        Self::make_mut(self).push(value);
    }

    #[track_caller]
    fn remove(&mut self, index: usize) -> Self::Elem {
        Self::make_mut(self).remove(index)
    }

    #[track_caller]
    fn insert(&mut self, index: usize, element: Self::Elem) {
        Self::make_mut(self).insert(index, element);
    }

    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        Self::make_mut(self).reserve(additional)
    }

    #[track_caller]
    fn reserve_exact(&mut self, additional: usize) {
        Self::make_mut(self).reserve_exact(additional)
    }

    #[track_caller]
    fn shrink_to(&mut self, min_capacity: usize) {
        Self::make_mut(self).shrink_to(min_capacity)
    }

    #[track_caller]
    fn shrink_to_fit(&mut self) {
        Self::make_mut(self).shrink_to_fit()
    }

    fn truncate(&mut self, len: usize) {
        Self::make_mut(self).truncate(len);
    }

    #[track_caller]
    fn split_off(&mut self, at: usize) -> Self::Collection {
        Self::make_mut(self).split_off(at)
    }

    #[track_caller]
    fn resize(&mut self, new_len: usize, value: Self::Elem)
    where Self::Elem: Clone,
    {
        Self::make_mut(self).resize(new_len, value);
    }

    #[track_caller]
    fn resize_with<F>(&mut self, new_len: usize, f: F)
    where F: FnMut() -> Self::Elem,
    {
        Self::make_mut(self).resize_with(new_len, f);
    }

    fn drain<R>(&mut self, range: R) -> Self::Drain<'_>
    where R: RangeBounds<usize>,
    {
        Self::make_mut(self).drain(range)
    }

    fn clear(&mut self) {
        Self::make_mut(self).clear();
    }

    #[track_caller]
    fn append(&mut self, other: &mut Self::Collection) {
        Self::make_mut(self).append(other);
    }

    fn retain<F>(&mut self, f: F)
    where F: FnMut(V::ElemRef<'_>) -> bool,
    {
        Self::make_mut(self).retain(f);
    }
}
impl<V: VecLikeSolid + Clone> VecLikeSolid for Rc<V> {
    fn swap_remove(&mut self, index: usize) -> Self::Elem {
        Self::make_mut(self).swap_remove(index)
    }

    fn retain_mut<F>(&mut self, f: F)
    where F: FnMut(&mut Self::Elem) -> bool,
    {
        Self::make_mut(self).retain_mut(f);
    }

    fn pop_if<F>(&mut self, predicate: F) -> Option<Self::Elem>
    where F: FnOnce(&mut Self::Elem) -> bool,
    {
        Self::make_mut(self).pop_if(predicate)
    }
}

impl<V: VecLike + Clone> VecLike for Arc<V> {
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
        Self::make_mut(self).as_mut_slice()
    }

    fn as_mut_collection(&mut self) -> &mut Self::Collection {
        Self::make_mut(self).as_mut_collection()
    }

    fn capacity(&self) -> usize {
        (**self).capacity()
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        Self::make_mut(self).pop()
    }

    #[track_caller]
    fn push(&mut self, value: Self::Elem) {
        Self::make_mut(self).push(value);
    }

    #[track_caller]
    fn remove(&mut self, index: usize) -> Self::Elem {
        Self::make_mut(self).remove(index)
    }

    #[track_caller]
    fn insert(&mut self, index: usize, element: Self::Elem) {
        Self::make_mut(self).insert(index, element);
    }

    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        Self::make_mut(self).reserve(additional)
    }

    #[track_caller]
    fn reserve_exact(&mut self, additional: usize) {
        Self::make_mut(self).reserve_exact(additional)
    }

    #[track_caller]
    fn shrink_to(&mut self, min_capacity: usize) {
        Self::make_mut(self).shrink_to(min_capacity)
    }

    #[track_caller]
    fn shrink_to_fit(&mut self) {
        Self::make_mut(self).shrink_to_fit()
    }

    fn truncate(&mut self, len: usize) {
        Self::make_mut(self).truncate(len);
    }

    #[track_caller]
    fn split_off(&mut self, at: usize) -> Self::Collection {
        Self::make_mut(self).split_off(at)
    }

    #[track_caller]
    fn resize(&mut self, new_len: usize, value: Self::Elem)
    where Self::Elem: Clone,
    {
        Self::make_mut(self).resize(new_len, value);
    }

    #[track_caller]
    fn resize_with<F>(&mut self, new_len: usize, f: F)
    where F: FnMut() -> Self::Elem,
    {
        Self::make_mut(self).resize_with(new_len, f);
    }

    fn drain<R>(&mut self, range: R) -> Self::Drain<'_>
    where R: RangeBounds<usize>,
    {
        Self::make_mut(self).drain(range)
    }

    fn clear(&mut self) {
        Self::make_mut(self).clear();
    }

    #[track_caller]
    fn append(&mut self, other: &mut Self::Collection) {
        Self::make_mut(self).append(other);
    }

    fn retain<F>(&mut self, f: F)
    where F: FnMut(V::ElemRef<'_>) -> bool,
    {
        Self::make_mut(self).retain(f);
    }
}
impl<V: VecLikeSolid + Clone> VecLikeSolid for Arc<V> {
    fn swap_remove(&mut self, index: usize) -> Self::Elem {
        Self::make_mut(self).swap_remove(index)
    }

    fn retain_mut<F>(&mut self, f: F)
    where F: FnMut(&mut Self::Elem) -> bool,
    {
        Self::make_mut(self).retain_mut(f);
    }

    fn pop_if<F>(&mut self, predicate: F) -> Option<Self::Elem>
    where F: FnOnce(&mut Self::Elem) -> bool,
    {
        Self::make_mut(self).pop_if(predicate)
    }
}

impl<S> VecLike for Cow<'_, S>
where S: ToOwned + Slice,
      S::Owned: VecLike<Slice = S>,
{
    type Elem = <S::Owned as VecLike>::Elem;
    type ElemRef<'a> = <S::Owned as VecLike>::ElemRef<'a> where Self: 'a;
    type Slice = <S::Owned as VecLike>::Slice;
    type Collection = <S::Owned as VecLike>::Collection;
    type Drain<'a> = <S::Owned as VecLike>::Drain<'a> where Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }

    fn as_slice(&self) -> &Self::Slice {
        self
    }

    fn as_mut_slice(&mut self) -> &mut Self::Slice {
        self.to_mut().as_mut_slice()
    }

    fn as_mut_collection(&mut self) -> &mut Self::Collection {
        self.to_mut().as_mut_collection()
    }

    fn capacity(&self) -> usize {
        match self {
            Cow::Borrowed(slice) => slice.len(),
            Cow::Owned(owned) => owned.capacity(),
        }
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        self.to_mut().pop()
    }

    #[track_caller]
    fn push(&mut self, value: Self::Elem) {
        self.to_mut().push(value);
    }

    #[track_caller]
    fn remove(&mut self, index: usize) -> Self::Elem {
        self.to_mut().remove(index)
    }

    #[track_caller]
    fn insert(&mut self, index: usize, element: Self::Elem) {
        self.to_mut().insert(index, element);
    }

    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        self.to_mut().reserve(additional)
    }

    #[track_caller]
    fn reserve_exact(&mut self, additional: usize) {
        self.to_mut().reserve_exact(additional)
    }

    #[track_caller]
    fn shrink_to(&mut self, min_capacity: usize) {
        self.to_mut().shrink_to(min_capacity)
    }

    #[track_caller]
    fn shrink_to_fit(&mut self) {
        self.to_mut().shrink_to_fit()
    }

    fn truncate(&mut self, len: usize) {
        self.to_mut().truncate(len);
    }

    #[track_caller]
    fn split_off(&mut self, at: usize) -> Self::Collection {
        self.to_mut().split_off(at)
    }

    #[track_caller]
    fn resize(&mut self, new_len: usize, value: Self::Elem)
    where Self::Elem: Clone,
    {
        self.to_mut().resize(new_len, value);
    }

    #[track_caller]
    fn resize_with<F>(&mut self, new_len: usize, f: F)
    where F: FnMut() -> Self::Elem,
    {
        self.to_mut().resize_with(new_len, f);
    }

    fn drain<R>(&mut self, range: R) -> Self::Drain<'_>
    where R: RangeBounds<usize>,
    {
        self.to_mut().drain(range)
    }

    fn clear(&mut self) {
        self.to_mut().clear();
    }

    #[track_caller]
    fn append(&mut self, other: &mut Self::Collection) {
        self.to_mut().append(other);
    }

    fn retain<F>(&mut self, f: F)
    where F: FnMut(<S::Owned as VecLike>::ElemRef<'_>) -> bool,
    {
        self.to_mut().retain(f);
    }
}
impl<S> VecLikeSolid for Cow<'_, S>
where S: ToOwned + Slice,
      S::Owned: VecLikeSolid<Slice = S>,
{
    fn swap_remove(&mut self, index: usize) -> Self::Elem {
        self.to_mut().swap_remove(index)
    }

    fn retain_mut<F>(&mut self, f: F)
    where F: FnMut(&mut Self::Elem) -> bool,
    {
        self.to_mut().retain_mut(f);
    }

    fn pop_if<F>(&mut self, predicate: F) -> Option<Self::Elem>
    where F: FnOnce(&mut Self::Elem) -> bool,
    {
        self.to_mut().pop_if(predicate)
    }
}
