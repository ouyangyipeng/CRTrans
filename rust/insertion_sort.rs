fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        
        // Shift elements greater than key to the right
        while j > 0 && key < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        
        // Insert key at the correct position
        arr[j] = key;
    }
}

fn test() -> Vec<i32> {
    let mut rng = std::hash::RandomState::new();
    let size = (rand::rand().unsigned_abs() as usize) % 500;
    
    let mut arr = vec![0; size];
    
    // Generate size random numbers from -50 to 49
    for i in 0..size {
        arr[i] = (rand::rand() % 100) - 50;
    }
    
    insertion_sort(&mut arr);
    
    // Verify the array is sorted
    for i in 0..arr.len().saturating_sub(1) {
        assert!(arr[i] <= arr[i + 1], "Array is not sorted at index {}", i);
    }
    
    arr
}

fn main() {
    // Initialize random number generator with current time as seed
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as u32;
    
    rand::srand(seed);
    
    // Call the test function and ignore the returned vector
    let _ = test();
}

// Note: The following helper is needed since we're using rand::srand
mod rand {
    use std::cell::RefCell;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    thread_local! {
        static RNG: RefCell<DefaultHasher> = RefCell::new(DefaultHasher::new());
    }
    
    pub fn srand(seed: u32) {
        RNG.with(|rng| {
            let mut hasher = rng.borrow_mut();
            seed.hash(&mut *hasher);
        });
    }
    
    pub fn rand() -> i32 {
        RNG.with(|rng| {
            let mut hasher = rng.borrow_mut();
            hasher.write_u8(0); // Add some entropy
            hasher.finish() as i32
        })
    }
}