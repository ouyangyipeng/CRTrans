use std::io::{self, Read};

fn skipws(s: &str, pos: &mut usize) {
    let bytes = s.as_bytes();
    while *pos < s.len() && bytes[*pos].is_ascii_whitespace() {
        *pos += 1;
    }
}

fn print_indent(d: i32) {
    for _ in 0..d {
        print!(" ");
    }
}

fn parse_string(s: &str, pos: &mut usize, _indent: i32) -> bool {
    *pos += 1; // Skip opening quote
    let start = *pos;
    let bytes = s.as_bytes();
    
    while *pos < s.len() && bytes[*pos] != b'"' {
        *pos += 1;
    }
    
    if *pos < s.len() {
        print!("\"{}\"", &s[start..*pos]);
        *pos += 1; // Skip closing quote
        true
    } else {
        false
    }
}

fn parse_number(s: &str, pos: &mut usize, _indent: i32) -> bool {
    let start = *pos;
    let bytes = s.as_bytes();
    
    // Handle optional minus sign
    if bytes[*pos] == b'-' {
        *pos += 1;
    }
    
    // Need at least one digit
    if *pos >= s.len() || !bytes[*pos].is_ascii_digit() {
        *pos = start;
        return false;
    }
    
    // Parse integer part
    while *pos < s.len() && bytes[*pos].is_ascii_digit() {
        *pos += 1;
    }
    
    // Parse fractional part
    if *pos < s.len() && bytes[*pos] == b'.' {
        *pos += 1;
        if *pos >= s.len() || !bytes[*pos].is_ascii_digit() {
            *pos = start;
            return false;
        }
        while *pos < s.len() && bytes[*pos].is_ascii_digit() {
            *pos += 1;
        }
    }
    
    // Parse exponent part
    if *pos < s.len() && (bytes[*pos] == b'e' || bytes[*pos] == b'E') {
        *pos += 1;
        if *pos < s.len() && (bytes[*pos] == b'+' || bytes[*pos] == b'-') {
            *pos += 1;
        }
        if *pos >= s.len() || !bytes[*pos].is_ascii_digit() {
            *pos = start;
            return false;
        }
        while *pos < s.len() && bytes[*pos].is_ascii_digit() {
            *pos += 1;
        }
    }
    
    print!("{}", &s[start..*pos]);
    true
}

fn parse_value(s: &str, pos: &mut usize, indent: i32) -> bool {
    skipws(s, pos);
    
    if *pos >= s.len() {
        return false;
    }
    
    let current_char = s.as_bytes()[*pos] as char;
    
    match current_char {
        '"' => parse_string(s, pos, indent),
        '{' => parse_object(s, pos, indent),
        '[' => parse_array(s, pos, indent as usize),
        _ => {
            // Check if it's a digit or negative sign
            let is_digit_or_negative = current_char.is_ascii_digit() || current_char == '-';
            
            if is_digit_or_negative {
                parse_number(s, pos, indent)
            } else {
                // Check for true/false/null literals
                let remaining = &s[*pos..];
                
                if remaining.starts_with("true") {
                    print!("true");
                    *pos += 4;
                    true
                } else if remaining.starts_with("false") {
                    print!("false");
                    *pos += 5;
                    true
                } else if remaining.starts_with("null") {
                    print!("null");
                    *pos += 4;
                    true
                } else {
                    false
                }
            }
        }
    }
}

fn parse_array(s: &str, pos: &mut usize, indent: usize) -> bool {
    *pos += 1;
    skipws(s, pos);
    
    // Check for empty array immediately
    if *pos < s.len() && s.as_bytes()[*pos] == b']' {
        *pos += 1;
        print!("[]");
        return true;
    }
    
    print!("[\n");
    
    let mut first = true;
    let mut valid = true;
    
    while *pos < s.len() && valid {
        let ch = s.as_bytes()[*pos] as char;
        if ch == ']' {
            break;
        }
        
        if !first {
            print!(",\n");
        }
        first = false;
        
        print_indent(indent as i32 + 2);
        valid = parse_value(s, pos, indent as i32 + 2);
        if !valid {
            break;
        }
        skipws(s, pos);
        
        if *pos < s.len() {
            let ch = s.as_bytes()[*pos] as char;
            if ch == ',' {
                *pos += 1;
                skipws(s, pos);
                // Check for trailing comma
                if *pos < s.len() && s.as_bytes()[*pos] == b']' {
                    valid = false;
                    break;
                }
            }
        }
    }
    
    if !valid {
        return false;
    }
    
    if *pos < s.len() && s.as_bytes()[*pos] == b']' {
        print!("\n");
        print_indent(indent as i32);
        print!("]");
        *pos += 1;
        true
    } else {
        false
    }
}

fn parse_object(s: &str, pos: &mut usize, indent: i32) -> bool {
    *pos += 1;
    skipws(s, pos);
    
    // Check for empty object immediately
    if *pos < s.len() && s.as_bytes()[*pos] == b'}' {
        *pos += 1;
        print!("{{}}");
        return true;
    }
    
    print!("{{\n");
    
    let mut first = true;
    let mut valid = true;
    
    while *pos < s.len() && valid {
        let ch = s.as_bytes()[*pos] as char;
        if ch == '}' {
            break;
        }
        
        if !first {
            print!(",\n");
        }
        first = false;
        
        // Parse key
        print_indent(indent + 2);
        if ch == '"' {
            if !parse_string(s, pos, indent + 2) {
                valid = false;
                break;
            }
        } else {
            valid = false;
            break;
        }
        
        skipws(s, pos);
        
        // Check for colon
        if *pos >= s.len() || s.as_bytes()[*pos] != b':' {
            valid = false;
            break;
        }
        *pos += 1;
        skipws(s, pos);
        
        // Parse value
        valid = parse_value(s, pos, indent + 2);
        if !valid {
            break;
        }
        skipws(s, pos);
        
        // Check for comma
        if *pos < s.len() {
            let ch = s.as_bytes()[*pos] as char;
            if ch == ',' {
                *pos += 1;
                skipws(s, pos);
                // Check for trailing comma
                if *pos < s.len() && s.as_bytes()[*pos] == b'}' {
                    valid = false;
                    break;
                }
            }
        }
    }
    
    if !valid {
        return false;
    }
    
    if *pos < s.len() && s.as_bytes()[*pos] == b'}' {
        print!("\n");
        print_indent(indent);
        print!("}}");
        *pos += 1;
        true
    } else {
        false
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut pos = 0;
    if !parse_value(&input, &mut pos, 0) {
        print!("null");
    }
    
    // Skip any trailing whitespace
    skipws(&input, &mut pos);
    
    // If we didn't consume entire input, output null
    if pos < input.len() {
        print!("null");
    }
    
    println!();
}