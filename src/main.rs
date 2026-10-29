use std::process;

fn main() {
    if let Err(e) = wc_rust::run() {
        eprintln!("{e}");
        process::exit(1)
    }
}
