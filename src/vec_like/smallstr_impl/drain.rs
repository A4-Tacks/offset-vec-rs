use core::{iter::FusedIterator, str};

use smallvec::Array;

pub struct Drain<'a, A: Array<Item = u8>> {
    pub(super) drain: smallvec::Drain<'a, A>,
}
impl<A: Array<Item = u8>> Iterator for Drain<'_, A> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 4];

        buf[0] = self.drain.next()?;
        let utf8_len = 1.max(buf[0].leading_ones() as usize);

        for i in 1..utf8_len {
            buf[i] = self.drain.next().unwrap();
        }

        unsafe { str::from_utf8_unchecked(&buf[..utf8_len]) }.chars().next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.drain.len();
        (len.div_ceil(4), Some(len))
    }
}
impl<A: Array<Item = u8>> DoubleEndedIterator for Drain<'_, A> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 4];
        let mut i = 3;

        buf[i] = self.drain.next_back()?;

        while buf[i].leading_ones() == 1 {
            i -= 1;
            buf[i] = self.drain.next_back().unwrap();
        }

        unsafe { str::from_utf8_unchecked(&buf[i..]) }.chars().next()
    }
}

impl<A: Array<Item = u8>> FusedIterator for Drain<'_, A> { }

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
