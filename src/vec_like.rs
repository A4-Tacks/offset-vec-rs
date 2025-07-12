#![deny(unconditional_recursion)]

use core::ops::RangeBounds;

mod pointers_impl;
mod vec_impl;
mod string_impl;
#[cfg(feature = "smallvec")]
#[cfg_attr(docsrs, doc(cfg(feature = "smallvec")))]
mod smallvec_impl;
#[cfg(feature = "smallstr")]
#[cfg_attr(docsrs, doc(cfg(feature = "smallstr")))]
mod smallstr_impl;
#[cfg(feature = "rc-vec")]
#[cfg_attr(docsrs, doc(cfg(feature = "rc-vec")))]
mod rc_vec_impl;

use crate::Slice;

macro_rules! noop {
    ($($e:expr),*) => {{ $(let _ = $e;)* }};
}

pub trait VecLike {
    type Elem;
    type Slice: ?Sized + Slice;
    type Collection: FromIterator<Self::Elem>;
    type Drain<'a>: Iterator<Item = Self::Elem> where Self: 'a;

    fn as_slice(&self) -> &Self::Slice;

    fn as_mut_slice(&mut self) -> &mut Self::Slice;

    fn as_mut_collection(&mut self) -> &mut Self::Collection;

    fn capacity(&self) -> usize;

    fn reserve(&mut self, additional: usize) { noop!(additional) }

    fn reserve_exact(&mut self, additional: usize) { noop!(additional) }

    fn shrink_to_fit(&mut self) { noop!() }

    fn shrink_to(&mut self, min_capacity: usize) { noop!(min_capacity) }

    fn truncate(&mut self, len: usize);

    fn insert(&mut self, index: usize, element: Self::Elem);

    fn remove(&mut self, index: usize) -> Self::Elem;

    fn push(&mut self, value: Self::Elem);

    fn pop(&mut self) -> Option<Self::Elem>;

    fn append(&mut self, other: &mut Self::Collection);

    fn drain<R>(&mut self, range: R) -> Self::Drain<'_>
    where R: RangeBounds<usize>,;

    fn clear(&mut self);

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    #[must_use = "use `.truncate()` if you don't need the other half"]
    fn split_off(&mut self, at: usize) -> Self::Collection {
        self.drain(at..).collect()
    }

    fn resize(&mut self, new_len: usize, value: Self::Elem)
    where Self::Elem: Clone,;

    fn resize_with<F>(&mut self, new_len: usize, f: F)
    where F: FnMut() -> Self::Elem,;
}

pub trait VecLikeSolid: VecLike {
    fn swap_remove(&mut self, index: usize) -> Self::Elem;

    fn retain<F>(&mut self, mut f: F)
    where F: FnMut(&Self::Elem) -> bool,
    {
        self.retain_mut(|elem| f(&*elem));
    }

    fn retain_mut<F>(&mut self, f: F)
    where F: FnMut(&mut Self::Elem) -> bool,;

    fn pop_if<F>(&mut self, predicate: F) -> Option<Self::Elem>
    where F: FnOnce(&mut Self::Elem) -> bool,;

    // FIXME: 该方法较为冷门, 暂时不实现

    //fn dedup_by_key<F, K>(&mut self, key: F)
    //where
    //    F: FnMut(&mut Self::Elem) -> K,
    //    K: PartialEq,;

    //fn dedup_by<F>(&mut self, same_bucket: F)
    //where
    //    F: FnMut(&mut Self::Elem, &mut Self::Elem) -> bool,;
}

pub trait VecLikeAbstract: VecLike {
    type Indices<'a>: Iterator<Item = (usize, Self::Elem)> where Self: 'a;

    fn retain<F>(&mut self, f: F)
    where F: FnMut(Self::Elem) -> bool,;

    fn elem_indices(&self) -> Self::Indices<'_>;
}
