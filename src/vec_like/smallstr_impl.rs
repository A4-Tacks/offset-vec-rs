use core::{iter::{repeat_n, repeat_with}, ops::{Bound, Range, RangeBounds}};
use smallstr::{SmallString, DrainRange};
use smallvec::Array;
use super::*;

impl<A: Array<Item = u8>> VecLike for SmallString<A> {
    type Elem = char;
    type ElemRef<'a> = char where Self: 'a;
    type Slice = str;
    type Collection = Self;
    type Drain<'a> = DrainRange<'a, A> where A: 'a;

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
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit()
    }

    fn truncate(&mut self, new_len: usize) {
        self.truncate(new_len);
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

    // FIXME: Replace to unimplemented smallstr::drain_range
    #[track_caller]
    fn drain<R>(&mut self, range: R) -> Self::Drain<'_>
    where R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n.checked_add(1).unwrap(),
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&n) => n.checked_add(1).unwrap(),
            Bound::Excluded(&n) => n,
            Bound::Unbounded => self.len(),
        };
        let range = Range { start, end };
        let _ = self.as_str()[range.clone()];
        self.drain_range(range)
    }

    fn clear(&mut self) {
        self.clear();
    }

    #[track_caller]
    fn append(&mut self, other: &mut Self::Collection) {
        self.push_str(other)
    }

    fn retain<F>(&mut self, f: F)
    where F: FnMut(Self::Elem) -> bool,
    {
        self.retain(f);
    }
}


#[cfg(test)]
mod tests {
    use alloc::string::String;
    use smallstr::SmallString;
    use crate::VecLike;

    #[test]
    fn test() {
        let mut s: SmallString<[u8; 0]> = SmallString::from_str("foobar");
        assert_eq!(s, "foobar");
        let s1: String = VecLike::drain(&mut s, 1..=3).collect();
        assert_eq!(s1, "oob");
        assert_eq!(s, "far");
    }

    #[test]
    fn test_rev() {
        let mut s: SmallString<[u8; 0]> = SmallString::from_str("foobar");
        assert_eq!(s, "foobar");
        let s1: String = VecLike::drain(&mut s, 1..=3).rev().collect();
        assert_eq!(s1, "boo");
        assert_eq!(s, "far");
    }

    #[test]
    fn test_empty() {
        let mut s: SmallString<[u8; 0]> = SmallString::new();
        assert_eq!(s, "");
        let s1: String = VecLike::drain(&mut s, 0..0).collect();
        assert_eq!(s1, "");
        assert_eq!(s, "");
    }

    #[test]
    fn test_multi_bytes() {
        let mut s: SmallString<[u8; 0]> = SmallString::from_str("从前有座山");
        assert_eq!(s, "从前有座山");
        let s1: String = VecLike::drain(&mut s, 6..12).collect();
        assert_eq!(s1, "有座");
        assert_eq!(s, "从前山");
    }

    #[test]
    fn test_multi_bytes_rev() {
        let mut s: SmallString<[u8; 0]> = SmallString::from_str("从前有座山");
        assert_eq!(s, "从前有座山");
        let s1: String = VecLike::drain(&mut s, 6..12).rev().collect();
        assert_eq!(s1, "座有");
        assert_eq!(s, "从前山");
    }

    #[test]
    fn some_consume() {
        let mut s: SmallString<[u8; 0]> = SmallString::from_str("foobar");
        assert_eq!(s, "foobar");
        let ch = VecLike::drain(&mut s, 1..=3).next();
        assert_eq!(ch, Some('o'));
        assert_eq!(s, "far");
    }

    #[test]
    fn no_consume() {
        let mut s: SmallString<[u8; 0]> = SmallString::from_str("foobar");
        assert_eq!(s, "foobar");
        VecLike::drain(&mut s, 1..=3);
        assert_eq!(s, "far");
    }

    #[test]
    fn size_hint() {
        let mut s: SmallString<[u8; 0]> = SmallString::from_str("foobar");
        assert_eq!(s, "foobar");
        let size_hint = VecLike::drain(&mut s, 1..=3).size_hint();
        assert_eq!(size_hint, (1, Some(3)));
    }
}
