use monkey_rs::repl;
use std::io;
fn main() {
    println!(
        "Hello {}. This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");
    repl::start(&mut io::stdin().lock(), &mut io::stdout());
}
