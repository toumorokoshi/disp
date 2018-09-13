use std::iter::Peekable;
use std::slice::Iter;

pub fn preprocess(input: &str) -> String {
    let lines = to_lines(input);
    let mut write_buffer = String::new();
    let mut read_buffer = lines.iter().peekable();
    parse_block(0, &mut read_buffer, &mut write_buffer);
    return write_buffer;
}

struct Line {
    buffer: String,
    indent: usize
}

impl Line {
    pub fn new() -> Line {
        return Line{
            buffer: String::new(),
            indent: 0
        };
    }
}

fn to_lines(input: &str) -> Vec<Line> {
    let mut lines = vec![];
    // a buffer to track characters that should nest. these
    // will be considered a single line regardless.
    let mut nested_buffer = vec![];
    let mut is_new_line = true;
    let mut is_comment = false;
    let mut current_line = Line::new();
    for c in input.chars() {
        if is_comment {
            if c != '\n' {
                continue;
            } else {
                is_comment = false;
            }
        }
        match c {
            '\n' => {
                if nested_buffer.len() == 0 {
                    is_new_line = true;
                    lines.push(current_line);
                    current_line = Line::new();
                }
            },
            '\t' => {
                if is_new_line {
                    current_line.indent += 1;
                } else {
                    current_line.buffer.push('\t');
                }
            },
            '#' => {
                is_comment = true;
            },
            '{' => {
                nested_buffer.push('{');
                current_line.buffer.push('{');
            },
            '}' => {
                let do_pop = match nested_buffer.len() {
                    0 => false,
                    n => nested_buffer[n - 1] == '{'
                };
                if do_pop {
                    nested_buffer.pop();
                }
                current_line.buffer.push('}');
            },
            '[' => {
                nested_buffer.push('[');
                current_line.buffer.push('[');
            },
            ']' => {
                let do_pop = match nested_buffer.len() {
                    0 => false,
                    n => nested_buffer[n - 1] == '['
                };
                if do_pop {
                    nested_buffer.pop();
                }
                current_line.buffer.push(']');
            },
            _ => {
                is_new_line = false;
                current_line.buffer.push(c);
            }
        }
    }
    lines.push(current_line);
    return lines;
}

fn parse_block(indent: usize, lines: &mut Peekable<Iter<Line>>, buffer: &mut String) {
    buffer.push('[');
    loop {
        let next_line = match lines.peek() {
                Some(l) => Some(l.clone()),
                None => None
        };
        if let Some(line) = next_line {
            if line.indent < indent {
                break;
            } else if line.buffer.len() == 0 {
                lines.next(); // consume an empty line
            } else {
                parse_statement(indent, lines, buffer);
                buffer.push(' ');
            }
        } else {
            break;
        }
    }
    buffer.push(']');
}

fn parse_statement(indent: usize, input: &mut Peekable<Iter<Line>>, buffer: &mut String) {
    // process first line differently
    let first_line = match input.peek() {
        Some(l) => Some(l.clone()),
        None => None
    };
    match first_line {
        None => {return;}
        Some(l) => {
            let autoparen = Some('(') != l.buffer.chars().nth(0);
            if autoparen {
                buffer.push('(');
            }
            buffer.push_str(&l.buffer);
            input.next();
            let maybe_next_line = match input.peek() {
                Some(l) => Some(l.clone()),
                None => None
            };
            if let Some(next_line) = maybe_next_line {
                if next_line.indent > indent {
                    buffer.push(' ');
                    parse_block(indent + 1, input, buffer);
                }
            }
            if autoparen {
                buffer.push(')');
            }
        }
    }
}
