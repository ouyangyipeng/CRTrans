fn display(arr: &[i32]) {
    for &num in arr {
        print!("{} ", num);
    }
    println!();
}

fn swap(first: &mut i32, second: &mut i32) {
    std::mem::swap(first, second);
}

fn partition(arr: &mut [i32], lower: usize, upper: usize) -> usize {
    let mut i = lower.wrapping_sub(1); // Use wrapping_sub to handle usize underflow
    
    let pivot = arr[upper];
    
    for j in lower..upper {
        if arr[j] <= pivot {
            i = i.wrapping_add(1); // Use wrapping_add for consistency
            arr.swap(i, j);
        }
    }
    
    arr.swap(i.wrapping_add(1), upper);
    i.wrapping_add(1)
}

fn quick_sort(arr: &mut [i32]) {
    fn quick_sort_helper(arr: &mut [i32], lower: usize, upper: usize) {
        if upper > lower {
            let partition_index = partition(arr, lower, upper);
            if partition_index > 0 {
                quick_sort_helper(arr, lower, partition_index - 1);
            }
            quick_sort_helper(arr, partition_index + 1, upper);
        }
    }
    
    if !arr.is_empty() {
        quick_sort_helper(arr, 0, arr.len() - 1);
    }
}

fn main() {
    use std::io::{self, Write};

    // Read array size
    print!("Enter size of array:\n");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap_or(0);

    if n == 0 {
        println!("Array size must be positive");
        return;
    }

    // Read array elements
    println!("Enter the elements of the array");
    let mut arr: Vec<i32> = Vec::with_capacity(n);
    
    for _ in 0..n {
        let mut element_input = String::new();
        io::stdin().read_line(&mut element_input).unwrap();
        let element: i32 = element_input.trim().parse().unwrap_or(0);
        arr.push(element);
    }

    // Display original array
    print!("Original array: ");
    display(&arr);

    // Sort the array
    let mut arr_slice = arr.as_mut_slice();
    quick_sort(&mut arr_slice);

    // Display sorted array
    print!("Sorted array: ");
    display(&arr);

    // Wait for user input (similar to getchar())
    println!("Press Enter to exit...");
    let mut _dummy = String::new();
    io::stdin().read_line(&mut _dummy).unwrap();
}

// Helper functions (already translated as per provided signatures)

