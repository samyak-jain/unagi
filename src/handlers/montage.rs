use std::process::Command;

pub fn combine(paths: Vec<String>, dest_path: String) {
    Command::new("/usr/bin/montage")
        .args(&["-border", "0"])
        .args(&["-geometry", "660x"])
        .args(&["-tile", "3x1"])
        .args(paths)
        .arg(dest_path)
        .output()
        .unwrap();
}
