extern crate mustache;
extern crate toml;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;
use toml::Value;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "toml_mustache",
    about = "render mustache templates using toml",
    version = "1.0",
    author = "Zachary Churchill <zacharyachurchill@gmail.com>"
)]
struct Opt {
    #[structopt(long, help = "input template file")]
    template: PathBuf,
    #[structopt(long, help = "toml object file, to be applied to the template")]
    object: PathBuf,
    #[structopt(long, help = "output file")]
    output: Option<PathBuf>,
    #[structopt(short = "w", long, help = "overwrite the output file")]
    overwrite: bool,
}

fn main() {
    let opt = Opt::from_args();

    // unwrap is used here because they are required by clap
    // expect on io error
    let mut object_file: File = File::open(opt.object).expect("Error with object file");
    // read the contents of the files
    let mut object_content = String::new();
    object_file
        .read_to_string(&mut object_content)
        .expect("Error reading object file");
    // compile
    let template: mustache::Template =
        mustache::compile_path(opt.template).expect("Error compiling template file");
    let object = object_content.parse::<Value>().unwrap();
    let mut handle: Box<dyn Write> = match opt.output {
        None => Box::new(io::stdout()),
        Some(name) => Box::new(if opt.overwrite {
            File::create(name).unwrap()
        } else {
            if Path::exists(Path::new(&name)) {
                panic!("Path exists, but the '--overwrite' flag was not set")
            } else {
                File::create(name).unwrap()
            }
        }),
    };
    template.render(&mut handle, &object).unwrap();
}
