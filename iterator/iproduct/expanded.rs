#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
fn test() {
    let iter = ::std::iter::IntoIterator::into_iter(0..3)
        .flat_map(move |a| ::std::iter::IntoIterator::into_iter(0..2).map(move |b| (a, b)));
    {
        match (
            &iter.collect::<Vec<(i32, i32)>>(),
            &<[_]>::into_vec(box [(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (2, 1)]),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    };
    let iter = ::std::iter::IntoIterator::into_iter(0..2).flat_map(move |a| {
        ::std::iter::IntoIterator::into_iter(0..2)
            .flat_map(move |b| ::std::iter::IntoIterator::into_iter(0..2).map(move |c| (a, b, c)))
    });
    {
        match (
            &iter.collect::<Vec<(i32, i32, i32)>>(),
            &<[_]>::into_vec(box [
                (0, 0, 0),
                (0, 0, 1),
                (0, 1, 0),
                (0, 1, 1),
                (1, 0, 0),
                (1, 0, 1),
                (1, 1, 0),
                (1, 1, 1),
            ]),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        }
    }
}
