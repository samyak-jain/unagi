use std::{fs, io, path::PathBuf};

use anitomy::{Anitomy, ElementCategory};
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Episode {
    pub name: String,
    pub number: Option<String>,
    pub path_raw: PathBuf,
    pub path: String,
    pub thumbnail: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Show {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub banner_image: Option<String>,
    pub cover_image: Option<String>,
    pub season: i64,
    path_raw: PathBuf,
    pub episodes: Vec<Episode>,
}

pub struct Library {
    pub path: String,
    path_raw: PathBuf,
    pub shows: Vec<Show>,
    pub lib_id: i32,
}

impl Library {
    pub fn new(path_string: String, library_id: i32) -> Library {
        Library {
            path: String::new(),
            path_raw: PathBuf::from(path_string),
            shows: vec![],
            lib_id: library_id,
        }
    }

    fn read_episodes(path: PathBuf, season: i64) -> Result<Vec<Show>, Box<dyn std::error::Error>> {
        let mut anitomy = Anitomy::new();
        let mut shows: Vec<Show> = Vec::new();
        let mut episodes: Vec<Episode> = Vec::new();
        let e = fs::read_dir(&path)?;
        let entries = e.collect::<Result<Vec<_>, io::Error>>()?;
        let entry_length = entries.len();
        info!("{:#?}", entries[2]);
        for index in 0..entry_length {
            info!("{}", index);
            let entry = &entries[index];
            let path = entry.path();
            info!("{:#?}", path);
            let metadata = entry.metadata()?;
            let fname = entry
                .file_name()
                .into_string()
                .ok()
                .ok_or("Cannot get fname")?;
            if metadata.is_file() {
                if let Ok(elements) = anitomy.parse(fname) {
                    let an_name = String::from(
                        elements
                            .get(ElementCategory::EpisodeTitle)
                            .ok_or("no title")?,
                    );
                    let an_number = elements
                        .get(ElementCategory::EpisodeNumber)
                        .map(|e| String::from(e));
                    episodes.push(Episode {
                        name: an_name,
                        number: an_number,
                        path_raw: entry.path(),
                        thumbnail: None,
                        path: String::from(
                            path.to_str().ok_or("Could not convert path to string")?,
                        ),
                    });
                }
            } else if metadata.is_dir() {
                let pattern = Regex::new(r".*(?i:season)\s*(\d+).*")?;
                let cap = pattern
                    .captures(&fname)
                    .ok_or("Could not get season number")?;
                let season_number = &cap[0].parse::<i64>()?;
                info!("{}", season_number);
                shows.extend(Library::read_episodes(entry.path(), *season_number)?);
            }
        }

        info!("{:#?}", episodes);
        info!("{:#?}", shows);
        shows.push(Show {
            name: String::from(
                path.file_name()
                    .ok_or("Could not get show name")?
                    .to_str()
                    .ok_or("Could not convert show name to str")?,
            ),
            path: String::from(path.to_str().ok_or("Could not convert path to string")?),
            description: None,
            banner_image: None,
            cover_image: None,
            season,
            episodes,
            path_raw: path,
        });

        Ok(shows)
    }

    pub fn read_library(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.path_raw.is_dir() {
            self.shows = fs::read_dir(&self.path_raw)?
                .map(|p| Library::read_episodes(p.unwrap().path(), 1))
                .filter_map(Result::ok)
                .flatten()
                .collect();
            Ok(())
        } else {
            bail!("Supplied Path is not a directory")
        }
    }
}
