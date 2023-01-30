use crossterm::style::Stylize;

pub mod file;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("{}", "HELP".bold());
        println!("");
        println!("{}", "COMMANDS".bold());
        println!("  connect:            Connect to a specific ssh connection");
        println!("  connection:         Create, modify or remove an ssh connection");
        println!("");
        println!("{}", "ADDITIONAL COMMANDS".bold());
        println!("  connection add:     Create a new ssh connection");
        println!("  connection remove:  Remove an ssh connection");
    }

    if args[1] == "connect" {}
}
