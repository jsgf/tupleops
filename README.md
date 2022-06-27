# Structural operations for Tuples

This crate implements three operations for tuples:
- join
- split
- index

The implementations are O(N^2) in the number of tuple elements. By default
they're implemented for up to 16 elements, but the additional
- tuple_20
- tuple_24
- tuple_28
- tuple_32

features allow the traits to be implemnented for more elements.