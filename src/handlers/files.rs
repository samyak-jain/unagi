use std::{ffi::OsStr, fs, path::PathBuf};

use anitomy::{Anitomy, ElementCategory};
use regex::Regex;

const SUPPORTED_FILETYPES: &[&str] = &["mkv", "mp4"];

#[derive(Clone, Debug)]
pub struct Episode {
    pub name: String,
    pub number: Option<i32>,
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
    pub thumbnail: Option<String>,
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
            thumbnail: None,
        }
    }

    fn read_episodes(
        path: PathBuf,
        season: i64,
        parent: bool,
    ) -> Result<Vec<Show>, Box<dyn std::error::Error>> {
        info!("Path: {:#?}", path);
        let mut anitomy = Anitomy::new();
        let mut shows: Vec<Show> = Vec::new();
        let mut episodes: Vec<Episode> = Vec::new();
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            let fname = entry
                .file_name()
                .into_string()
                .ok()
                .ok_or("Cannot get fname")?;
            if metadata.is_file() {
                if let Some(extension) = path.extension().and_then(OsStr::to_str) {
                    if SUPPORTED_FILETYPES.contains(&extension) {
                        let elements = match anitomy.parse(fname.clone()) {
                            Ok(ele) => ele,
                            Err(ele) => ele,
                        };
                        let an_name = String::from(
                            elements
                                .get(ElementCategory::EpisodeTitle)
                                .unwrap_or(&fname),
                        );
                        let an_number = elements
                            .get(ElementCategory::EpisodeNumber)
                            .map(|e| String::from(e));

                        info!("name: {}, number: {:#?}", an_name, an_number);

                        episodes.push(Episode {
                            name: an_name,
                            number: an_number.map(|num| num.parse()).transpose()?,
                            path_raw: entry.path(),
                            thumbnail: None,
                            path: String::from(
                                path.to_str().ok_or("Could not convert path to string")?,
                            ),
                        });
                    }
                }
            } else if metadata.is_dir() {
                let pattern = Regex::new(r".*(?i:season)\s*(\d+).*")?;
                let cap = pattern
                    .captures(&fname)
                    .ok_or("Could not get season number")?;
                let season_number = String::from(&cap[1]).parse::<i64>();
                if season_number.is_err() {
                    info!("Capture: {}", &cap[1]);
                    bail!("parse error");
                } else {
                    shows.extend(Library::read_episodes(
                        entry.path(),
                        season_number.unwrap(),
                        true,
                    )?);
                }
            }
        }

        let show_name_path = if parent {
            path.parent().ok_or("Could not get parent of path")?
        } else {
            path.as_path()
        };

        if shows.is_empty() {
            shows.push(Show {
                name: String::from(
                    show_name_path
                        .file_name()
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
        }

        Ok(shows)
    }

    pub fn read_library(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.path_raw.is_dir() {
            self.shows = fs::read_dir(&self.path_raw)?
                .map(|p| Library::read_episodes(p.unwrap().path(), 1, false))
                .filter_map(Result::ok)
                .flatten()
                .collect();

            Ok(())
        } else {
            bail!("Supplied Path is not a directory")
        }
    }
}
