pub mod i;
pub mod o;

pub mod prelude {
    pub use super::{
        i::{marker::*, Source},
        o::{marker::*, Printer},
    };
    use std::io::{StdinLock, StdoutLock};
    pub type Stdio<'a> = (Source<StdinLock<'a>>, Printer<StdoutLock<'a>>);
}
