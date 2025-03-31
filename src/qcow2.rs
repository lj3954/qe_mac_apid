use std::{
    fmt::Debug,
    io::{self, Read, Seek, SeekFrom, Write},
    path::Path,
};

use anyhow::Result;
use imago::{file::File, qcow2::Qcow2 as Backing, SyncFormatAccess};

pub struct Qcow2 {
    backing: SyncFormatAccess<File>,
    seek_pos: u64,
}

impl Qcow2 {
    pub fn new(path: &Path, dry_run: bool) -> Result<Self> {
        let image = Backing::<File>::open_path_sync(path, !dry_run)?;
        let backing = SyncFormatAccess::new(image)?;
        Ok(Self {
            backing,
            seek_pos: 0,
        })
    }
}

impl Debug for Qcow2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Qcow2")
            .field("seek_pos", &self.seek_pos)
            .finish()
    }
}

impl Read for Qcow2 {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = buf.len();
        if len == 0 {
            return Ok(0);
        }

        let file_size = self.backing.size();
        if self.seek_pos >= file_size {
            return Ok(0);
        }

        let bytes_left = file_size - self.seek_pos;
        let to_read = len.min(bytes_left as usize);

        if to_read == 0 {
            return Ok(0);
        }

        let read_buf = &mut buf[..to_read];
        self.backing.read(read_buf, self.seek_pos)?;
        self.seek_pos += to_read as u64;
        Ok(to_read)
    }
}

impl Write for Qcow2 {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = buf.len();
        if len == 0 {
            return Ok(0);
        }

        self.backing.write(buf, self.seek_pos)?;
        self.seek_pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.backing.flush()
    }
}

impl Seek for Qcow2 {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(pos) => pos,
            SeekFrom::End(pos) => {
                let len = self.backing.size();
                if pos >= 0 {
                    len + pos as u64
                } else {
                    let abs_pos = pos.unsigned_abs();
                    if abs_pos > len {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "attempted to seek before start of file",
                        ));
                    }
                    len - abs_pos
                }
            }
            SeekFrom::Current(pos) => {
                if pos >= 0 {
                    self.seek_pos + pos as u64
                } else {
                    let abs_pos = pos.unsigned_abs();
                    if abs_pos > self.seek_pos {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "attempted to seek before start of file",
                        ));
                    }
                    self.seek_pos - abs_pos
                }
            }
        };
        self.seek_pos = new_pos;
        Ok(new_pos)
    }
}
