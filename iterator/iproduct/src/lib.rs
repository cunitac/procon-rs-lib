//! ```
//! use iproduct::*;
//!
//! let iter = iproduct!(0..2, "abc".chars(), vec![true, false]);
//! assert!(
//!     iter.eq([
//!         (0, 'a', true),
//!         (0, 'a', false),
//!         (0, 'b', true),
//!         (0, 'b', false),
//!         (0, 'c', true),
//!         (0, 'c', false),
//!         (1, 'a', true),
//!         (1, 'a', false),
//!         (1, 'b', true),
//!         (1, 'b', false),
//!         (1, 'c', true),
//!         (1, 'c', false),
//!     ].iter().copied())
//! )
//! ```

#[macro_export]
macro_rules! iproduct {
    ([$($i:ident),*][$i0:ident, $($is:ident),*] $iter0:expr $(,)?) => {
        ::std::iter::IntoIterator::into_iter($iter0)
            .map(move |$i0| ($($i,)* $i0,))
    };
    ([$($i:ident),*][$i0:ident, $($is:ident),*] $iter0:expr, $($iter:expr),+ $(,)?) => {
        ::std::iter::IntoIterator::into_iter($iter0)
            .flat_map(move |$i0| $crate::iproduct!([$($i,)* $i0][$($is),*] $($iter),*))
    };
    ($($rest:tt)*) => {
        $crate::iproduct!([][a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r] $($rest)*)
    };
}

#[test]
fn test() {
    let iter = iproduct!(0..4);
    assert_eq!(iter.collect::<Vec<(i32,)>>(), vec![(0,), (1,), (2,), (3,)],);
    let iter = iproduct!(0..3, 0..2);
    assert_eq!(
        iter.collect::<Vec<(i32, i32)>>(),
        vec![(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (2, 1)]
    );
    let iter = iproduct!(0..2, 0..2, 0..2);
    assert_eq!(
        iter.collect::<Vec<(i32, i32, i32)>>(),
        vec![
            (0, 0, 0),
            (0, 0, 1),
            (0, 1, 0),
            (0, 1, 1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 1, 0),
            (1, 1, 1)
        ]
    );
    let iter = iproduct!("ab".chars(), "xyz".chars());
    assert!(iter
        .map(|(a, b)| format!("{}{}", a, b))
        .eq(["ax", "ay", "az", "bx", "by", "bz"].iter().copied()));
}
