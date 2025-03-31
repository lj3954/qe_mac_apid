use std::io::{Read, Seek, SeekFrom, Write};

use fatfs::ReadWriteSeek;

pub struct IoSubset<S>
where
    S: ReadWriteSeek,
{
    inner: S,
    start: u64,
    size: u64,
    offset: u64,
}

impl<S> IoSubset<S>
where
    S: ReadWriteSeek,
{
    pub fn new(inner: S, start: u64, end: u64) -> Self {
        Self {
            inner,
            start,
            size: end - start,
            offset: 0,
        }
    }
}

impl<S> Read for IoSubset<S>
where
    S: ReadWriteSeek,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = buf.len();
        if len == 0 {
            return Ok(0);
        }

        let bytes_left = self.size - self.offset;
        let to_read = len.min(bytes_left as usize);

        if to_read == 0 {
            return Ok(0);
        }

        let read_buf = &mut buf[..to_read];
        self.inner.seek(SeekFrom::Start(self.start + self.offset))?;
        self.inner.read(read_buf)?;
        self.offset += to_read as u64;
        Ok(to_read)
    }
}

impl<S> Write for IoSubset<S>
where
    S: ReadWriteSeek,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = buf.len();
        if len == 0 {
            return Ok(0);
        }

        let bytes_left = self.size - self.offset;
        let to_write = len.min(bytes_left as usize);

        if to_write == 0 {
            return Ok(0);
        }

        let write_buf = &buf[..to_write];

        self.inner.seek(SeekFrom::Start(self.start + self.offset))?;
        let bytes_written = self.inner.write(write_buf)?;

        self.offset += bytes_written as u64;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl<S> Seek for IoSubset<S>
where
    S: ReadWriteSeek,
{
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(pos) => pos,
            SeekFrom::End(pos) => {
                let len = self.size;
                if pos >= 0 {
                    len + pos as u64
                } else {
                    let abs_pos = pos.unsigned_abs();
                    if abs_pos > len {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "attempted to seek before start of file",
                        ));
                    }
                    len - abs_pos
                }
            }
            SeekFrom::Current(pos) => {
                if pos >= 0 {
                    self.offset + pos as u64
                } else {
                    let abs_pos = pos.unsigned_abs();
                    if abs_pos > self.offset {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "attempted to seek before start of file",
                        ));
                    }
                    self.offset - abs_pos
                }
            }
        };
        self.offset = new_pos;
        Ok(new_pos)
    }
}
