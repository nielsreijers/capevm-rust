use clap::Parser;
use jar::JarReader;

mod jar;

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

    let mut jar = JarReader::new(&args.jar).unwrap();

    println!(
        "Path to file: {:?}, headers: {:?}",
        args.jar, args.cap_header
    );
    println!("");
    println!("Jar: {:?}", jar);
    println!("");
    println!("Classes");
    for file in jar.classfile_names() {
        let class = jar.by_filename(&file).unwrap();
        println!("    {:?}: {} bytes", file, class.len());
    }
}
