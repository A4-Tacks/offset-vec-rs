pub(crate) fn transform_char_index(s: &str, byte_index: usize) -> usize {
    debug_assert!(s.is_char_boundary(byte_index),
                 "{s:?} <- {byte_index} is not on char boundary");
    s.char_indices()
        .take_while(|&(i, _)| i < byte_index)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trst_transform_char_index() {
        let datas = [
            ("", 0, 0),
            ("a", 0, 0),
            ("a", 1, 1),
            ("你", 0, 0),
            ("你", 3, 1),
            ("你好", 3, 1),
            ("你好", 6, 2),
        ];

        for (s, bi, ci) in datas {
            assert_eq!(transform_char_index(s, bi), ci, "{s:?}, {bi}, {ci}");
        }
    }
}
