Packer for `Vec` and `String` etc

For all methods index add a `offset`

# Examples

```rust
use offset_vec::Offset;

let mut vec = vec![0, 1, 2, 3, 4];
let mut vec1 = vec.offset_mut(2);

assert_eq!(vec1, [2, 3, 4]);
assert_eq!(vec1[1], 3);

vec1[1] += 2;
assert_eq!(vec, [0, 1, 2, 5, 4]);
```

If there is no need for reallocation, consider [tailvec](https://github.com/A4-Tacks/tailvec-rs)
