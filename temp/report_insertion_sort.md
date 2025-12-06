# Equivalence Assessment Report

## Summary
**Overall Verdict**: Not functionally equivalent  
**Confidence**: 40/100

The Rust implementation has significant deviations from the C version in random number generation and algorithm behavior, making the outputs non-equivalent. The insertion sort logic itself is mathematically equivalent, but the testing framework differs substantially.

## Functional Equivalence
**Outputs/Side Effects**: **DO NOT MATCH**

Key divergences:
1. **Random number generation**: The C code uses `rand()` from stdlib with time-based seeding via `srand(time(NULL))`. The Rust implementation uses a custom `rand` module with a `DefaultHasher`-based PRNG that produces completely different sequences.
2. **Algorithm indexing**: While mathematically equivalent, the Rust version uses different indexing (`j` vs `j-1`) which could affect performance but not correctness.
3. **Test return value**: Rust's `test()` returns the sorted vector (unused), while C's `test()` returns nothing.

## UB/Memory Safety Notes
**C Code Potential UB**:
- `rand() % 100` - Modulo bias exists but is minimal for 100
- `size = rand() % 500` - Could be 0, leading to zero-size allocation (handled correctly)
- No overflow checks on `(rand() % 100) - 50`

**Rust Improvements**:
- Memory safety guaranteed by Rust's ownership system
- `vec![0; size]` handles zero-size case safely
- No manual memory management needed
- `saturating_sub(1)` prevents underflow in bounds checking

## API/IO Differences
- **Exit codes**: Both return 0 on success (assert failures would exit non-zero)
- **No stdin/stdout usage**: Both are silent unless assertions fail
- **Random seeding**: Different mechanisms (C: time(NULL), Rust: SystemTime + hashing)

## Type/Overflow Considerations
**C**:
- `int` type (platform-dependent, typically 32-bit)
- Potential integer overflow in `(rand() % 100) - 50` if `rand()` returns `INT_MAX`

**Rust**:
- Explicit `i32` type (always 32-bit)
- `rand::rand()` returns `i32` directly (no modulo bias from `% 100`)
- `as usize` conversion from `unsigned_abs()` is safe for values ≤ 500

## Suggestions
1. **Fix random number generation**: Replace custom `rand` module with `rand` crate or match C's `rand()` behavior exactly:
   ```rust
   use rand::Rng;
   let mut rng = rand::thread_rng();
   let size = rng.gen_range(0..500);
   arr[i] = rng.gen_range(-50..50);
   ```

2. **Match C's algorithm indexing exactly** (optional for correctness, recommended for equivalence):
   ```rust
   let mut j = i - 1;
   while j >= 0 && key < arr[j] {
       arr[j + 1] = arr[j];
       j -= 1;
   }
   arr[j + 1] = key;
   ```

3. **Remove unused return value** from `test()` to match C's `void` return type, or document why it's kept.