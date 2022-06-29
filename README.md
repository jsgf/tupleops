# Structural operations for Tuples

This crate implements three operations for tuples:
- join
- split
- index

For example, you can simply concatenate two tuples with:
```rust
use tuplestructops::TupleJoin;

let concat = (1, 'b', 3).join(('a', 5, 'c'));
```

This crate focuses purely on the overall structure of tuples, and is completely
agnostic to the types of their elements.

The implementations are O(N^2) in the number of tuple elements. By default
they're implemented for up to 16 elements, but the additional
- tuple_24
- tuple_32

features allow the traits to be implemented for more elements.

The `impl_docs` feature enables documentation of the trait implementations for
all the tuple types. It is disabled by default since it's very repetitive.
