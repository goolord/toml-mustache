#[macro_use]
extern crate clap;
extern crate mustache;
extern crate toml;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use toml::Value;

fn main() {
    let matches = clap_app!( toml_mustache =>
        (version: "1.0")
        (author: "Zachary Churchill <zacharyachurchill@gmail.com>")
        (about: "render mustache templates using toml")
        (@arg TEMPLATE: --template +required +takes_value "input template file")
        (@arg OBJECT: --object +required +takes_value "toml object file, to be applied to the template")
        (@arg OUTPUT: --output +takes_value "output file")
        (@arg overwrite: -w --overwrite "overwrite the output file")
    ).get_matches();

    // unwrap is used here because they are required by clap
    let template_file_name: &str = matches.value_of("TEMPLATE").unwrap();
    let object_file_name: &str = matches.value_of("OBJECT").unwrap();
    let output_file_name: Option<&str> = matches.value_of("OUTPUT");
    let will_overwrite: bool = matches.is_present("overwrite");
    // expect on io error
    let mut object_file: File = File::open(object_file_name).expect("Error with object file");
    // read the contents of the files
    let mut object_content = String::new();
    object_file
        .read_to_string(&mut object_content)
        .expect("Error reading object file");
    // compile
    let template: mustache::Template =
        mustache::compile_path(template_file_name).expect("Error compiling template file");
    let object = object_content.parse::<Value>().unwrap();
    let mut handle: Box<dyn Write> = match output_file_name {
        None => Box::new(io::stdout()),
        Some(name) => Box::new(if will_overwrite {
            File::create(name).unwrap()
        } else {
            if Path::exists(Path::new(name)) {
                panic!("Path exists, but the '--overwrite' flag was not set")
            } else {
                File::create(name).unwrap()
            }
        }),
    };
    template.render(&mut handle, &object).unwrap();
}
