fn main() {
    let args = match siv::cli::parse() {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(2);
        }
    };

    let image = match siv::infra::load_image(&args.path) {
        Ok(i) => i,
        Err(msg) => {
            eprint!("{msg}");
            std::process::exit(2);
        }
    };

    if let Err(msg) = siv::app::run(image, &args) {
        eprint!("{msg}");
        std::process::exit(2);
    }
}
