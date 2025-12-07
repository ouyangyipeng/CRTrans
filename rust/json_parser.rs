use std::io;

fn skipws(s: &str, pos: &mut usize) {
    let bytes = s.as_bytes();
    while *pos < bytes.len() && bytes[*pos].is_ascii_whitespace() {
        *pos += 1;
    }
}

fn print_indent(d: i32) {
    for _ in 0..d {
        print!(" ");
    }
}

fn parse_string_local(s: &str, pos: &mut usize) -> bool {
    print!("\"");
    *pos += 1;
    
    while let Some(&c) = s.as_bytes().get(*pos) {
        if c == b'"' {
            break;
        }
        if c == b'\\' {
            print!("\\");
            *pos += 1;
            if let Some(&c) = s.as_bytes().get(*pos) {
                print!("{}", c as char);
            }
            *pos += 1;
        } else {
            print!("{}", c as char);
            *pos += 1;
        }
    }
    
    if let Some(&c) = s.as_bytes().get(*pos) {
        if c == b'"' {
            print!("\"");
            *pos += 1;
            return true;
        }
    }
    false
}

fn parse_number_local(s: &str, pos: &mut usize) -> bool {
    let start = *pos;
    
    if let Some(&c) = s.as_bytes().get(*pos) {
        if c == b'-' {
            *pos += 1;
        }
    }
    
    let mut has_digit = false;
    while let Some(&c) = s.as_bytes().get(*pos) {
        if c.is_ascii_digit() {
            *pos += 1;
            has_digit = true;
        } else {
            break;
        }
    }
    
    if let Some(&c) = s.as_bytes().get(*pos) {
        if c == b'.' {
            *pos += 1;
            let mut after_decimal = false;
            while let Some(&c) = s.as_bytes().get(*pos) {
                if c.is_ascii_digit() {
                    *pos += 1;
                    after_decimal = true;
                } else {
                    break;
                }
            }
            if !after_decimal {
                *pos = start;
                return false;
            }
        }
    }
    
    if !has_digit {
        *pos = start;
        return false;
    }
    
    for i in start..*pos {
        if let Some(&c) = s.as_bytes().get(i) {
            print!("{}", c as char);
        }
    }
    true
}

fn parse_array_local(s: &str, indent: i32, pos: &mut usize) -> bool {
    print!("[\n");
    *pos += 1;
    skipws(s, pos);
    
    let mut first = true;
    while let Some(&c) = s.as_bytes().get(*pos) {
        if c == b']' {
            break;
        }
        
        if !first {
            print!(",\n");
        }
        first = false;
        
        print_indent(indent + 2);
        if !parse_value_local(s, indent + 2, pos) {
            return false;
        }
        skipws(s, pos);
        
        if let Some(&c) = s.as_bytes().get(*pos) {
            if c == b',' {
                *pos += 1;
                skipws(s, pos);
            } else if c != b']' {
                return false;
            }
        }
    }
    
    if let Some(&c) = s.as_bytes().get(*pos) {
        if c == b']' {
            print!("\n");
            print_indent(indent);
            print!("]");
            *pos += 1;
            return true;
        }
    }
    false
}

fn parse_object_local(s: &str, indent: i32, pos: &mut usize) -> bool {
    print!("{{\n");
    *pos += 1;
    skipws(s, pos);
    
    let mut first = true;
    while let Some(&c) = s.as_bytes().get(*pos) {
        if c == b'}' {
            break;
        }
        
        if !first {
            print!(",\n");
        }
        first = false;
        
        print_indent(indent + 2);
        if let Some(&c) = s.as_bytes().get(*pos) {
            if c == b'"' {
                if !parse_string_local(s, pos) {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        skipws(s, pos);
        
        if let Some(&c) = s.as_bytes().get(*pos) {
            if c == b':' {
                *pos += 1;
            } else {
                return false;
            }
        } else {
            return false;
        }
        skipws(s, pos);
        
        print!(": ");
        if !parse_value_local(s, indent + 2, pos) {
            return false;
        }
        skipws(s, pos);
        
        if let Some(&c) = s.as_bytes().get(*pos) {
            if c == b',' {
                *pos += 1;
                skipws(s, pos);
            } else if c != b'}' {
                return false;
            }
        }
    }
    
    if let Some(&c) = s.as_bytes().get(*pos) {
        if c == b'}' {
            print!("\n");
            print_indent(indent);
            print!("}}");
            *pos += 1;
            return true;
        }
    }
    false
}

fn parse_value_local(s: &str, indent: i32, pos: &mut usize) -> bool {
    skipws(s, pos);
    
    if let Some(c) = s.as_bytes().get(*pos) {
        match c {
            b'"' => {
                parse_string_local(s, pos)
            }
            b'{' => {
                parse_object_local(s, indent, pos)
            }
            b'[' => {
                parse_array_local(s, indent, pos)
            }
            b'-' | b'0'..=b'9' => {
                parse_number_local(s, pos)
            }
            _ => {
                if *pos + 4 <= s.len() && &s.as_bytes()[*pos..*pos + 4] == b"true" {
                    print!("true");
                    *pos += 4;
                    true
                } else if *pos + 5 <= s.len() && &s.as_bytes()[*pos..*pos + 5] == b"false" {
                    print!("false");
                    *pos += 5;
                    true
                } else if *pos + 4 <= s.len() && &s.as_bytes()[*pos..*pos + 4] == b"null" {
                    print!("null");
                    *pos += 4;
                    true
                } else {
                    false
                }
            }
        }
    } else {
        false
    }
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    
    let mut pos = 0;
    if !parse_value_local(&input, 0, &mut pos) {
        println!("null");
        return;
    }
    
    skipws(&input, &mut pos);
    if pos < input.len() {
        println!("null");
        return;
    }
    
    println!();
}