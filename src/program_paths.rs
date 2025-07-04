use std::env;
use std::error;
use std::fs;
use std::fs::exists;
// use std::os::unix::process::CommandExt;
use freedesktop_entry_parser::parse_entry;
use std::path::Path;
use std::process::Command;
use std::{collections::HashMap, fs::File};

fn parse_desktop(program: &str) -> freedesktop_entry_parser::Entry {
    let path = Path::new(program);
    // let contents = fs::read_to_string(&path).expect("unable to read .desktop");
    let program_data = parse_entry(path).expect("failed to parse Desktop Entry");
    let start_cmd = program_data
        .section("Desktop Entry")
        .attr("Exec")
        .expect("Attribute doesn't exist");
    println!("{}", start_cmd);
    program_data
}
fn create_cache(program_names: String) {
    let mut program_data_vec: Vec<freedesktop_entry_parser::Entry> = vec![];
    for line in program_names.lines() {
        println!("parsing {line}");
        program_data_vec.push(parse_desktop(line));
    }
}
fn check_for_match(input: String) -> bool {
    let cache_dir = Path::new("/home/ascaletty23/.cache/crab_bucket_mentality.txt");
    let iscache_exist = cache_dir.exists();
    if iscache_exist == true {
        let contents = fs::read_to_string(&cache_dir).expect("unable to read .desktop");
        if input == contents {
            return true;
        } else {
            fs::write(cache_dir, input);
            return false;
        }
    } else {
        fs::write(cache_dir, input);
        return true;
    }
}
pub fn parse_programs() {
    let applications = Path::new("/usr/share/applications/");
    assert!(env::set_current_dir(&applications).is_ok());
    let output = Command::new("sh")
        .arg("-c")
        .arg("ls -d *.desktop")
        .output()
        .expect("failed to execute process");
    let program_names = String::from_utf8_lossy(&output.stdout);
    if check_for_match(program_names.to_string()) == true {
        print!("cache exist and unchanged");
    } else {
    }
}
pub fn read_cache(
    mut programs: HashMap<String, String>,
) -> Result<HashMap<String, String>, Box<dyn error::Error>> {
    let file = File::open("/home/ascaletty23/.cache/kaolinitedrun.cache")?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        programs.insert(record[0].to_string(), record[1].to_string());
    }
    Ok(programs)
}
