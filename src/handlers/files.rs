use std::{fs, path::PathBuf};

use anitomy::{Anitomy, ElementCategory};

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

    fn read_episodes(path: PathBuf) -> Result<Show, Box<dyn std::error::Error>> {
        let mut anitomy = Anitomy::new();
        let episodes: Vec<Episode> = fs::read_dir(&path)?
            .filter_map(|ep| {
                if ep.as_ref().ok()?.file_type().ok()?.is_file() {
                    ep.as_ref()
                        .ok()?
                        .file_name()
                        .into_string()
                        .ok()
                        .and_then(|episode_name| {
                            let mut an_name = String::from(episode_name.clone());
                            let mut an_number: Option<String> = None;
                            if let Ok(elements) = anitomy.parse(episode_name.clone()) {
                                an_name =
                                    String::from(elements.get(ElementCategory::EpisodeTitle)?);
                                an_number = Some(String::from(
                                    elements.get(ElementCategory::EpisodeNumber)?,
                                ));
                            }
                            Some(Episode {
                                name: an_name,
                                number: an_number,
                                path_raw: ep.as_ref().ok()?.path(),
                                path: String::from(ep.as_ref().ok()?.path().to_str()?),
                                thumbnail: None,
                            })
                        })
                } else {
                    None
                }
            })
            .collect();

        Ok(Show {
            name: path
                .file_name()
                .ok_or("Could not get directory name")?
                .to_os_string()
                .into_string()
                .ok()
                .ok_or("Could not get directory name")?,
            path: String::from(path.to_str().ok_or("Could not convert path to string")?),
            description: None,
            banner_image: None,
            cover_image: None,
            path_raw: path.clone(),
            episodes,
        })
    }

    pub fn read_library(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.path_raw.is_dir() {
            self.shows = fs::read_dir(&self.path_raw)?
                .map(|p| Library::read_episodes(p.unwrap().path()))
                .filter_map(Result::ok)
                .collect();
            Ok(())
        } else {
            bail!("Supplied Path is not a directory")
        }
    }
}
