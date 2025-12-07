use std::io::{self, Read};

static mut S: Option<Vec<u8>> = None;
static mut POS: usize = 0;

unsafe fn skipws_global() {
    while POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS].is_ascii_whitespace() {
        POS += 1;
    }
}

unsafe fn print_indent_global(d: i32) {
    for _ in 0..d {
        print!(" ");
    }
}

unsafe fn parse_string() {
    print!("\"");
    POS += 1;
    while POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS] != b'"' {
        if S.as_ref().unwrap()[POS] == b'\\' {
            print!("\\");
            POS += 1;
            if POS < S.as_ref().unwrap().len() {
                print!("{}", S.as_ref().unwrap()[POS] as char);
            }
            POS += 1;
        } else {
            print!("{}", S.as_ref().unwrap()[POS] as char);
            POS += 1;
        }
    }
    if POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS] == b'"' {
        print!("\"");
        POS += 1;
    }
}

unsafe fn parse_number() {
    print_indent_global(0);
    let start = POS;
    if POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS] == b'-' {
        POS += 1;
    }
    while POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS].is_ascii_digit() {
        POS += 1;
    }
    if POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS] == b'.' {
        POS += 1;
        while POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS].is_ascii_digit() {
            POS += 1;
        }
    }
    for i in start..POS {
        print!("{}", S.as_ref().unwrap()[i] as char);
    }
}

unsafe fn parse_array(indent: i32) {
    print!("[\n");
    POS += 1;
    skipws_global();
    let mut first = true;
    while POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS] != b']' {
        if !first {
            print!(",\n");
        }
        first = false;
        print_indent_global(indent + 2);
        parse_value(indent + 2);
        skipws_global();
        if POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS] == b',' {
            POS += 1;
            skipws_global();
        }
    }
    print!("\n");
    print_indent_global(indent);
    print!("]");
    if POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS] == b']' {
        POS += 1;
    }
}

unsafe fn parse_object(indent: i32) {
    print!("{{\n");
    POS += 1;
    skipws_global();
    let mut first = true;
    while POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS] != b'}' {
        if !first {
            print!(",\n");
        }
        first = false;
        print_indent_global(indent + 2);
        if POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS] == b'"' {
            parse_string();
        } else {
            print!("ERROR_KEY");
        }
        skipws_global();
        if POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS] == b':' {
            POS += 1;
        }
        skipws_global();
        print!(": ");
        parse_value(indent + 2);
        skipws_global();
        if POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS] == b',' {
            POS += 1;
            skipws_global();
        }
    }
    print!("\n");
    print_indent_global(indent);
    print!("}}");
    if POS < S.as_ref().unwrap().len() && S.as_ref().unwrap()[POS] == b'}' {
        POS += 1;
    }
}

fn parse_value(indent: i32) {
    unsafe {
        skipws_global();
        if POS >= S.as_ref().unwrap().len() {
            print!("null");
            return;
        }
        
        let current = S.as_ref().unwrap()[POS];
        match current {
            b'"' => parse_string(),
            b'{' => parse_object(indent),
            b'[' => parse_array(indent),
            b'-' => parse_number(),
            b'0'..=b'9' => parse_number(),
            _ => {
                if POS + 4 <= S.as_ref().unwrap().len() && &S.as_ref().unwrap()[POS..POS+4] == b"true" {
                    print!("true");
                    POS += 4;
                } else if POS + 5 <= S.as_ref().unwrap().len() && &S.as_ref().unwrap()[POS..POS+5] == b"false" {
                    print!("false");
                    POS += 5;
                } else if POS + 4 <= S.as_ref().unwrap().len() && &S.as_ref().unwrap()[POS..POS+4] == b"null" {
                    print!("null");
                    POS += 4;
                } else {
                    print!("null");
                    POS += 1;
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    unsafe {
        S = Some(input.into_bytes());
        POS = 0;
        parse_value(0);
        println!();
    }
}