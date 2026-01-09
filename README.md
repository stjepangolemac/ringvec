# ringvec

[![License](https://img.shields.io/badge/license-MIT-green)](https://github.com/stjepangolemac/ringvec/blob/main/LICENSE)

A simple ring buffer implementation based on a vector. No dependencies except criterion for benchmarking.

## Usage

```rust
use ringvec::RingVec;

let mut v = RingVec::new(3);
v.push(1);
v.push(2);
v.push(3);
v.push_force(4);

assert_eq!(v.peek_oldest(), Some(&2));
assert_eq!(v.peek_newest(), Some(&4));
assert_eq!(v.pop(), Some(2));
```

## Performance

Latest benches:

- push_force 4.35 ns
- peek_oldest 0.32 ns
- peek_newest 0.32 ns
