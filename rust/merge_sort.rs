fn swap(a: &mut i32, b: &mut i32) {
    std::mem::swap(a, b);
}

fn merge_sort(a: &mut [i32], l: usize, r: usize) {
    if r.saturating_sub(l) == 1 {
        if a[l] > a[r] {
            swap(&mut a[l], &mut a[r]);
        }
    } else if l < r {
        let mid = l + (r - l) / 2;
        merge_sort(a, l, mid);
        merge_sort(a, mid + 1, r);
        merge(a, l, r);
    }
    // No change if l == r
}

fn merge(a: &mut [i32], l: usize, r: usize) {
    let mid = l + (r - l) / 2;
    let mut merged = Vec::with_capacity(r - l + 1);
    
    let (mut left, mut right) = (l, mid + 1);
    
    while left <= mid && right <= r {
        if a[left] <= a[right] {
            merged.push(a[left]);
            left += 1;
        } else {
            merged.push(a[right]);
            right += 1;
        }
    }
    
    // Copy remaining elements from left half
    while left <= mid {
        merged.push(a[left]);
        left += 1;
    }
    
    // Copy remaining elements from right half
    while right <= r {
        merged.push(a[right]);
        right += 1;
    }
    
    // Copy merged elements back to original slice
    a[l..=r].copy_from_slice(&merged);
}




fn main() {
    use std::io::{self, Write};

    // Get array size from user
    print!("Enter Array size: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input for array size!");
            return;
        }
    };

    // Validate array size
    if n == 0 {
        eprintln!("Array size must be greater than 0!");
        return;
    }

    // Create vector to store numbers
    let mut numbers = Vec::with_capacity(n);

    // Read numbers from user
    for i in 0..n {
        print!("Enter number[{}]: ", i);
        io::stdout().flush().unwrap();

        let mut num_input = String::new();
        io::stdin().read_line(&mut num_input).unwrap();
        
        let num: i32 = match num_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid number! Please enter a valid integer.");
                return;
            }
        };
        
        numbers.push(num);
    }

    // Sort the array using merge_sort
    // Note: Using the second signature provided: fn merge_sort(a: &mut [i32], l: usize, r: usize)
    let len = numbers.len();
    if len > 0 {
        merge_sort(&mut numbers, 0, len - 1);
    }

    // Print sorted array
    print!("Sorted Array: ");
    for num in &numbers {
        print!("{} ", num);
    }
    println!();
}