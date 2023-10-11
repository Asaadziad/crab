use clap::Parser;

#[derive(Parser)]
struct CLI {
    pattern : String,
    path: std::path::PathBuf,
}
fn main() {
    let _args = CLI::parse();
    let content = std::fs::read_to_string(&_args.path).expect("could'nt read file");

    for line in content.lines() {
        if line.contains(&_args.pattern){
            println!("{}", line);
        }
    }
}
