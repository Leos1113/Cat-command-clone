fn main() {
    if let Err(e) = cat_command::get_args().and_then(cat_command::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
