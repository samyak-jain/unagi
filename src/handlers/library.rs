use std::{fs, path::PathBuf};

pub struct Season {
    path: PathBuf,
}

pub struct Library {
    path: PathBuf,
    seasons: Vec<Season>,
}

impl Library {
    pub fn new(path_string: String) -> Library {
        Library {
            path: PathBuf::from(path_string),
            seasons: vec![],
        }
    }

    pub fn read_library(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.path.is_dir() {
            self.seasons = fs::read_dir(&self.path)?
                .into_iter()
                .map(|p| Season {
                    path: p.unwrap().path(),
                })
                .collect();
            Ok(())
        } else {
            bail!("Supplied Path is not a directory")
        }
    }
}
