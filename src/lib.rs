//! # Structural operations for tuples
//!
//! This crate implements splitting and joining tuples.
//!
//! The traits are implemented for tuples from zero len (ie, `()` unit) to 16.
//! (More with with the `tuple_24` and `tuple_32` features enabled.)
//!
//! They are implemented for both tuples by value and reference, which either
//! consume or borrow their inputs respectively.
//!
//! An example of [`TupleJoin`] by value:
//! ```rust
//! use tuplestructops::TupleJoin;
//!
//! let out = (1,'a',"b").join((1., 2.));
//! println!("out {out:?}");
//! ```
//!
//! [`TupleSplit`] does the converse. It relies on pattern matching to determine
//! the split.
//! ```rust
//! use tuplestructops::TupleSplit;
//!
//! let out: (_, (_,_,_)) = (1,2,3,4,5).split();
//! println!("out {out:?}");
//! ```
use seq_macro::seq;

#[cfg(test)]
mod test;

mod macro_impl;

/// Implement `join` for tuples.
///
/// `Self` is the left side of the join, and right is the `RHS` type parameter.
pub trait TupleJoin<RHS>: seal::Sealed {
    /// Joined output type. See [`TupleJoinOutput`] as an easy way to reference
    /// this in other type contexts.
    type Output;

    /// Join two tuples. `self` is the left (prefix) and `other` is the right
    /// (suffix).
    /// ```rust
    /// # use tuplestructops::TupleJoin;
    /// let out: (_, _, _, _, _, _) = (1, 2, 3).join(('a', 'b', 'c'));
    /// ```
    /// Joining unit `()` tuples has no effect.
    /// ```rust
    /// # use tuplestructops::TupleJoin;
    /// assert_eq!(().join((1, 2, 3)), (1, 2, 3));
    /// ```
    fn join(self, other: RHS) -> Self::Output;
}

/// Resulting type of joining tuples `L` and `R`. This is useful for referencing
/// it in other type contexts. For example:
/// ```rust
/// # use std::path::PathBuf;
/// # use tuplestructops::{TupleJoin, TupleJoinOutput};
/// type DefaultBits = (String, PathBuf);
///
/// fn add_default<E>(extra: E) -> TupleJoinOutput<DefaultBits, E>
/// where DefaultBits: TupleJoin<E>
/// {
///     ("hello".to_string(), PathBuf::from("/world")).join(extra)
/// }
/// ```
pub type TupleJoinOutput<L, R> = <L as TupleJoin<R>>::Output;

/// Split a tuple into left and right portions.
pub trait TupleSplit<LHS, RHS>: seal::Sealed {
    /// The left (prefix) and right (suffix) portions are defined by the `LHS`
    /// and `RHS` trait type parameters. When invoking `split`, these will
    /// generally be inferred by the calling context. For example, to split the
    /// last three fields off a tuple, one can use:
    /// ```rust
    /// # use tuplestructops::TupleSplit;
    /// let sometuple = (1, 2, 3, 4, 5, 'a', 'b', 'c');
    /// let (left, (a, b, c)) = sometuple.split();
    /// ```
    /// Note that in this example `sometuple` can be any tuple type so long as
    /// it has at least three fields.
    fn split(self) -> (LHS, RHS);
}

mod seal {
    pub trait Sealed {}
}
