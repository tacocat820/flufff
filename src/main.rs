//use which::which;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::path::Path;

use directories::ProjectDirs;
use indicatif::{ProgressBar, ProgressStyle};

// TODO: add escape characters so you can include colons in the command 
// TODO: add timer

mod ini;
mod exe;

fn upd(path : PathBuf, types : &HashMap<&String, Vec<&str>>, bar : &mut ProgressBar) -> Result<bool, String> {
    let mut conf_path = path.clone(); conf_path.push("INFO.ini");

    let conf = match ini::config(conf_path) {
        Ok(v) => v,
        Err(e) => { return Err(format!("Cannot find INFO.ini! {}", e)); },
    };

    let t = match conf.get("").unwrap().get("TYPE") {
        Some(v) => v,
        None => { return Err("Invalid INFO.ini! (no TYPE value)".to_string()) },
    };

    let vars = conf.get("").unwrap();

    let mut cmd = types.get(t).unwrap().get(2).expect("Incorrect config separation").to_string();
    for i in vars {
        cmd = cmd.replace(&format!("[{}]", i.0), i.1);
    }

    match exe::run(&cmd, path, &mut Some(bar)) {
        Ok(_) => {},
        Err(v) => { return Err(format!("Cannot update! {}", v)) },
    } 

    Ok(true)

}

fn main() {

    let projectdirs = ProjectDirs::from("", "",  "flufff").unwrap();
    let config = projectdirs.config_dir();
    if !config.exists() {
        fs::create_dir_all(config).expect("Cannot create the config folder!");
        fs::write(config.join("conf.ini"), "[types]\ngit=url:git clone [url] .:git pull")
            .expect("Cannot create the conf.ini file!");
        fs::write(config.join("track"), "")
            .expect("Cannot create the track file!");
    }

    let confpath = config.join("conf.ini");
    let trackpath = config.join("track");
    
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.is_empty() {
        println!("Expected an action ('new', 'remove', 'update')"); return;
    }

    let rawtrack = fs::read_to_string(&trackpath).expect("expected a 'track' file to exist");
    let mut track : Vec<&str> = rawtrack.split('\n').collect();
    let conf = match ini::config(confpath) {
        Ok(v) => v,
        Err(e) => { println!("Cannot read conf.ini! {}", e); return; },
    };

    let types : HashMap<&String, Vec<&str>> = conf.get("types").expect("Expected 'types' in config").iter().map(|s| (s.0, s.1.splitn(3, ':').collect::<Vec<&str>>())).collect();

    match args[0].as_str() {
        "new" => { 
            let n = match args.get(1) {
                Some(v) => v,
                None => { println!("Expected name"); return; },
            };
            let t = match args.get(2) {
                Some(v) => v,
                None => { println!("Expected type"); return; },
            };
            
            if !types.contains_key(t) { println!("No such type!"); return; }

            let var : Vec<&str> = types.get(t).expect("No such type!").first().expect("Incorrect config separation").split(',').collect();
            let mut vars : HashMap<String, String> = HashMap::new();
            
            let mut info : String = String::new();
            info += &format!("TYPE={}\n\n", t);
            let mut cmd = types.get(t).unwrap().get(1).expect("Incorrect config separation").to_string();

            let mut ii = 0;
            for i in var {
                if i.is_empty() { continue; }
                let r = match args.get(3 + ii) {
                    Some(v) => { v },
                    None => { println!("Expected {}", i); return; },
                };

                vars.insert(i.to_string(), r.to_string());
                info += &format!("{}={}\n", i, r);
                cmd = cmd.replace(&format!("[{}]", i), r);
                ii += 1;
            }

            fs::create_dir(n).expect("Unable to create directory");

            match exe::run(&cmd, PathBuf::from(n), &mut None) {
                Ok(_) => {},
                Err(v) => { println!("Command failed! {}", v); return; },
            } 

            fs::write(format!("{}/INFO.ini", n), info).expect("Unable to make an info file");

            let totrackpath = PathBuf::from(n);
            let canon = totrackpath.canonicalize().expect("Weird path");
            let add = canon.as_os_str().to_str().expect("Unable to convert name from os string to str");

            let rawtrack = fs::read_to_string(&trackpath).expect("expected a 'track' file to exist");
            track = rawtrack.split('\n').collect();

            if !track.contains(&add) { track.push(add); }
            
            if track[0].is_empty() { track.remove(0); }
            fs::write(&trackpath, track.join("\n")).expect("Unable to modify the track file");

            println!("Finished!");
        }
        "remove" => {
            let n = args.get(1).expect("Expected the backup path");
            if track.contains(&n.as_str()) {
                let index = track.iter().position(|&r| r == n).unwrap();
                track.remove(index);
                
                println!("{} untracked!", n);
            } else {
                println!("Not found! (use the absolute path please)"); return
            }
            
            fs::write(&trackpath, track.join("\n")).expect("Unable to modify the track file");
        }
        "update" => {
            let mut errors : Vec<String> = Vec::new();
            
            let mut pb = ProgressBar::new(track.len() as u64);
            pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar:.red}]")
                .unwrap()
                .progress_chars("=>-"));

            let mut ii = 0;
            for i in track {

                let path = PathBuf::from(i);

                if !(Path::new(&path).exists()) {
                    println!("> {} doesn't exist!", path.display());
                    errors.push(format!("{} doesn't exist! Remove it with flufff remove", path.display()));

                    ii += 1;
                    pb.set_position(ii);
                    continue;
                }
                println!("> Updating {}...", path.display());

                match upd(path.clone(), &types, &mut pb) {
                    Ok(_) => {},
                    Err(v) => { errors.push(format!("{} : {}", path.display(), v)); },
                }

                ii += 1;
                pb.set_position(ii);

            }

            println!("-------------");
            if !errors.is_empty() { println!("> Some backups failed!") }
            for i in errors {
                println!("- {}", i);
            }

            println!("Finished!");
        }
        _ => { 
            println!("No such subcommand!");
        } 
    }

    

}
