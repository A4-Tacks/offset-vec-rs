use core::{iter::{repeat_n, repeat_with}, ops::RangeBounds};
use alloc::string::{Drain, String};
use super::*;

impl VecLike for String {
    type Elem = char;
    type Slice = str;
    type Collection = String;
    type Drain<'a> = Drain<'a>;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn as_slice(&self) -> &Self::Slice {
        self
    }

    fn as_mut_slice(&mut self) -> &mut Self::Slice {
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
    fn push(&mut self, ch: Self::Elem) {
        self.push(ch);
    }

    #[track_caller]
    fn remove(&mut self, idx: usize) -> Self::Elem {
        self.remove(idx)
    }

    #[track_caller]
    fn insert(&mut self, idx: usize, ch: Self::Elem) {
        self.insert(idx, ch);
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
    fn shrink_to(&mut self, min_capacity: usize) {
        self.shrink_to(min_capacity)
    }

    #[track_caller]
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit()
    }

    fn truncate(&mut self, new_len: usize) {
        self.truncate(new_len);
    }

    #[track_caller]
    fn split_off(&mut self, at: usize) -> Self::Collection {
        self.split_off(at)
    }

    #[track_caller]
    fn resize(&mut self, new_len: usize, value: Self::Elem)
    where Self::Elem: Clone,
    {
        let len = self.len();
        if new_len > len {
            self.extend(repeat_n(value, new_len-len));
        } else {
            self.truncate(new_len);
        }
    }

    #[track_caller]
    fn resize_with<F>(&mut self, new_len: usize, f: F)
    where F: FnMut() -> Self::Elem,
    {
        let len = self.len();
        if new_len > len {
            self.extend(repeat_with(f).take(new_len-len));
        } else {
            self.truncate(new_len);
        }
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
        self.push_str(other)
    }
}
impl VecLikeAbstract for String {
    fn retain<F>(&mut self, f: F)
    where F: FnMut(Self::Elem) -> bool,
    {
        self.retain(f);
    }
}
