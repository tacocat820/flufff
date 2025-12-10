use tar::Builder;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use tar::Archive;

pub fn zip() -> Result<File, String> {
    if Path::new("BACKUP.tar.gz").exists() { match fs::remove_file("BACKUP.tar.gz") {
        Ok(_) => {},
        Err(v) => { println!("BACKUP.tar.gz exists but is impossible to remove: {}", v) },
    } }
    
    let compressed_file = match File::create("BACKUP.tar.gz") {
        Ok(v) => v,
        Err(e) => { return Err(e.to_string()) },
    };

    let mut encoder = GzEncoder::new(compressed_file, Compression::fast());

    {
        let mut archive = Builder::new(&mut encoder);

        let ls = match fs::read_dir("./") {
            Ok(v) => v,
            Err(e) => { return Err(format!("Error listing files: {}", e.to_string())) },
        };
        for i in ls.into_iter() { 
            if i.is_err() { continue; }
            if i.as_ref().unwrap().file_name() == "BACKUP.tar.gz" { continue; }

            println!("archiving {}", i.as_ref().unwrap().path().display());

            if i.as_ref().unwrap().metadata().unwrap().is_file() {
                match archive.append_file(i.as_ref().unwrap().file_name(), &mut File::open(i.unwrap().path()).unwrap()) { // TODO not use unwrap please
                    Ok(_) => {},
                    Err(e) => { return Err(e.to_string()) },
                } 
            } else {
                match archive.append_dir_all(i.as_ref().unwrap().file_name(), i.unwrap().path()) {
                    Ok(_) => {},
                    Err(e) => { return Err(e.to_string()) },
                } 
            }
        }
    }

    match encoder.finish() {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }

}

pub fn unzip() -> Result<(), String> {
    if !PathBuf::from("BACKUP.tar.gz").exists() { return Err("No BACKUP.tar.gz!".to_string()); }

    let ls = match fs::read_dir("./") {
        Ok(v) => v,
        Err(e) => { return Err(format!("Error listing files: {}", e.to_string())) },
    };
    for i in ls.into_iter() {
        if i.is_err() { continue; }

        if i.as_ref().unwrap().file_name() != "BACKUP.tar.gz" {
            println!("removing {} !!!", i.as_ref().unwrap().path().display());
            
            if i.as_ref().unwrap().metadata().unwrap().is_file() {
                match fs::remove_file(i.unwrap().path()) {
                    Ok(_) => {},
                    Err(e) => { return Err(format!("Error deleting file: {}", e)) },
                }
            } else {
                match fs::remove_dir_all(i.unwrap().path()) {
                    Ok(_) => {},
                    Err(e) => { return Err(format!("Error deleting dir: {}", e)) },
                }
            }
            
        }
    }

    let path = "BACKUP.tar.gz";

    let tar_gz = match File::open(path) {
        Ok(v) => v,
        Err(e) => { return Err(e.to_string()) },
    };
    let tar = GzDecoder::new(tar_gz);
    let mut archive: Archive<GzDecoder<File>> = Archive::new(tar);
    match archive.unpack(".") {
        Ok(_) => { Ok(()) },
        Err(e) => { Err(e.to_string()) },
    }

}