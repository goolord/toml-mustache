#[macro_use]
extern crate clap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = clap_app!( toml_mustache =>
        (version: "1.0")
        (author: "Zachary Churchill <zacharyachurchill@gmail.com>")
        (about: "render mustache templates using toml")
        (@arg TEMPLATE: -i --input-template +required +takes_value "input template file")
        (@arg OBJECT: -t --toml-object +required +takes_value "toml object file, to be applied to the template")
        (@arg OUTPUT: -o --output +takes_value "output file")
        (@arg overwrite: -w --overwrite "overwrite the output file")
    ).get_matches();

    // unwrap is used here because they are required by clap
    let template_file_name: &str = matches.value_of("TEMPLATE").unwrap();
    let object_file_name: &str = matches.value_of("OBJECT").unwrap();
    let output_fIle_name: Option<&str> = matches.value_of("OUTPUT");
    let will_overwrite: bool = matches.is_present("overwrite");
    // expect on io error
    let mut template_file: File = File::open(template_file_name).expect("Error with template file");
    let mut object_file: File = File::open(object_file_name).expect("Error with object file");
    // read the contents of the files
    let mut template_content = String::new();
    let mut object_content = String::new();
    template_file
        .read_to_string(&mut template_content)
        .expect("Error reading template file");
    object_file
        .read_to_string(&mut object_content)
        .expect("Error reading object file");
}
