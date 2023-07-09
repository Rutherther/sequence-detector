use std::{time::{SystemTime, Duration}, io::{self, BufReader, BufWriter, Write}, fmt, path::PathBuf, fs::{File, self}, ops::Add};

use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceFile {
    time: SystemTime,
    keys: Vec<String>,
}

impl SequenceFile {
    pub fn empty() -> Self {
        Self {
            time: SystemTime::now(),
            keys: Vec::new()
        }
    }

    pub fn time(&self) -> SystemTime {
        self.time
    }

    pub fn keys(&self) -> &Vec<String> {
        &self.keys
    }
}

impl From<Vec<String>> for SequenceFile {
    fn from(value: Vec<String>) -> Self {
        Self {
            time: SystemTime::now(),
            keys: value
        }
    }
}

pub struct SequenceCacher {
    cache_path: PathBuf,
    cache_metadata: SystemTime
}

pub enum CacheError {
    Expired,
    IO(io::Error),
    Serde(serde_json::Error)
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CacheError::Expired =>
                write!(f, "cache has expired"),
            CacheError::IO(..) =>
                write!(f, "an I/O error has occurred."),
            CacheError::Serde(..) =>
                write!(f, "there was a problem with serialization or deserialization")
        }
    }
}

impl From<io::Error> for CacheError {
    fn from(err: io::Error) -> Self {
        CacheError::IO(err)
    }
}

impl From<serde_json::Error> for CacheError {
    fn from(err: serde_json::Error) -> Self {
        CacheError::Serde(err)
    }
}

impl SequenceCacher {
    pub fn new(cache_path: &PathBuf, group_id: &str) -> Self {
        let file_path = cache_path
            .to_string_lossy()
            .replace("{group}", group_id);
        let file_path = PathBuf::from(file_path);

        Self {
            cache_path: file_path,
            cache_metadata: SystemTime::now()
        }
    }

    pub fn try_load(&self, debounce_time: Duration) -> Result<SequenceFile, CacheError> {
        let file = File::open(&self.cache_path)?;
        let reader = BufReader::new(file);
        let read: SequenceFile = serde_json::from_reader(reader)?;

        let debounce_time = read.time.add(debounce_time);
        if SystemTime::now() > debounce_time {
            return Err(CacheError::Expired.into());
        }

        Ok(read)
    }

    pub fn try_cache(&mut self, keys: Vec<String>) -> Result<(), CacheError> {
        {
            let file = File::create(&self.cache_path)?;
            let mut writer = BufWriter::new(file);
            let data: SequenceFile = keys.into();
            serde_json::to_writer(&mut writer, &data)?;
            writer.flush()?;
        }

        self.cache_metadata = fs::metadata(&self.cache_path)?.modified()?;
        Ok(())
    }

    pub fn exists(&self) -> bool {
        self.cache_path.exists()
    }

    pub fn modified(&self) -> io::Result<bool> {
        if !self.exists() {
            Ok(true)
        } else {
            Ok(self.cache_metadata != fs::metadata(&self.cache_path)?.modified()?)
        }
    }

    pub fn remove(&self) -> io::Result<()> {
        if !self.exists() {
            Ok(())
        } else {
            fs::remove_file(&self.cache_path)
        }
    }
}
