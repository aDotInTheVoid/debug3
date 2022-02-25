use crate::{Result, Write};

// impl Write for String {
//     fn write_str(&mut self, s: &str) -> Result {
//         self.push_str(s);
//         Ok(())
//     }

//     fn write_char(&mut self, c: char) -> Result {
//         self.push(c);
//         Ok(())
//     }
// }

// impl Write for Formatter<'_> {
//     fn write_str(&mut self, s: &str) -> Result {
//         self.buf.write_str(s)
//     }

//     fn write_char(&mut self, c: char) -> Result {
//         self.buf.write_char(c)
//     }
// }

// impl<W: Write + ?Sized> Write for &mut W {
//     fn write_str(&mut self, s: &str) -> Result {
//         (**self).write_str(s)
//     }

//     fn write_char(&mut self, c: char) -> Result {
//         (**self).write_char(c)
//     }
// }

impl<T: std::fmt::Write> Write for T {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_str(s).map_err(|_| crate::Error {})
    }
}
