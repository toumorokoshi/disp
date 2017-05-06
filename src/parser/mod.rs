/// contains all the parsing structures of ghvm
use super::{Token};
peg_file! grammar("grammar.rustpeg");

#[cfg(test)]
mod tests;


/// context for the parser
struct ParserContext {
    start_of_line: bool,
    autowrap_parens: bool,
    current_indent: usize,
}

pub fn parse(body: &str) -> Token {
    let processed_body = preprocess(body);
    grammar::token(&processed_body).unwrap()
}

pub fn preprocess(input: &str) -> String {
    let mut context = ParserContext{
        start_of_line: true,
        current_indent: 0,
        autowrap_parens: true,
    };
    let mut buffer = input.chars().peekable();
    let mut processed_buffer = String::from("[");

    let mut maybe_next_char = None;
    {
        match buffer.peek() {
            Some(r) => {maybe_next_char = Some(r.clone());},
            _ => {}
        }
    }

    while let Some(next_char) = maybe_next_char {
        if context.start_of_line {
            context.autowrap_parens = next_char != '(';
            context.start_of_line = false;
            processed_buffer.push('(');
        }
        match next_char {
            '\n' => {
                buffer.next();
                let mut new_indent = 0;
                loop {
                    let next = Some(&'\t') == buffer.peek();
                    if next {
                        buffer.next();
                        new_indent += 1;
                    } else {
                        break;
                    }
                }
                context.start_of_line = true;
                if new_indent > context.current_indent {
                    processed_buffer.push(' ');
                    processed_buffer.push('[');
                } else if new_indent < context.current_indent {
                    if context.autowrap_parens {
                        processed_buffer.push(')');
                    }
                    processed_buffer.push(' ');
                    processed_buffer.push(']');
                } else {
                    processed_buffer.push(' ');
                    if context.autowrap_parens {
                        processed_buffer.push(')');
                    }
                }
            },
            _ => {
                processed_buffer.push(next_char);
                buffer.next();
            }
        }
        match buffer.peek() {
            Some(r) => {
                maybe_next_char = Some(r.clone());
            },
            None => {
                maybe_next_char = None;
            }
        }
    }

    if context.current_indent > 0 {
        if context.autowrap_parens {
            processed_buffer.push(')');
        }
        processed_buffer.push(' ');
        processed_buffer.push(']');
    }

    if context.autowrap_parens {
        processed_buffer.push(')');
    }

    processed_buffer.push(']');
    return processed_buffer;
}
