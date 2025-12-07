fn sum_from(arr: &[i32], start: usize) -> i32 {
    if start >= arr.len() {
        return 0;
    }
    
    arr[start..].iter().sum()
}

fn main() {
    println!("int 42");
    println!("str hello");
}