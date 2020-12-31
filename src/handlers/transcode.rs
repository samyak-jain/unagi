use std::{fs, io, path::PathBuf, process::Command};

use crate::errors::TranscodingError;

fn check_gpu_availability() -> Result<bool, io::Error> {
    let gpu_checker = Command::new("/usr/bin/nvidia-smi").output();
    Ok(gpu_checker?.status.success())
}

pub fn start_transcoding(
    source_path: PathBuf,
    destination_path: PathBuf,
    is_hw_enabled: bool,
) -> Result<Command, TranscodingError> {
    fs::create_dir_all(destination_path.clone())?;

    let mut destination_files = destination_path.clone();
    destination_files.push(r#"stream_%v"#);
    destination_files.push(r#"file%03d"#);
    destination_files.set_extension("ts");

    let mut destination_playlist = destination_path;
    destination_playlist.push(r#"stream_%v"#);
    destination_playlist.set_extension("m3u8");

    let mut ffmpeg = Command::new("/usr/bin/ffmpeg");
    let should_enable_gpu = is_hw_enabled && check_gpu_availability()?;

    let command = match should_enable_gpu {
        true => ffmpeg
            .args(&["-vsync", "0"])
            .args(&["-hwaccel", "cuvid"])
            .arg("-i"),
        false => ffmpeg.arg("-i"),
    };

    let codec = if should_enable_gpu {
        "h264_nvenc"
    } else {
        "h264"
    };

    let filter = if should_enable_gpu {
        "[v:0]hwupload_cuda,split=2[vtemp001][vout002];[vtemp001]hwupload_cuda,scale_npp=w=960:h=540,hwdownload[vout001]"
    } else {
        "[v:0]split=2[vtemp001][vout002];[vtemp001]scale=w=960:h=540[vout001]"
    };

    command
        .arg(source_path)
        .args(&["-c:a", "aac"]) // Change audio stream codec to aac
        .args(&["-c:v", codec]) // Change video codec
        .args(&["-c:s", "webvtt"]) // Change subtitle format to webvtt
        .args(&["-crf", "20"]) // Constant rate factor. Higher is lossyer
        .args(&["-filter_complex", filter]) // Split stream into two. Take temporary stream and downscale it
        .args(&["-map", "[vout001]", "-b:v:0", "2000k"]) // Set bitrate for video for the 1st video stream
        .args(&["-map", "[vout002]", "-b:v:1", "6000k"]) // Set bitrate for video for the 2nd video stream
        .args(&["-map", "0:v", "-map", "0:a", "-map", "0:s"])
        .args(&["-map", "0:v", "-map", "0:a", "-map", "0:s"])
        .args(&["-var_stream_map", "v:0,a:0,s:0 v:1,a:1,s:1"])
        .args(&["-force_key_frames:v", "expr:gte(t,n_forced*2.000)"])
        .args(&["-hls_time", "6"])
        .args(&["-hls_playlist_type", "event"])
        .args(&[
            "-hls_flags",
            "delete_segments+independent_segments+discont_start+program_date_time",
        ])
        .args(&["-master_pl_name", "master.m3u8"])
        .arg("-hls_segment_filename")
        .arg(destination_files)
        .arg(destination_playlist);

    Ok(ffmpeg)
}
