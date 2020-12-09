use std::{
    error::Error,
    path::PathBuf,
    process::{Command, Output},
};

use crate::SETTINGS;

pub fn start_transcoding(
    source_path: PathBuf,
    destination_path: PathBuf,
) -> Result<Output, Box<dyn Error>> {
    let resolution = "";

    let mut destination_files = destination_path.clone();
    destination_files.push(format!("{}", resolution));
    destination_files.set_extension(".ts");

    let mut destination_playlist = destination_path;
    destination_playlist.push(resolution);
    destination_playlist.set_extension("m3u8");

    let is_hw_enabled = SETTINGS.read()?.get_bool("HW_ACCEL")?;

    let mut ffmpeg = Command::new("/usr/bin/ffmpeg");
    let command = match is_hw_enabled {
        true => ffmpeg
            .args(&["-vsync", "0"])
            .args(&["-hwaccel", "cuvid"])
            .arg("-i"),
        false => ffmpeg.arg("-i"),
    };

    Ok(command
        .arg(source_path)
        .args(&["-c:a", "copy"])
        .args(&["-c:v", if is_hw_enabled { "h264_nvenc" } else { "h264" }])
        .args(&["-sc_threshold", "0"])
        .args(&["-b:v", "1400000"])
        .args(&["-hls_time", "6"])
        .args(&["-hls_playlist_type", "vod"])
        .arg("-hls_segment_filename")
        .arg(destination_files)
        .arg(destination_playlist)
        .output()?)
}
