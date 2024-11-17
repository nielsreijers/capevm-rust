use clap::Parser;
use opcodes::Opcode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the jar file to process
    jar: std::path::PathBuf,

    /// Add a header file for a CapeVM library the current
    #[arg(short)]
    cap_header: Vec<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();

    println!("Jar file: {:?}, headers: {:?}", args.jar, args.cap_header);
}
