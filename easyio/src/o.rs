use {
    marker::*,
    std::{
        fmt::{self, Display},
        io::{self, prelude::*, BufWriter},
    },
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
    pub fn print<T: Display>(&mut self, val: T) {
        write!(self, "{}", val).unwrap()
    }
    pub fn println<T: Display>(&mut self, val: T) {
        writeln!(self, "{}", val).unwrap()
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
        ($f:ident: &mut Formatter; $($type:ident, $delim:expr;)*) => {$(
            pub struct $type<'a, I>(pub &'a I);
            impl<'a, I> Display for $type<'a, I>
            where
                &'a I: IntoIterator,
                <&'a I as IntoIterator>::Item: Display,
            {
                fn fmt(&self, $f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let mut iter = self.0.into_iter();
                    if let Some(first) = iter.next() {
                        first.fmt($f)?;
                    }
                    iter.try_for_each(|v| $delim.and_then(|_| v.fmt($f)))
                }
            }
        )*};
    }

    display_with_delim! {
        f: &mut Formatter;
        Lines, writeln!(f);
        Words, write!(f, " ");
        Concat, fmt::Result::Ok(());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut output = Vec::new();

        let mut printer = Printer::new(&mut output);
        printer.print(Lines(&[1, 2, 3]));
        std::mem::drop(printer);

        assert_eq!(output, b"1\n2\n3")
    }
}
