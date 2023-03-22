use clap::{App, Arg};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::fs;
use std::fs::read_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::LineWriter;
use std::path::Path;

fn main() {
    // TODO: Get input and output folders from user
    let app = App::new("Svclean CLI")
        .version("1.0")
        .author("Simon-Olivier Desautels")
        .about("A cli for cleaning svg for the web")
        .arg(Arg::new("output_folder").required(false).index(1))
        // .arg(Arg::new("output_folder").required(true).index(2))
        .get_matches();

    // TODO:Make sure folders exists

    // let folders = [
    //     app.value_of("input_folder").unwrap(),
    //     app.value_of("output_folder").unwrap(),
    // ];
    let input_folder = "./";
    let mut output_folder = String::from(app.value_of("output_folder").unwrap_or(""));

    output_folder = verify_output_dir(output_folder);

    // TODO: Get a list of all the file names in the svg folder
    let file_names = read_dir(input_folder)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .filter(|entry| Path::new(entry).is_dir() == false)
        .filter(|entry| Path::new(entry).extension().is_some())
        .filter(|entry| Path::new(entry).extension().unwrap() == "svg")
        .collect::<Vec<String>>();

    // TODO: Iterate through the list of file names

    for file_name in file_names {
        // Build the file path
        let file_path = format!("{}/{}", input_folder, file_name);
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
        let re = Regex::new(r"st\d+").unwrap();
        let modified_file_contents = re.replace_all(&file_contents, |caps: &regex::Captures| {
            format!("{}-{}", &caps[0], &random_string)
        });
        // Use a regular expression to remove the specified string
        // Regex::new(r#"<defs>.*?</defs>"#).unwrap();
        let re1 = Regex::new(r#"<defs>[\s\S]*?</defs>"#).unwrap();
        let modified_file_contents = re1.replace_all(&modified_file_contents, "");

        let re2 = Regex::new(r#"clip-path=".*?""#).unwrap();
        let modified_file_contents = re2.replace_all(&modified_file_contents, "");

        let re3 = Regex::new(r#"<clipPath*?[\s\S]*?</clipPath>"#).unwrap();
        let modified_file_contents = re3.replace_all(&modified_file_contents, "");

        let re4 = Regex::new(r#"clip-path:*?[\s\S]*?;"#).unwrap();
        let modified_file_contents = re4.replace_all(&modified_file_contents, "");
        // Write the modified string to the output file
        let mut file_out =
            LineWriter::new(File::create(format!("{}/{}", output_folder, file_name)).unwrap());
        for line in modified_file_contents.lines() {
            let strip_line = line.trim();
            if !strip_line.is_empty() {
                writeln!(file_out, "{}", line).unwrap();
            }
        }
    }
    println!("Great Success!");
    println!("https://www.youtube.com/watch?v=r13riaRKGo0&ab_channel=LeFantasque");
}

fn verify_output_dir(output: String) -> String {
    // for directory_name in directories {
    let mut input_value = output.trim().to_string();
    let path_to_input = Path::new(&input_value);

    if path_to_input.is_dir() == false {
        if input_value == "" {
            input_value = String::from("ouput");
            fs::create_dir_all("ouput").unwrap();
        } else {
            fs::create_dir_all(&input_value).unwrap();
        };
    }
    input_value
}
