use super::*;

macro_rules! impl_tupleops {
    (@impl $($left:ident)* ; $($right:ident)*) => {
        // Join by value
        #[cfg_attr(not(feature = "impl_docs"), doc(hidden))]
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
        #[cfg_attr(not(feature = "impl_docs"), doc(hidden))]
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
        #[cfg_attr(not(feature = "impl_docs"), doc(hidden))]
        impl<$($left,)* $($right,)*> TupleSplit<($($left,)*), ($($right,)*)> for ($($left,)* $($right,)*) {
            #[allow(clippy::unused_unit, non_snake_case)]
            fn split(self) -> (($($left,)*), ($($right,)*)) {
                let ($($left,)* $($right,)*) = self;

                (($($left,)*), ($($right,)*))
            }
        }

        // Split by reference
        #[cfg_attr(not(feature = "impl_docs"), doc(hidden))]
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
