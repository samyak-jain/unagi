use std::{ffi::OsStr, fs, path::PathBuf};

use anitomy::{Anitomy, ElementCategory};
use regex::Regex;

const SUPPORTED_FILETYPES: &[&str] = &["mkv", "mp4"];
const EXCLUDE_FILENAMES: &[&str] = &["NCOP", "NCED"];

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
    pub shows: Vec<Show>,
    pub lib_id: i32,
}

impl Library {
    pub fn read(path: String, id: i32) -> anyhow::Result<Library> {
        let path_buffer = PathBuf::from(&path);
        if !path_buffer.is_dir() {
            return Err(anyhow!("Given path is not a directory"));
        }

        let shows = fs::read_dir(path_buffer)?
            .map(|p| match p {
                Ok(directory) => Library::read_episodes(directory.path(), 1, false),
                Err(_) => Err(anyhow!("IO Error")),
            })
            .filter_map(Result::ok)
            .flatten()
            .collect();

        Ok(Library {
            lib_id: id,
            path,
            thumbnail: None,
            shows,
        })
    }

    fn read_episodes(path: PathBuf, season: i64, parent: bool) -> anyhow::Result<Vec<Show>> {
        let mut anitomy = Anitomy::new();
        let mut shows: Vec<Show> = Vec::new();
        let mut episodes: Vec<Episode> = Vec::new();

        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            let pattern = Regex::new(r".*(?i:season)\s*(\d+).*")?;
            let fname = entry
                .file_name()
                .into_string()
                .map_err(|err| anyhow!("Cannot get filename"))?;
            if metadata.is_file() {
                if EXCLUDE_FILENAMES.contains(&&fname[..]) {
                    continue;
                }
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

                        episodes.push(Episode {
                            name: an_name,
                            number: an_number.map(|num| num.parse()).transpose()?,
                            path_raw: entry.path(),
                            thumbnail: None,
                            path: String::from(
                                path.to_str()
                                    .ok_or(anyhow!("Could not convert path to string"))?,
                            ),
                        });
                    }
                }
            } else if metadata.is_dir() && pattern.is_match(&fname) {
                let cap = pattern
                    .captures(&fname)
                    .ok_or(anyhow!("Could not get season number"))?;
                let season_number = String::from(&cap[1]).parse::<i64>();
                if season_number.is_err() {
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
            path.parent()
                .ok_or(anyhow!("Could not get parent of path"))?
        } else {
            path.as_path()
        };

        if shows.is_empty() {
            shows.push(Show {
                name: String::from(
                    show_name_path
                        .file_name()
                        .ok_or(anyhow!("Could not get show name"))?
                        .to_str()
                        .ok_or(anyhow!("Could not convert show name to str"))?,
                ),
                path: String::from(
                    path.to_str()
                        .ok_or(anyhow!("Could not convert path to string"))?,
                ),
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
}
