use std::{fs, io, path::PathBuf};

pub struct FileLoader {
    pub path_buf: PathBuf,
    pub bytes: Vec<u8>,
}

impl FileLoader {
    pub fn load(path: &str) -> Result<Self, io::Error> {
        let path_buf = PathBuf::from(path);
        let bytes = fs::read(&path_buf)?;

        Ok(Self { path_buf, bytes })
    }

    /// # Safety
    ///
    /// loaded file must be valid UTF-8
    pub fn source_code(&self) -> &str {
        if cfg!(feature = "safe-loader-utf8") {
            std::str::from_utf8(&self.bytes).unwrap()
        } else {
            unsafe { std::str::from_utf8_unchecked(&self.bytes) }
        }
    }
}
