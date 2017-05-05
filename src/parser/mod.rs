/// contains all the parsing structures of ghvm
use super::{Token};
peg_file! grammar("grammar.rustpeg");

#[cfg(test)]
mod tests;


/// context for the parser
pub struct Context {}

pub fn parse(body: &str) -> Token {
    let processed_body = preprocess(body);
    let mut context = Context{};
    grammar::token(&processed_body, &mut context).unwrap()
}


/// preprocess a string, returning
/// back something that can be processed
/// by the peg parser
///
/// converts all newline + indents into
/// an indented string.
pub fn preprocess(input: &str) -> String {
    let mut current_indent = 0;
    let mut processed_buffer = String::new();
    let mut buffer = input.chars().peekable();
    loop {
        if let Some(c) = buffer.next() {
            match c {
                '\n' => {
                    processed_buffer.push(' ');
                    let mut new_indent = 0;
                    loop {
                        let mut next = false;
                        if let Some(&'\t') = buffer.peek() {
                            next = true;
                        }
                        if next {
                            buffer.next();
                            new_indent += 1;
                        } else {
                            break;
                        }
                    }
                    if new_indent > current_indent {
                        processed_buffer.push('[');
                    } else if new_indent < current_indent {
                        processed_buffer.push(']');
                    }
                    current_indent = new_indent;
                },
                '\t'|' ' => {
                    loop {
                        let mut next = false;
                        if let Some(c) = buffer.peek() {
                            match c {
                                &'\t'|&' ' => {next = true},
                                _ => {}
                            }
                        }
                        if next {
                            buffer.next();
                        } else {
                            break;
                        }
                    }
                    processed_buffer.push(' ');
                },
                _ => processed_buffer.push(c)
            }
        } else {
            break;
        }
    }
    return processed_buffer;
}
