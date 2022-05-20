use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    num_bytes: usize,
    num_reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            num_bytes: 0,
            num_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes
    }

    pub fn reads(&self) -> usize {
        self.num_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.wrapped.read(buf)?;
        self.num_reads += 1;
        self.num_bytes += n;
        Ok(n)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    num_bytes: usize,
    num_writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            num_bytes: 0,
            num_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes
    }

    pub fn writes(&self) -> usize {
        self.num_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = self.wrapped.write(buf)?;
        self.num_writes += 1;
        self.num_bytes += n;
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
