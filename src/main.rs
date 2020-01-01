use std::process;

fn main() {
    if let Err(e) = ft_calc::run() {
        eprintln!("Error! {}", e);
        process::exit(1);
    }
}
