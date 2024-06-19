mod war;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    files: Vec<String>,
    #[arg(short, long)]
    main_file: Option<String>,
    #[arg(short, long)]
    function: Option<String>,
}



fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
