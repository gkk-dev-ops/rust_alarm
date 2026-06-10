fn main() {
    if let Err(error) = clck::run() {
        eprintln!("clck: {error:#}");
        std::process::exit(1);
    }
}
