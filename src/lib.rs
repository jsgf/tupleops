//! # Structural operations for tuples
//!
//! This crate implements splitting, joining and indexing tuples.
//!
//! The traits are implemented for tuples from zero len (ie, `()` unit) to 16.
//!
//! ```rust
//! use tuplestructops::TupleJoin;
//!
//! let out = (1,'a',"b").join((1., 2.));
//! println!("out {out:?}");
//! ```
//!
//! [`TupleSplit`] does the converse. It relies on pattern matching for
//! determining the split.
//! ```rust
//! use tuplestructops::TupleSplit;
//!
//! let out: (_, (_,_,_)) = (1,2,3,4,5).split();
//! println!("out {out:?}");
//! ```
//!
//! [`TupleIdx`] allows a single tuple member to be referenced. `idx` gets a
//! reference to a field, and `extract` moves it out.
use seq_macro::seq;

/// Implement `join` for tuples.
///
/// `Self` is the left side of the join, and right is the `RHS` type parameter.
pub trait TupleJoin<RHS>: seal::Sealed {
    /// Joined output type.
    type Output;

    /// Join two tuples, consuming both. `self` is the left (prefix) and `other`
    /// is the right (suffix).
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

/// Resulting type of joining tuples `L` and `R`.
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
    /// let ((left), (a, b, c)) = sometuple.split();
    /// ```
    /// Note that in this example `sometuple` can be any tuple type so long as
    /// it has at least three fields.
    fn split(self) -> (LHS, RHS);
}

/// Index an element of a tuple.
pub trait TupleIdx<const N: usize>: seal::Sealed {
    /// Indexed element type.
    type Output;
    /// Index.
    const INDEX: usize;

    /// Return a tuple element.
    fn idx(self) -> Self::Output;
}

mod seal {
    pub trait Sealed {}
}

macro_rules! impl_tupleops {
    (@impl $($left:ident)* ; $($right:ident)*) => {
        // Join by value
        impl<$($left,)* $($right,)*> TupleJoin<($($right,)*)> for ($($left,)*) {
            type Output = ($($left,)* $($right,)*);

            #[allow(clippy::unused_unit, non_snake_case)]
            fn join(self, other: ($($right,)*)) -> Self::Output {
                let ($($left,)*) = self;
                let ($($right,)*) = other;

                ($($left,)* $($right,)*)
            }
        }

        // Join by reference
        impl<'a, $($left,)* $($right,)*> TupleJoin<&'a ($($right,)*)> for &'a ($($left,)*) {
            type Output = ($(&'a $left,)* $(&'a $right,)*);

            #[allow(clippy::unused_unit, non_snake_case)]
            fn join(self, other: &'a ($($right,)*)) -> Self::Output {
                let ($($left,)*) = self;
                let ($($right,)*) = other;

                ($($left,)* $($right,)*)
            }
        }

        // Split by value
        impl<$($left,)* $($right,)*> TupleSplit<($($left,)*), ($($right,)*)> for ($($left,)* $($right,)*) {
            #[allow(clippy::unused_unit, non_snake_case)]
            fn split(self) -> (($($left,)*), ($($right,)*)) {
                let ($($left,)* $($right,)*) = self;

                (($($left,)*), ($($right,)*))
            }
        }

        // Split by reference
        impl<'a, $($left,)* $($right,)*> TupleSplit<($(&'a $left,)*), ($(&'a $right,)*)> for &'a ($($left,)* $($right,)*) {
            #[allow(clippy::unused_unit, non_snake_case)]
            fn split(self) -> (($(&'a $left,)*), ($(&'a $right,)*)) {
                let ($($left,)* $($right,)*) = self;

                (($($left,)*), ($($right,)*))
            }
        }
    };
    (@recur $($left:ident)* ; ) => {
        impl_tupleops!(@impl $($left)* ; );
    };
    (@recur $($left:ident)* ; $first:ident $($rest:ident)*) => {
        impl_tupleops!(@impl $($left)* ; $first $($rest)*);
        impl_tupleops!(@recur $($left)* $first ; $($rest)*);
    };
    ($($types:ident)*) => {
        impl_tupleops!(@recur ; $($types)*);
    };
}

