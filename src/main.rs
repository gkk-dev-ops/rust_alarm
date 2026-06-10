fn main() {
    if let Err(error) = alarm_clock::run() {
        eprintln!("alarm-clock: {error:#}");
        std::process::exit(1);
    }
}
