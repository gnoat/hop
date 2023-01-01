use hopper;
use std::{env, io};

fn main() -> io::Result<()> {
    let hopper = hopper::hopper::Hopper::new(".config/hop");

    let output: String = match env::args().nth(1) {
        Some(cmd) => match cmd.as_str() {
            "add" => hopper.add_hop(
                env::current_dir().unwrap(),
                &env::args()
                    .nth(2)
                    .expect("Need to specify name to add hop."),
            ),
            "ls" => hopper.list_hops(),
            _ => hopper.hop(&cmd),
        },
        None => "echo \"[error] Invalid command.\"".to_string(),
    };
    println!("{}", output);
    Ok(())
}
