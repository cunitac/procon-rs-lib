use std::{
    fmt::{self, Display},
    io::{self, prelude::*, BufWriter},
};

/// BufWriter のラッパ
pub struct Printer<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> Printer<W> {
    pub fn new(output: W) -> Self {
        Self {
            writer: BufWriter::new(output),
        }
    }
    pub fn print(&mut self, val: impl Display) {
        write!(self, "{}", val).unwrap()
    }
}

impl<W: Write> Write for Printer<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.writer.write_all(buf)
    }
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.writer.write_vectored(bufs)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

pub mod marker {
    use super::*;

    macro_rules! display_with_delim {
        ($f:ident: &mut Formatter; $($type:ident, $delim:expr);* $(;)+) => {$(
            pub struct $type<I>(pub I);
            impl<I> Display for $type<I>
            where
                for<'a> &'a I: IntoIterator,
                for<'a> <&'a I as IntoIterator>::Item: Display,
            {
                fn fmt(&self, $f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let mut iter = self.0.into_iter();
                    if let Some(first) = iter.next() {
                        first.fmt($f)?;
                    }
                    iter.try_for_each(|v| $delim.and_then(|_| v.fmt($f)))
                }
            }
        )*}
    }

    display_with_delim! {
        f: &mut Formatter;
        Lines, writeln!(f);
        Words, write!(f, " ");
        Concat, fmt::Result::Ok(());
    }
}
