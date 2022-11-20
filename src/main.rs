mod lexer;
mod repl;
mod tokens;

fn main() {
    println!("Welcome to the Monkey Programming Language!");
    println!("Feel free to type in commands\n");
    repl::start()
}
