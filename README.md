# ringvec

[![License](https://img.shields.io/badge/license-MIT-green)](https://github.com/stjepangolemac/ringvec/blob/main/LICENSE)

A simple ring buffer implementation based on a vector. No dependencies except criterion
for benchmarking.

## Usage

```rust
use ringvec::RingVec;

fn main() {
    let mut v = RingVec::new(3);

    assert!(v.is_empty());
    assert!(!v.is_full());

    v.push(1);
    v.push(2);
    v.push(3);

    v.push_force(4);
    v.push_force(5);

    assert!(!v.is_empty());
    assert!(v.is_full());

    assert_eq!(v.peek_oldest(), Some(&3));
    assert_eq!(v.peek_newest(), Some(&5));

    assert_eq!(v.pop(), Some(3));

    assert!(!v.is_empty());
    assert!(!v.is_full());

    assert_eq!(v.pop(), Some(4));
    assert_eq!(v.pop(), Some(5));

    assert!(v.is_empty());
    assert!(!v.is_full());
}
```

## Performance

It's optimized for peeking, but not slow when pushing too. I might've messed
something up when it comes to benchmarking peeking.

- push 7.4ns
- peek oldest 0ns
- peek newest 0ns
