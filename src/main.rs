// rcli csv -i input.csv -o output.json --header -d ','

use std::path::Path;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Read csv or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_file_exists)]
    input: String,

    #[arg(short, long, default_value = "out.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(short = 'a', long, default_value_t = true)]
    header: bool,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}

fn verify_file_exists(path: &str) -> Result<String, &'static str> {
    if Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err("File not found")
    }
}
