
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use indicatif::ProgressBar;

pub fn run(cmd : &String, n : PathBuf, bar : &mut Option<&mut ProgressBar>) -> Result<bool, String> {
    let mut run = match Command::new("bash")
            .current_dir(n)
            .arg("-c").arg(cmd)
            .stdout(Stdio::piped())
            .spawn() {
        Ok(v) => v,
        Err(_) => { return Err("Failed".to_string()) },
    };

    {
        let stdout = run.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("- {}", match line {
                Ok(v) => v,
                Err(_) => "".to_string(),
            });
            if bar.is_some() {
                bar.as_mut().unwrap().inc(1);
            }
        }
    }

    match run.wait() {
        Ok(_) => { return Ok(true); },
        Err(_) => { return Err("Failed".to_string()); },
    }
}