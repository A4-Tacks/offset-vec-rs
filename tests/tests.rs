use offset_vec::*;
use std::{borrow::ToOwned, vec};

#[test]
fn it_works() {
    let mut vec = vec![1, 2, 3, 4];
    let vec1 = vec.offset_mut(1);
    assert_eq!(vec1[0], 2);
    assert_eq!(vec1[1], 3);
    assert_eq!(vec1[0..2], [2, 3]);
}

#[test]
fn str() {
    let mut vec = "foobar".to_owned();
    let mut vec1 = vec.offset_mut(1);
    assert_eq!(&vec1[0..3], "oob");
    let vec2 = vec1.offset_mut(1);
    assert_eq!(&vec2[0..3], "oba");
}

#[test]
fn pop() {
    let mut vec = vec![0, 1, 2, 3, 4, 5].offset(3);
    assert_eq!(vec, [3, 4, 5]);
    assert_eq!(vec.pop(), Some(5));
    assert_eq!(vec.pop(), Some(4));
    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec.pop(), None);
    assert_eq!(vec.pop(), None);
    assert_eq!(vec.origin_vec(), &[0, 1, 2]);
}

#[test]
fn into_iter() {
    let vec = vec![0, 1, 2, 3, 4, 5].offset(3);
    assert_eq!(vec, [3, 4, 5]);
    assert_eq!(vec.into_iter().collect::<Vec<_>>(), [3, 4, 5]);
}

#[test]
fn iter() {
    let vec = vec![0, 1, 2, 3, 4, 5].offset(3);
    assert_eq!(vec, [3, 4, 5]);
    assert_eq!(vec.iter().collect::<Vec<_>>(), [&3, &4, &5]);
}

#[test]
fn remove() {
    let mut vec = vec![0, 1, 2, 3, 4, 5].offset(3);
    assert_eq!(vec, [3, 4, 5]);
    assert_eq!(vec.remove(1), 4);
    assert_eq!(vec, [3, 5]);
}

#[test]
fn insert() {
    let mut vec = vec![0, 1, 2, 3, 4, 5].offset(3);
    assert_eq!(vec, [3, 4, 5]);
    vec.insert(1, 0);
    assert_eq!(vec, [3, 0, 4, 5]);
}

#[test]
fn retain() {
    let mut s = vec![0, 1, 2, 3].to_owned().offset(1);
    s.retain(|_| false);
    assert_eq!(s.origin_vec(), &[0]);
    assert_eq!(s, []);
}

#[test]
fn retain_val() {
    let mut s = "foo".to_owned().offset(1);
    s.retain(|_| false);
    assert_eq!(s.origin_vec(), "f");
    assert_eq!(s, "");
}

#[test]
fn retain_val2() {
    let mut s = "foo".to_owned().offset(2);
    s.retain(|_| false);
    assert_eq!(s.origin_vec(), "fo");
    assert_eq!(s, "");
}

#[test]
fn retain_val3() {
    let mut s = "foo".to_owned().offset(3);
    s.retain(|_| false);
    assert_eq!(s.origin_vec(), "foo");
    assert_eq!(s, "");
}

#[test]
fn retain_val_fat_zero() {
    let mut s = "测试中".to_owned().offset(0);
    s.retain(|_| false);
    assert_eq!(s.origin_vec(), "");
    assert_eq!(s, "");
}

#[test]
fn retain_val_fat() {
    let mut s = "测试中".to_owned().offset(3);
    s.retain(|_| false);
    assert_eq!(s.origin_vec(), "测");
    assert_eq!(s, "");
}

#[test]
fn retain_val_fat1() {
    let mut s = "测试中".to_owned().offset(6);
    s.retain(|_| false);
    assert_eq!(s.origin_vec(), "测试");
    assert_eq!(s, "");
}

#[test]
fn retain_val_fat2() {
    let mut s = "测试中".to_owned().offset(9);
    s.retain(|_| false);
    assert_eq!(s.origin_vec(), "测试中");
    assert_eq!(s, "");
}
