use std::env;
use std::error;
use std::fs;
use std::fs::exists;
use std::process::Termination;
// use std::os::unix::process::CommandExt;
use freedesktop_entry_parser::parse_entry;
use std::path::Path;
use std::process::Command;
use std::{collections::HashMap, error::Error, fs::File, io, process};
pub struct ProgramData {
    pub name: String,
    pub exec: String,
    pub terminal: bool,
    pub no_display: bool,
}

fn parse_desktop(program: &str) -> freedesktop_entry_parser::Entry {
    let path = Path::new(program);
    println!("parsing {program}");
    let program_data = parse_entry(path).expect("failed to parse Desktop Entry");
    program_data
}
fn create_cache(program_names: String, path: &Path) {
    // let mut program_data_vec: Vec<freedesktop_entry_parser::Entry> = vec![];
    let mut wtr = csv::Writer::from_path(path).expect("writer gen failed");
    for line in program_names.lines() {
        println!("creating cache for {line}");
        wtr.write_record(&[
            "0",
            parse_desktop(line)
                .section("Desktop Entry")
                .attr("Name")
                .expect("attribute does not exist"),
            parse_desktop(line)
                .section("Desktop Entry")
                .attr("Exec")
                .expect("attribute does not exist"),
            parse_desktop(line)
                .section("Desktop Entry")
                .attr("Terminal")
                .unwrap_or("false"),
            parse_desktop(line)
                .section("Desktop Entry")
                .attr("NoDisplay")
                .unwrap_or("false"),
        ])
        .expect("writing to csv failed");
    }
}
fn check_for_updates(input: String) {
    let program_list_raw = Path::new("/home/ascaletty23/.cache/crab_bucket_mentality.txt");
    let program_list_raw_exist = program_list_raw.exists();
    let cache_dir = Path::new("/home/ascaletty23/.cache/crab_bucket_mentality.cache");
    let cache_dir_exist = cache_dir.exists();
    if cache_dir_exist != true {
        fs::write(program_list_raw, &input);
        create_cache(input, cache_dir);
        println!("cache does not exist, writing program list and cache");
    } else {
        if program_list_raw_exist == true {
            let contents = fs::read_to_string(&program_list_raw).expect("unable to read .desktop");
            if input == contents {
                println!("program_list unchanged");
            } else {
                fs::write(program_list_raw, &input);
                create_cache(input, cache_dir);
                println!("program list changed and cache rewritten");
            }
        } else {
            println!("error occured in checking for updates");
        }
    }
}
pub fn parse_programs(programs: Vec<ProgramData>) -> Result<Vec<ProgramData>, Box<dyn Error>> {
    let applications = Path::new("/usr/share/applications/");
    assert!(env::set_current_dir(&applications).is_ok());
    let output = Command::new("sh")
        .arg("-c")
        .arg("ls -d *.desktop")
        .output()
        .expect("failed to execute process");
    let program_names = String::from_utf8_lossy(&output.stdout);
    check_for_updates(program_names.to_string());
    read_cache(programs)
}
fn read_cache(mut programs: Vec<ProgramData>) -> Result<Vec<ProgramData>, Box<dyn error::Error>> {
    let file = Path::new("/home/ascaletty23/.cache/crab_bucket_mentality.cache");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file)
        .expect("writer gen failed");
    for result in rdr.records() {
        let record = result?;
        println!("reading {:?}", record);
        let mut program_data = {
            ProgramData {
                name: "none".to_string(),
                exec: "none".to_string(),
                terminal: false,
                no_display: false,
            }
        };
        program_data.name = record[1].to_string();
        let trimming_exec: Vec<&str> = record[2].split(" ").collect();
        program_data.exec = trimming_exec[0].to_string();
        program_data.terminal = record[3].parse().unwrap_or(false);
        program_data.no_display = record[4].parse().unwrap_or(false);
        programs.push(program_data);
    }
    Ok(programs)
}
