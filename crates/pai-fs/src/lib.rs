use std::{fs, io, path::PathBuf};

pub struct SourceFile {
    pub path: PathBuf,
    pub bytes: Vec<u8>,
}

impl SourceFile {
    pub fn read(path: &str) -> Result<Self, io::Error> {
        let mut bytes = fs::read(path)?;

        bytes.push(b'\0');

        Ok(Self {
            path: PathBuf::from(path),
            bytes,
        })
    }

    /// # Safety
    ///
    /// source file must be valid UTF-8
    pub fn source(&self) -> &str {
        if cfg!(feature = "safe-loader-utf8") {
            std::str::from_utf8(&self.bytes).unwrap()
        } else {
            unsafe { std::str::from_utf8_unchecked(&self.bytes) }
        }
    }
}
