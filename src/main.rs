use std::process;

fn main() {
    let money = 100;
    let time = 10; // Minutes
    if let Err(e) = ft_calculator::run() {
        eprintln!("Error! {}", e);
        process::exit(1);
    }
}
