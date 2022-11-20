use crate::lexer::Lexer;
use std::io;

pub fn start() {
    let mut lexer: Lexer;
    let mut input = String::new();
    let std_in = io::stdin();

    // TODO: make it nicer
    // TODO: handle CtrlZ
    println!(">> ");
    read_terminal(&std_in, &mut input);

    while input != String::from("CtrlZ") {
        lexer = Lexer::new(&input);

        while let Some(token) = lexer.next_token() {
            println!("{:#?}", token);
        }

        print!(">> ");
        read_terminal(&std_in, &mut input);
    }
}

fn read_terminal(std_in: &io::Stdin, input: &mut String) {
    if let Err(e) = std_in.read_line(input) {
        panic!("{}", e);
    }
}
