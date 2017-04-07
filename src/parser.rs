#![plugin(peg_syntax_ext)]
peg_file! peg_grammar("grammar.rustpeg");

pub enum Token {
    List(Vec<Token>),
    // Dict(HashMap<Token, Token>)
    Value(String)
}

// read a statement into a token
pub fn read(input: &str) -> Token {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut list_buffer =
    for c in input {
        match c {
            ' ' => {
            },
            '[' =>
        }
    }
}
