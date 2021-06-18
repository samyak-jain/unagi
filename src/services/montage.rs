use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use std::{io, process::Command};

pub fn combine(paths: Vec<String>, dest_str: String) {
    let dest_path = Path::new(&dest_str);

    if dest_path.exists() {
        fs::remove_dir_all(&dest_path).unwrap();
    }
    fs::create_dir_all(&dest_path).unwrap();

    for (index, path) in paths.iter().enumerate() {
        let response = reqwest::blocking::get(path).unwrap();
        if response.status().is_success() {
            let body = response.bytes().unwrap();
            let mut file = File::create(format!("{}/{}.png", dest_str, index)).unwrap();
            file.write_all(&body.to_vec()).unwrap();
        }
    }

    let output = Command::new("/usr/bin/convert")
        .arg("(")
        .args((0..3).map(|n| format!("{}/{}.png", dest_str, n)))
        .args(&["-resize", "x660"])
        .arg("+append")
        .arg(")")
        .arg("-append")
        .args(&["-frame", "1x1"])
        .arg(format!("{}/combined.png", dest_str))
        .output()
        .unwrap();

    println!("Status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stdout().write_all(&output.stderr).unwrap();
}
