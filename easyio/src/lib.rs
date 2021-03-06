pub mod i;
pub mod o;

pub mod prelude {
    pub use super::{
        i::{marker::*, Source},
        o::{marker::*, Printer},
    };
    use std::io::{StdinLock, StdoutLock};
    pub type IO<R, W> = (Source<R>, Printer<W>);
    pub type Stdio<'a> = IO<StdinLock<'a>, StdoutLock<'a>>;
}
