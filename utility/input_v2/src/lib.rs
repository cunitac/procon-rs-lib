pub mod parser;
pub mod source;

use crate::source::Source;
use std::io::StdinLock;

thread_local!(
    #[doc(hidden)]
    pub static STDIN_SOURCE: Source<StdinLock<'static>> =
        Source::new(Box::leak(Box::new(std::io::stdin())).lock());
);

#[macro_export]
macro_rules! input {
    (from($source:expr), ) => {};
    (from($source:expr), $name:tt: $type:ty = $parse:expr, $($rest:tt)*) => {
        let $name: $type = $crate::source::Source::parse(&mut $source, &$parse);
        $crate::input!(from($source), $($rest)*);
    };
    (from($source:expr), $name:tt: $type:ty, $($rest:tt)*) => {
        $crate::input!(from($source), $name: $type = Just, $($rest)*);
    };
    ($($rest:tt)*) => {
        $crate::input!($($rest)*,);
    }
}

#[test]
fn test() {
    use crate::parser::{Base, Just};

    let mut source = Source::from("1 2 4");

    input!(from(source), a: (u32, u32) = (Just, Just), b: u32 = Base(3));
    assert_eq!(a, (1, 2));
    assert_eq!(b, 1);
}
