use clap::{App, Arg};
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
    read_directory();

    // let file_names = read_dir(svg_folder)
    // .unwrap()
    // .map(|entry| entry.unwrap().file_name().into_string().unwrap())
    // .collect::<Vec<String>>();

    // TODO: Iterate through the list of file names

    // Build the file path

    // Open the file in read-only mode

    // Generate a random 6 character string

    // Read the entire contents of the file into a string

    // Replace all instances of ".st" followed by any characters until "{"
    // with the same string followed by a hyphen and the random string

    // Use a regular expression to remove the specified string

    // Write the modified string to the output file

    // let x = read_dir(svg_folder).unwrap().map();

    // println!("{:#?}", file_names);
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

fn read_directory() {
    return;
}
