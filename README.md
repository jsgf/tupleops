# Structural operations for Tuples

This crate implements three operations for tuples:
- join
- split
- index

These operations are implemented via traits implemented for tuples up to a limit
of 16 - the implementations are O(N^2), so it's worth bounding the tuple size to
keep compilation time under control.