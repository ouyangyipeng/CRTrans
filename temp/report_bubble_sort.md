# Bubble Sort C/Rust Equivalence Assessment

## Summary
**Verdict**: Functionally equivalent with minor differences in random number generation and memory management.  
**Confidence**: 95/100

## Functional Equivalence
The core bubble sort algorithm is functionally equivalent between both implementations:
- Sorting logic and early termination optimization are identical
- Both sort arrays in ascending order using the same comparison (`arr[j] > arr[j + 1]`)
- Both implementations verify sorting correctness with assertions

**Divergences**:
1. **Random number generation**: C uses `srand(time(NULL))` while Rust uses `SystemTime::now()` for seeding
2. **Memory allocation**: C dynamically allocates with `calloc`, Rust uses `vec![0; SIZE]`
3. **Display function**: Rust's `display` takes a slice reference, C's takes pointer+length

## UB/Memory Safety Notes
**C code potential issues**:
- No bounds checking in `display()` or `bubbleSort()` - relies on correct `n`/`size` parameter
- `rand()` returns `int` which may be negative on some platforms (though modulo 100 handles this)
- No null check after `calloc` (though unlikely to fail for small allocation)

**Rust improvements**:
- All array/slice accesses are bounds-checked (panic on out-of-bounds)
- `Vec` ensures proper memory management (no manual `free` needed)
- `arr.swap(j, j + 1)` is safe and optimized
- **Critical issue**: Rust uses `unsafe` blocks to call C's `rand()` and `srand()` functions, which is unnecessary and potentially unsafe. Rust has its own `rand` crate for safe random number generation.

## API/IO Differences
- **Output formatting**: Both produce space-separated numbers with trailing newline
- **Exit codes**: Both exit with success (0) on normal completion
- **Error handling**: Both use `assert()`/`assert!()` for verification (will abort on failure)
- **Random seeding**: C seeds with seconds precision, Rust with seconds since UNIX epoch

## Type/Overflow Considerations
- **C**: `rand()` returns `int`, `% 100` ensures values 0-99, stored in `int`
- **Rust**: Casts `rand()` result (C `int`) to `i32`, then `% 100`
- **Potential issue**: C's `rand()` returns values 0 to `RAND_MAX` (≥32767), but Rust's unsafe call assumes it returns `i32`. On platforms where C `int` is not 32-bit, this could cause truncation.
- **Loop bounds**: Both use `size - 1 - i` pattern, safe for `usize`/`int` types given small array sizes

## Suggestions
1. **Replace unsafe C rand calls with Rust's rand crate**:
   ```rust
   use rand::Rng;
   let mut rng = rand::thread_rng();
   arr[i] = rng.gen_range(0..100);
   ```

2. **Remove redundant verification in `main()`** (test() already asserts correctness):
   ```rust
   let sorted_array = test();
   // Remove the windows verification loop - it's redundant
   ```

3. **Consider making `display()` more idiomatic**:
   ```rust
   fn display(arr: &[i32]) {
       println!("{}", arr.iter()
           .map(|&x| x.to_string())
           .collect::<Vec<_>>()
           .join(" "));
   }
   ```

**Overall**: The Rust implementation is safer and more idiomatic except for the unnecessary unsafe C function calls, which should be replaced with Rust's native random number generation.