macro_rules! tuple_impl {
    ($low:literal, $high:literal) => {
        // N - total tuple length
        // This is N^2 so N shouldn't be too large.
        seq!(N in $low..=$high {
            #(
                seq!(J in 0..N {
                    impl<#(T~J,)*> seal::Sealed for (#(T~J,)*) {}
                    impl<'a, #(T~J,)*> seal::Sealed for &'a (#(T~J,)*) {}

                    impl_tupleops!(#(T~J)*);

                    seq!(I in 0..N {
                        // Index by value
                        impl<#(T~J,)*> TupleIdx<I> for (#(T~J,)*) {
                            type Output = T~I;
                            const INDEX: usize = I;

                            #[allow(non_snake_case, unused_variables)]
                            fn idx(self) -> Self::Output {
                                let (#(T~J,)*) = self;
                                T~I
                            }
                        }

                        // Index by reference
                        impl<'a, #(T~J,)*> TupleIdx<I> for &'a (#(T~J,)*) {
                            type Output = &'a T~I;
                            const INDEX: usize = I;

                            #[allow(non_snake_case, unused_variables)]
                            fn idx(self) -> Self::Output {
                                let (#(T~J,)*) = self;
                                T~I
                            }
                        }
                    });
                });
            )*
        });
    };
}

tuple_impl!(0, 16);
#[cfg(any(feature = "tuple_32", feature = "tuple_24"))]
tuple_impl!(17, 24);
#[cfg(any(feature = "tuple_32"))]
tuple_impl!(25, 32);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn join() {
        assert_eq!((1,).join((2,)), (1, 2));
        assert_eq!((1, 'a',).join((2, 'b',)), (1, 'a', 2, 'b'));
    }

    #[test]
    fn join_ref() {
        assert_eq!((&(1,)).join(&(2,)), (&1, &2));
        assert_eq!((&(1, 'a',)).join(&(2, 'b',)), (&1, &'a', &2, &'b'));
    }

    #[test]
    fn join_nil() {
        assert_eq!(().join(()), ());
        assert_eq!((1,).join(()), (1,));
        assert_eq!(().join((1,)), (1,));
    }

    #[test]
    fn split() {
        let ((a, b), rest) = (1, 'a', 2, 'b').split();

        assert_eq!(a, 1);
        assert_eq!(b, 'a');
        assert_eq!(rest, (2, 'b'));
    }

    #[test]
    fn split_ref() {
        let ((a, b), rest) = (&(1, 'a', 2, 'b')).split();

        assert_eq!(a, &1);
        assert_eq!(b, &'a');
        assert_eq!(rest, (&2, &'b'));
    }

    #[test]
    fn split_nil() {
        let ((), ()) = ().split();
        let ((_,), ()) = (1,).split();
        let ((), (_,)) = (1,).split();
    }

    #[test]
    fn index() {
        let a: &char = TupleIdx::<1>::idx(&(1, 'a'));

        assert_eq!(*a, 'a');
    }

    #[test]
    fn boundaries() {
        let seq!(N in 0..16 { (#(_~N,)*) }) =
            seq!(I in 0..8 { (#(I,)*) }).join(seq!(J in 0..8 { (#(J,)*) }));
        #[cfg(any(feature = "tuple_24", feature = "tuple_32"))]
        {
            let seq!(N in 0..24 { (#(_~N,)*) }) =
                seq!(I in 0..12 { (#(I,)*) }).join(seq!(J in 0..12 { (#(J,)*) }));
        }
        #[cfg(any(feature = "tuple_32"))]
        {
            let seq!(N in 0..32 { (#(_~N,)*) }) =
                seq!(I in 0..16 { (#(I,)*) }).join(seq!(J in 0..16 { (#(J,)*) }));
        }
    }
}
