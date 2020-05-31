use std::{env};
use std::process::Command;
use walkdir::WalkDir;

fn main() {
    let mut paths: Vec<String> = vec![];
    let mut hashcat = String::from("");

    for arg in env::args().skip(1) {

        for entry in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir())
        {
            paths.push(String::from(entry.path().to_string_lossy()))
        }
    }

    paths.sort_by(|x, y| x.to_lowercase().cmp(&y.to_lowercase()));

    for f in paths {

        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(format!("md5sum \"{}\"", f));
        let output = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();

        let vec: Vec<&str> = output.split(" ").collect();
        hashcat += vec[0];

    }

    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(format!("echo \"{}\" | md5sum", hashcat));
    let output = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
    print!("{}", output);
}