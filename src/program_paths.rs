use freedesktop_entry_parser::parse_entry;
use serde::{Deserialize, Serialize};
use std::env;
use std::error;
use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::process::Command;
use std::{cmp::Reverse, error::Error, fs::rename};
#[derive(Debug, Deserialize, Serialize)]
pub struct ProgramData {
    pub num_exec: i32,
    pub name: String,
    pub exec: String,
    pub terminal: bool,
    pub no_display: bool,
}
fn parse_desktop(program: &str) -> ProgramData {
    println!("parsing {program}");
    let binding = parse_entry(program).expect("parse failed");
    let program_entry = binding;
    ProgramData {
        num_exec: 0,
        name: program_entry
            .section("Desktop Entry")
            .attr("Name")
            .expect("Name not found")
            .to_string(),
        exec: program_entry
            .section("Desktop Entry")
            .attr("Exec")
            .expect("Name not found")
            .to_string(),
        terminal: program_entry
            .section("Desktop Entry")
            .attr("Terminal")
            .unwrap_or("false")
            .parse()
            .unwrap_or(false),
        no_display: program_entry
            .section("Desktop Entry")
            .attr("NoDisplay")
            .unwrap_or("false")
            .parse()
            .unwrap_or(false),
    }
}
fn create_cache(program_names: String, path: &Path) {
    // let mut program_data_vec: Vec<freedesktop_entry_parser::Entry> = vec![];

    let output = fs::File::create(path).expect("failed creating temp_path");
    let mut wtr = csv::WriterBuilder::new().from_writer(BufWriter::new(output));
    for line in program_names.lines() {
        let program_data = parse_desktop(line);
        wtr.serialize(program_data).expect("serialize failed");
    }
}
fn check_for_updates(input: String) {
    let program_list_raw = Path::new("/home/ascaletty23/.cache/crab_bucket_mentality.txt");
    let program_list_raw_exist = program_list_raw.exists();
    let cache_dir = Path::new("/home/ascaletty23/.cache/crab_bucket_mentality.cache");
    let cache_dir_exist = cache_dir.exists();
    if !cache_dir_exist {
        fs::write(program_list_raw, &input).expect("failed write");
        create_cache(input, cache_dir);
        println!("cache does not exist, writing program list and cache");
    } else if program_list_raw_exist {
        let contents = fs::read_to_string(program_list_raw).expect("unable to read .desktop");
        if input == contents {
            println!("program_list unchanged");
        } else {
            fs::write(program_list_raw, &input).expect("failed program_list raw write");
            create_cache(input, cache_dir);
            println!("program list changed and cache rewritten");
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
        .has_headers(true)
        .from_path(file)
        .expect("writer gen failed");
    for result in rdr.records() {
        let record = result?;
        let mut program_data = {
            ProgramData {
                num_exec: 0,
                name: "none".to_string(),
                exec: "none".to_string(),
                terminal: false,
                no_display: false,
            }
        };
        program_data.num_exec = record[0].parse().expect("failed num_used parse");
        program_data.name = record[1].to_string();
        let trimming_exec: Vec<&str> = record[2].split(" ").collect();
        program_data.exec = trimming_exec[0].to_string();
        program_data.terminal = record[3].parse().unwrap_or(false);
        program_data.no_display = record[4].parse().unwrap_or(false);
        programs.push(program_data);
    }
    Ok(programs)
}
pub fn edit_cache(
    program_triggerd: &str,
    path: &Path,
    temp_path: &Path,
) -> Result<(), Box<dyn Error>> {
    print!("edit_cache CALLED ");
    let input = fs::File::open(path).expect("failed open input");
    let output = fs::File::create(temp_path).expect("failed creating temp_path");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(input));
    let mut _num_used: i32 = 0;
    let mut wtr = csv::WriterBuilder::new().from_writer(BufWriter::new(output));
    let mut program_data_vec: Vec<ProgramData> = vec![];
    for result in rdr.deserialize() {
        let mut record: ProgramData = result?;
        if record.name == program_triggerd {
            record.num_exec += 1;
        }
        program_data_vec.push(record);
    }
    program_data_vec.sort_by_key(|s| Reverse(s.num_exec));
    for program_data in program_data_vec.iter() {
        wtr.serialize(program_data).expect("failed serialize");
    }
    wtr.flush()?;
    rename(temp_path, path)?;
    Ok(())
}
