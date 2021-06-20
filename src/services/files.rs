use std::{collections::HashMap, fs, path::PathBuf};

use anitomy::{Anitomy, ElementCategory};
use regex::Regex;

const SUPPORTED_FILETYPES: &[&str] = &["mkv", "mp4"];
const EXCLUDE_FILENAMES: &[&str] = &["NCOP", "NCED"];

#[derive(Clone, Debug)]
pub struct EpisodeDetails {
    pub name: String,
    pub path: String,
    pub thumbnail: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ShowDetails {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub banner_image: Option<String>,
    pub cover_image: Option<String>,
    pub season: i32,
    pub episodes: HashMap<i32, EpisodeDetails>,
}

pub struct LibraryDirectory {
    pub path: String,
    pub thumbnail: Option<String>,
    pub shows: Vec<ShowDetails>,
    pub lib_id: i32,
}

impl LibraryDirectory {
    /// Recurse into a given directory and add all shows
    pub fn create_library(path: String, id: i32) -> anyhow::Result<LibraryDirectory> {
        let path_buffer = PathBuf::from(&path);
        if !path_buffer.is_dir() {
            return Err(anyhow!("Given path is not a directory"));
        }

        Ok(LibraryDirectory {
            lib_id: id,
            path,
            thumbnail: None,
            shows: Self::read_shows(path_buffer)?,
        })
    }

    /// Read episodes reads all the episode files in a directory and sturctures it
    /// Required Parameters:
    /// path: A PathBuf that is the directory of the show we are reading
    /// season: The season number that we are assuming for the show
    /// parent: This is a flag to determine if this function is called as a result of a recursion
    /// or not
    ///
    /// The directory structure that we are assuming is for example:
    /// |- LibraryName
    /// |  |- Pyscho Pass
    /// |  |  |- episode 01.mkv
    /// |  |  |- episode 02.mkv
    ///
    /// |  |- Attack On Titan
    /// |  |  |- Season 01
    /// |  |  |  |- episode 01.mkv
    /// |  |  |  |- episode 02.mkv
    /// |  |  |- Season 02
    /// |  |  |  |- episode 01.mkv
    /// |  |  |  |- episode 02.mkv
    ///
    /// As shown above, there are two types of representing shows.
    /// 1. With just one directory with the name of the show. In that case, we assume
    /// we are talking about season 1
    /// 2. The name of the directory is the name of the show. But, we have multiple
    /// sub directories name Season 01, Season 02, etc... We use this to determine the season
    /// number
    ///
    /// For episode names, the above are just illustrative. We get the episode name by using
    /// anitomy to parse the filename and the episode show not have the above scheme. The episode
    /// names can be left as is.
    ///
    /// TODO: Parallelize this using Rayon
    fn read_shows(library_path: PathBuf) -> anyhow::Result<Vec<ShowDetails>> {
        // Iterate through all the children of the path. This is fine since we want to
        // iterate through all the children anyway. So there is no benefit lazy loading.
        // We can use this length and then make the entire allocation for the vector at once
        let dir_entry_iterator = fs::read_dir(&library_path)?.collect::<Vec<_>>();
        let mut show_list: Vec<ShowDetails> = Vec::with_capacity(dir_entry_iterator.len());

        for entry in dir_entry_iterator {
            let show_file = entry?;

            // If the library directory contains a file, we ignore it since it isn't useful to us
            if show_file.file_type()?.is_file() {
                continue;
            }

            // Get the name of the show
            let show_os_string = show_file.file_name();
            let show_name = show_os_string.to_string_lossy();

            // Let's assume at first that only the top level show directory exists
            // Meaning we don't have the Season 01, Season 02 etc... layout
            let mut do_seasons_exist = false;

            for sub_entry in fs::read_dir(&show_file.path())? {
                let sub_path = sub_entry?;

                // If we find a directory inside the show directory, we need to recurse into it
                // for the episodes
                if sub_path.path().is_dir() {
                    if let Some(show) =
                        Self::create_show(sub_path.path(), Some(show_name.to_string()))?
                    {
                        show_list.push(show);

                        // If we are setting this flag to true because if a child directory
                        // has a show then it means that the parent directory will not have
                        // any episodes on it own. Only other child directories of the parent
                        // will have the episodes
                        //
                        // Notice that the flag does not get set when the create_show method
                        // returns None. This is since None is representing an error state where
                        // we were unable to correctly parse the directory name. This error
                        // however, should not be propogated like the others. We should just ignore
                        // it.
                        do_seasons_exist = true;
                    }
                }
            }

            // If the flag is set to false, we were not able to find any shows in the sub
            // directory. This mean that the top level directory should contain the show.
            if !do_seasons_exist {
                if let Some(show) = Self::create_show(show_file.path(), None)? {
                    show_list.push(show);
                }
            }
        }

        Ok(show_list)
    }

