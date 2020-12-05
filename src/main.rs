use structopt::StructOpt;
mod encode;
mod decode;

// the StructOpt crate lets me quickly set up a Cli data structure
#[derive(StructOpt)]
struct Cli {
    arguement: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    // args are parsed from Cli struct
    let args = Cli::from_args();
    // data is extracted from input buffer
    let content = std::fs::read_to_string(&args.path)
        .expect("Could not read file");

    match args {
        e => for line in content.lines() {encode::encode_buf(line.to_string());}
    }
}