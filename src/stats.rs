use std::path::PathBuf;

#[derive(Debug)]
pub struct FileStat {
    pub path: PathBuf,
    pub before: u64,
    pub after: u64,
    pub error: Option<String>,
    #[allow(dead_code)]
    pub duration_ms: u128,
}

impl FileStat {
    pub fn success(path: PathBuf, before: u64, after: u64, duration_ms: u128) -> Self {
        Self {
            path,
            before,
            after,
            error: None,
            duration_ms,
        }
    }

    pub fn error(path: PathBuf, error: String, duration_ms: u128) -> Self {
        Self {
            path,
            before: 0,
            after: 0,
            error: Some(error),
            duration_ms,
        }
    }

    pub fn saved_bytes(&self) -> u64 {
        self.before.saturating_sub(self.after)
    }
}