    // TODO: Better IO Error handling. Give feeback if there's an permission or interrupt error
    // TODO: Maybe think of better semantics than returning Result<Option>?
    //
    // Currently we are using Result for a catch all for any kind of errors. Whether it is IO or
    // Regex errors.
    //
    // The Option is used for the specific case when we find a directory but it is not formatted
    // correctly in the Season + Number format. Example: Season 01, Season 02, etc...
    //
    // Ideally there should be a better way to express this.
    fn create_show(path: PathBuf, parent: Option<String>) -> anyhow::Result<Option<ShowDetails>> {
        let directory_name = path
            .file_name()
            .ok_or(anyhow!("Directory does not have a name"))?
            .to_string_lossy()
            .to_string();

        let (season_number, show_name) = if let Some(parent_name) = parent {
            // This scenario, we are getting the parent name from the function caller.
            // The parent name is the directory that actually contains the name of the show
            //
            // This name of this directory contains which season the show is in.
            // For Example:
            // Season 01, Season 02, etc...

            // Regex to capture the season number from directory's name
            let pattern = Regex::new(r".*(?i:season)\s*(\d+).*")?;

            // If we are unable to match the pattern, we are returning an error
            let cap = pattern.captures(&directory_name);

            if let Some(capture) = cap {
                (String::from(&capture[1]).parse::<i32>()?, parent_name)
            } else {
                return Ok(None);
            }
        } else {
            (1, directory_name)
        };

        let mut anitomy = Anitomy::new();

        // Iterate through all the children of the path. This is fine since we want to
        // iterate through all the children anyway. So there is no benefit lazy loading.
        let mut dir_entry_iterator = fs::read_dir(&path)?
            .filter_map(|r| r.ok())
            .collect::<Vec<_>>();

        // We are sorting through all the files in ascending order.
        // We are doing this is because, there is a chance that we might fail parsing the episode
        // number from the name of the file. In that case, we will be using the index of the file
        // to guess the episode number
        dir_entry_iterator.sort_by_key(|entry| entry.path());

        // We can use the length and then make the entire allocation for the hashmap at once
        let mut episode_map: HashMap<i32, EpisodeDetails> =
            HashMap::with_capacity(dir_entry_iterator.len());

        // We are counting the number of successfull episodes that we parsed. We are doing this
        // because we are using the index of the loop as a fallback for guessing what the episode
        // number or a particular file will be. We don't want any failed attempts to count towards
        // the episode number
        let mut number_successfull = 0;

        for episode_file in dir_entry_iterator {
            let file_type = episode_file.file_type()?;

            // If we are in a directory, we can ignore
            if file_type.is_dir() {
                continue;
            }

            // TODO: We are currently not handling symlinks

            let file_os_string = episode_file.file_name();
            let file_name = file_os_string.to_string_lossy();

            // Exclude certain filename from being indexed
            if EXCLUDE_FILENAMES
                .iter()
                .any(|exclude| file_name.contains(exclude))
            {
                continue;
            }

            let episode_path = episode_file.path();
            let file_extension = episode_path.extension();

            // If we are able to get the file extension, we can check with the list
            // of file extensions we support.
            // If we are not able to find a file extension, we will be lenient and let
            // the program continue anyway
            if let Some(extension) = file_extension {
                let lossy_extension: &str = &extension.to_string_lossy();
                if !SUPPORTED_FILETYPES.contains(&lossy_extension) {
                    continue;
                }
            }

            // Use anitomy to parse the file name of the episode
            // Anitomy returns elements whether it succeeds or fails
            // When it succeeds, it is able to get all the elements,
            // but when it fails, it still returns the elements it was able to parse
            // We make use of whatever elements we get
            let anitomy_elements = anitomy.parse(&file_name).map_or_else(|ele| ele, |ele| ele);

            // Try getting the Episode number and title
            let try_episode_name = anitomy_elements.get(ElementCategory::EpisodeTitle);
            let try_episode_number = anitomy_elements
                .get(ElementCategory::EpisodeNumber)
                .and_then(|number_string| number_string.parse::<i32>().ok());

            // If we are not able to use the episode name from anitomy,
            // we fallback to using the file name as the episode name.
            //
            // If we are unable to parse the episode number using anitomy,
            // we fallback to using the index of the loop.
            //
            // TODO: Figure out a better way to get a fallback for the episode number.
            //
            // If we are not able to figure out the episode name, we are using an api
            // to fetch episode details. However, this requires the episode number to be accurate.
            episode_map.insert(
                try_episode_number.unwrap_or(number_successfull + 1),
                EpisodeDetails {
                    name: try_episode_name.unwrap_or(&file_name).to_string(),
                    path: episode_path.to_string_lossy().to_string(),
                    thumbnail: None,
                },
            );

            number_successfull += 1;
        }

        Ok(Some(ShowDetails {
            name: show_name,
            path: path.to_string_lossy().to_string(),
            description: None,
            banner_image: None,
            cover_image: None,
            season: season_number,
            episodes: episode_map,
        }))
    }
}
