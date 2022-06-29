#![allow(clippy::unit_cmp, clippy::just_underscores_and_digits)]

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
