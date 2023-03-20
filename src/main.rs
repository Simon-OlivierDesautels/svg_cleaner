use clap::{App, Arg};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::fmt::format;
use std::fs::read_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::LineWriter;
use std::path::Path;
use std::{io, vec};

fn main() {
    // TODO: Get input and output folders from user
    let app = App::new("Svclean CLI")
        .version("1.0")
        .author("Simon-Olivier Desautels")
        .about("A cli for cleaning svg for the web")
        .arg(Arg::new("input_folder").required(true).index(1))
        .arg(Arg::new("output_folder").required(true).index(2))
        .get_matches();

    // let svg_folder = define_folder(String::from(app.value_of("input_folder").unwrap()));
    // let svg_output_folder = define_folder(app.value_of("output_folder").unwrap());
    // TODO:Make sure folders exists
    // ,app.value_of("output_folder")

    let folders = [
        app.value_of("input_folder").unwrap(),
        app.value_of("output_folder").unwrap(),
    ];

    verify_folders(folders);

    // TODO: Get a list of all the file names in the svg folder
    let file_names = read_dir(folders[0])
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .filter(|entry| Path::new(entry).extension().unwrap() == "svg")
        .collect::<Vec<String>>();

    // TODO: Iterate through the list of file names

    for file_name in file_names {
        // Build the file path
        let file_path = format!("{}/{}", folders[0], file_name);

        // Open the file in read-only mode
        let file = File::open(&file_path).unwrap();
        let mut buf_reader = BufReader::new(file);

        // Generate a random 6 character string
        let random_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        // Read the entire contents of the file into a string
        let mut file_contents = String::new();
        buf_reader.read_to_string(&mut file_contents).unwrap();

        // Replace all instances of ".st" followed by any characters until "{"
        // with the same string followed by a hyphen and the random string
        let re = Regex::new(r"\.st.*?\{").unwrap();
        let modified_file_contents = re.replace_all(&file_contents, |caps: &regex::Captures| {
            format!("{}-{}{{", &caps[0][1..caps[0].len() - 1], random_string)
        });
        // Use a regular expression to remove the specified string
        // Regex::new(r#"<defs>.*?</defs>"#).unwrap();
        let re1 = Regex::new(r#"<defs>[\s\S]*?</defs>"#).unwrap();
        let modified_file_contents = re1.replace_all(&modified_file_contents, "");

        let re2 = Regex::new(r#"clip-path=".*?""#).unwrap();
        let modified_file_contents = re2.replace_all(&modified_file_contents, "");

        // Write the modified string to the output file
        let mut file_out =
            LineWriter::new(File::create(format!("{}/{}", folders[1], file_name)).unwrap());
        for line in modified_file_contents.lines() {
            let strip_line = line.trim();
            if !strip_line.is_empty() {
                writeln!(file_out, "{}", line).unwrap();
            }
        }
    }
}

fn verify_folders(directories: [&str; 2]) {
    for directory_name in directories {
        let input_value = directory_name.trim().to_string();
        let path_to_input = Path::new(&input_value);

        if path_to_input.is_dir() == false {
            println!("There seem to be no '{}' directory", &input_value);
            std::process::exit(1);
        }
    }
    println!("All clear");
}
