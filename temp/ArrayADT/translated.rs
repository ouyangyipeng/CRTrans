use std::io;
use std::io::Write;

#[derive(Debug)]
struct MyArray {
    total_size: usize,
    used_size: usize,
    data: Vec<i32>,
}

fn create_array(a: &mut MyArray, t_size: usize, u_size: usize) {
    a.total_size = t_size;
    a.used_size = u_size;
    a.data = vec![0; t_size];
}

fn show(a: &MyArray) {
    if a.used_size == 0 || a.data.is_empty() {
        return;
    }
    
    for &value in &a.data[..a.used_size] {
        println!("{}", value);
    }
}

fn set_val(a: &mut MyArray) {
    for i in 0..a.used_size {
        print!("Enter {} element:", i);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(n) = input.trim().parse::<i32>() {
            a.data[i] = n;
        } else {
            eprintln!("Invalid input, using default value 0");
            a.data[i] = 0;
        }
    }
}

fn set(a: &mut MyArray, index: usize) {
    if index < a.used_size {
        a.data[index] = 32;
    }
}

fn get(a: &MyArray, n: usize) {
    if n < a.used_size {
        println!("The value in the array in this index is {}", a.data[n]);
    } else {
        eprintln!("Index out of bounds: {} >= {}", n, a.used_size);
    }
}

fn main() {
    let mut marks = MyArray {
        total_size: 0,
        used_size: 0,
        data: Vec::new(),
    };
    
    create_array(&mut marks, 10, 2);
    println!("We are running setVal now");
    set_val(&mut marks);
    println!("We are running show now");
    show(&marks);
    set(&mut marks, 0);
    println!("We are running show now");
    show(&marks);
    get(&marks, 0);
}