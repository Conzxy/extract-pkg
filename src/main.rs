use std::env;
use std::path;
use std::fs::{self, DirEntry};
use std::process::Command;
use std::process;

fn extract_image(scene_file_path: &str) {
    static REPKG_PATH: &str = "./RePKG.exe";

    let output = Command::new(REPKG_PATH)
        .arg("extract").arg(scene_file_path)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        println!("extracted scene file successfully");
        // println!("output = {:?}", output.stdout);
    } else {
        eprintln!("Failed to extract scene file successfully");
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} workshop-path(workshop/content/(number-id) output-path", args[0]);
        process::exit(1);
    }

    let workshop_path = std::path::Path::new(&args[1]);
    let output_path = std::path::Path::new(&args[2]);

    if !workshop_path.exists() {
        println!("The given workshop path doesn't exists");
        process::exit(1);
    }
    if !workshop_path.is_dir() {
        println!("The given workshop path isn't a directory");
        process::exit(1);
    }

    for ent in std::fs::read_dir(workshop_path)? {
        let entry = ent?;
        let path = entry.path();
        if path.is_dir() {
            let scene_file =
                std::fs::read_dir(path)?
                    .find_map(|entry| {
                        entry.ok().and_then(|ent| {
                            if ent.path().extension()? == "pkg" { Some(ent.path()) } else { None }
                        })
                    });
            if let Some(scene_file) = scene_file {
                println!("scene_file = {scene_file:?}");
                extract_image(scene_file.to_str().unwrap());
            }
        }
    }

    for ent in std::fs::read_dir("output/materials")? {
        let entry = ent?;
        let path = entry.path();
        println!("ori file = {path:?}");
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ["png", "gif", "jpg", "jpeg", "webp", "mp4"].contains(&ext.to_str().unwrap()) {
                    let to_path = output_path.join(entry.file_name());
                    if let Err(err) = std::fs::copy(&path, &to_path) {
                        println!("Failed to copy the file: {:?} since {}", path, err);
                    } else {
                        println!("move the file: {:?} to {:?}", path, to_path);
                        let _ = std::fs::remove_file(&path);
                    }
                }
            }
        }
    }

    Ok(())
}
