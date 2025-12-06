fn display(arr: &[i32]) {
    for &num in arr {
        print!("{} ", num);
    }
    println!();
}

fn swap(first: &mut i32, second: &mut i32) {
    std::mem::swap(first, second);
}

fn bubble_sort(arr: &mut [i32]) {
    let size = arr.len();
    
    for i in 0..size {
        let mut swapped = false;
        // The last i elements are already sorted
        for j in 0..size - i - 1 {
            if arr[j] > arr[j + 1] {
                // Use slice indexing which gives us &mut references
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        
        // If no swaps occurred, the array is already sorted
        if !swapped {
            break;
        }
    }
}

fn test() -> Vec<i32> {
    const SIZE: usize = 10;
    let mut arr = vec![0; SIZE];
    
    // Generate SIZE random numbers from 0 to 99
    for i in 0..SIZE {
        // Use the imported C rand() function
        unsafe {
            arr[i] = (rand() % 100) as i32;
        }
    }
    
    bubble_sort(&mut arr);
    
    // Verify the array is sorted
    for i in 0..SIZE - 1 {
        assert!(arr[i] <= arr[i + 1], "Array is not sorted at index {}", i);
    }
    
    arr
}

fn main() {
    // Initialize random number generator with current time
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as u32;
    
    // Use seed for randomization
    unsafe {
        srand(seed);
    }
    
    // Call test function and get the sorted vector
    let sorted_array = test();
    
    // Verify the array is sorted (test() already does this internally)
    for window in sorted_array.windows(2) {
        assert!(window[0] <= window[1], "Array is not properly sorted");
    }
    
    // Exit with success code (implicit in Rust)
}

// Import the C functions (kept for compatibility with original C code)
extern "C" {
    fn srand(seed: u32);
    fn rand() -> i32;
}

// Bubble sort implementation in safe Rust