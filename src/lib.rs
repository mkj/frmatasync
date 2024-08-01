use core::fmt;
use std::fmt::Write;

pub struct Frmatasync<W: embedded_io_async::Write> {
    out: W,
}

impl<W> Frmatasync<W> where W: embedded_io_async::Write {
    pub fn new(out: W) -> Self {
        Self {
            out,
        }
    }

    pub async fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<(), W::Error> {
        let mut w = WasteFormatter::default();
        // Format the arguments repeatedly, skipping the output
        // that has already been produced.
        loop {
            w.reset();
            let e = w.write_fmt(args.clone());
            self.out.write_all(&w.current).await?;
            if e.is_ok() {
                break;
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
struct WasteFormatter {
    /// The byte position in this write_fmt() call
    pos: usize,
    /// The byte position that has been written out
    written: usize,
    // 5 byte buffer.
    current: heapless::Vec<u8, 5>,
}

impl WasteFormatter {
    /// Called before each write_fmt()
    fn reset(&mut self) {
        self.pos = 0;
        self.current.clear();
    }
}

impl fmt::Write for WasteFormatter {
    /// Returns Ok(()) once complete.
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let b = s.as_bytes();

        // shortcut
        if self.pos + b.len() < self.written {
            self.pos += b.len();
            return Ok(())
        }

        for &c in b {
            if self.pos < self.written {
                self.pos += 1;
                continue
            }
            self.pos += 1;
            if self.current.push(c).is_err() {
                return Err(fmt::Error);
            }
            self.written += 1;
        }
        Ok(())
        
    }
}
