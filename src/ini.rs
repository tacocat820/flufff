use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::collections::HashMap;

pub fn config(conf_path : PathBuf) -> Result<HashMap<String, HashMap<String, String>>, String> {
    let mut conf_file = match File::open(&conf_path) {
        Ok(v) => v,
        Err(_) => { return Err(format!("Cannot open file {}", conf_path.display())); },
    };

    let mut text = String::new();
    match conf_file.read_to_string(&mut text) {
        Ok(v) => v,
        Err(_) => { return Err(format!("Cannot read file {}", conf_path.display())); },
    };

    let words : Vec<&str> = text.split("\n").collect();

    let mut subconf = String::new();
    let mut result : HashMap<String, HashMap<String, String>> = HashMap::new();
    result.insert(subconf.clone(), HashMap::new());

    for i in words.iter() {
        if i.starts_with('#') { continue; }
        if i.starts_with('[') && i.ends_with(']') {

            subconf = i.strip_prefix('[').unwrap().strip_suffix(']').unwrap().to_string();
            result.insert(subconf.clone(), HashMap::new());
                    
        } else {
            let splitted : Vec<&str> = i.splitn(2, "=").collect();

            if splitted.len() <= 1 { continue; }
            let first = splitted[0];
            let second = splitted[1];

            result.get_mut(&subconf).unwrap().insert(first.to_string(), second.to_string());
        }
        
    }

   Ok(result)
}