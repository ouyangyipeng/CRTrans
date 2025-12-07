#![crate_name = "translated_converting_matrix_to_sparse"]

#[derive(Default, Clone)]
struct Term {
    row: i32,
    col: i32,
    value: i32,
}

fn create_sparse(a: &[[i32; 10]; 10], row: usize, column: usize, c: &mut [Term]) {
    let mut k = 0;
    for i in 0..row {
        for j in 0..column {
            if a[i][j] != 0 {
                k += 1;
            }
        }
    }
    
    if c.is_empty() {
        return;
    }
    c[0].row = row as i32;
    c[0].col = column as i32;
    c[0].value = k as i32;
    
    let mut count = 1;
    for i in 0..row {
        for j in 0..column {
            if a[i][j] != 0 {
                if count < c.len() {
                    c[count].row = i as i32;
                    c[count].col = j as i32;
                    c[count].value = a[i][j];
                    count += 1;
                } else {
                    return;
                }
            }
        }
    }
    
    println!("\nSparse form list of matrix in triple form is");
    for i in 0..=k {
        if i < c.len() {
            println!("{}\t{}\t{}", c[i].row, c[i].col, c[i].value);
        }
    }
}

use std::io::{self, Write};

fn main() {
    const MAX_TERMS: usize = 100;
    const SIZE: usize = 10;

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut ar1 = [[0; SIZE]; SIZE];
    let mut ar2 = [[0; SIZE]; SIZE];
    let mut x = vec![Term::default(); MAX_TERMS];
    let mut y = vec![Term::default(); MAX_TERMS];

    print!("\nEnter the row and column size of the 1st matrix : ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut sizes = input.split_whitespace();
    a = sizes.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    b = sizes.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    println!("Enter elements of 1st matrix row wise :");
    for i in 0..a {
        for j in 0..b {
            print!("Enter next element : ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            ar1[i][j] = input.trim().parse().unwrap_or(0);
        }
    }

    print!("\nEnter the row and column size of the 2nd matrix : ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut sizes = input.split_whitespace();
    c = sizes.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    d = sizes.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    println!("Enter elements of 2nd matrix row wise :");
    for i in 0..c {
        for j in 0..d {
            print!("Enter next element : ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            ar2[i][j] = input.trim().parse().unwrap_or(0);
        }
    }

    println!("Entered matrices are");
    for i in 0..a {
        for j in 0..b {
            print!("{}  ", ar1[i][j]);
        }
        println!();
    }
    println!("\n and \n");
    for i in 0..c {
        for j in 0..d {
            print!("{}  ", ar2[i][j]);
        }
        println!();
    }

    println!("\nConverting the first matrix to triple form");
    create_sparse(&ar1, a, b, &mut x);
    println!("\nConverting the second matrix to triple form");
    create_sparse(&ar2, c, d, &mut y);
}