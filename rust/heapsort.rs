use std::io::{self, Write};

struct MyArray {
    ptr: Box<[i32]>,
    total_size: i32,
    used_size: i32,
}

impl MyArray {
    fn new() -> Self {
        MyArray {
            ptr: Box::new([]),
            total_size: 0,
            used_size: 0,
        }
    }
}

fn CreateArray(a: &mut MyArray, tSize: i32, uSize: i32) -> Result<(), Box<dyn std::error::Error>> {
    a.total_size = tSize;
    a.used_size = uSize;
    
    let size = tSize as usize;
    a.ptr = vec![0; size].into_boxed_slice();
    
    Ok(())
}

fn show(a: &MyArray) {
    for i in 0..a.used_size {
        println!("{}", a.ptr[i as usize]);
    }
}

fn set_val(a: &mut MyArray) {
    for i in 0..a.used_size {
        print!("Enter element {}: ", i);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<i32>() {
            Ok(n) => {
                if (i as usize) < a.ptr.len() {
                    a.ptr[i as usize] = n;
                }
            }
            Err(_) => {
                if (i as usize) < a.ptr.len() {
                    a.ptr[i as usize] = 0;
                }
            }
        }
    }
}

fn set(a: &mut MyArray, index: usize) -> Result<(), &'static str> {
    if index >= a.ptr.len() {
        return Err("Index out of bounds");
    }
    
    a.ptr[index] = 32;
    Ok(())
}

fn get(a: &MyArray, n: usize) {
    if let Some(value) = a.ptr.get(n) {
        println!("The value in the array in this index is {}", value);
    } else {
        println!("Index {} is out of bounds", n);
    }
}

fn main() {
    println!("int 42");
    println!("str hello");
}