use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

use std::net::Shutdown;
use std::net::TcpStream;
use std::net::ToSocketAddrs;

use std::time::Duration;

use crate::Result;

/// The byte used to prompt a client for a command.
pub const PROMPT: u8 = b'>';

/// A wrapper around a [`TcpStream`] that enables buffered I/O calls.
pub struct IoSocket {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl IoSocket {
    /// The default maximum amount of time that can pass before a read call is terminated.
    const READ_TIMEOUT: Duration = Duration::from_secs(1);

    /// The default maximum amount of time that can pass before a write call is terminated.
    const WRITE_TIMEOUT: Duration = Duration::from_secs(1);

    /// Establishes a connection to the VLC player's TCP interface at the given address.
    pub fn connect<A>(addr: A) -> Result<IoSocket>
    where
        A: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addr)?;

        stream.set_read_timeout(Some(Self::READ_TIMEOUT))?;
        stream.set_write_timeout(Some(Self::WRITE_TIMEOUT))?;

        let mut reader = BufReader::new(stream.try_clone()?);
        {
            // Consume the greeting VLC gives a client when it connects.
            let mut greeting = Vec::new();
            reader.read_until(PROMPT, &mut greeting)?;
        }

        let writer = BufWriter::new(stream);

        Ok(Self { reader, writer })
    }

    /// Closes the underling [`TcpStream`]'s connection.
    pub fn shutdown(&self) -> Result<()> {
        self.reader.get_ref().shutdown(Shutdown::Read)?;
        self.writer.get_ref().shutdown(Shutdown::Write)?;
        Ok(())
    }
}

impl Read for IoSocket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl BufRead for IoSocket {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }

    fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let amt = self.reader.read_line(buf)?;

        // The prompt can appear as an "artifact" when `read_line` is called repeatedly, so we need to trim the buffer's output to counter this.
        trim_output(buf);

        Ok(amt)
    }
}

impl Write for IoSocket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

/// Removes any instances of the [`PROMPT`] along with any whitespace characters from the start of the string.
fn trim_output(buf: &mut String) {
    (buf.starts_with(PROMPT as char) || buf.starts_with(' ')).then(|| {
        buf.remove(0);
        trim_output(buf);
    });
}
