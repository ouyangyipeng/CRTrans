# Equivalence Assessment Report

## Summary
**Verdict**: Functionally equivalent with minor differences in memory management patterns  
**Confidence**: 85/100  

The Rust translation correctly mimics the C code's behavior but uses safer patterns for string handling. The main divergence is in how strings are allocated and managed, though the observable output remains identical.

## Functional Equivalence
**Outputs match**: Yes
- Both programs print:
  ```
  int 42
  str hello
  ```

**Side effects match**: Mostly
- Both allocate/free memory for integer and string
- Rust uses `CString` for proper null-terminated string handling vs C's manual `strcpy`
- Rust's `into_raw()`/`from_raw()` pattern mirrors C's malloc/free ownership transfer

**Divergences**:
1. String allocation: C uses `malloc(16)` + `strcpy`, Rust uses `CString::new` (proper null termination)
2. Memory safety: Rust's approach prevents buffer overflows that could occur in C version

## UB/Memory Safety Notes
**C UB risks**:
- `strcpy(s, "hello")` is safe only because "hello" fits in 16 bytes
- No bounds checking on string operations
- Casting `void*` requires careful type discipline
- Potential use-after-free if tag mismatches pointer type

**Rust improvements**:
- `CString` ensures proper null termination
- Explicit unsafe blocks isolate dangerous operations
- Type conversions are explicit with `as` casts
- Still vulnerable to tag-pointer mismatches in unsafe blocks

**Remaining Rust risks**:
- Unsafe pointer casts could cause UB if tags don't match actual types
- `from_raw` assumes valid ownership (matches C's free assumptions)

## API/IO Differences
**stdout formatting**: Equivalent
- Both use `printf`/`println!` with same format
- Rust's `to_string_lossy()` handles invalid UTF-8 gracefully (not needed here)

**Exit codes**: Equivalent (both return 0)

**Allocation differences**:
- C: `malloc` + manual size calculation
- Rust: `Box::new` for integer, `CString::new` for string
- Both free memory before exit

## Type/Overflow Considerations
**Type safety**:
- C: Relies on programmer to match tags with casts
- Rust: Same issue in unsafe blocks, but isolated

**Overflow**:
- No arithmetic operations, so no overflow concerns
- String buffer: Rust's `CString` prevents overflow by construction

**Alignment**: Both use system allocator, so alignment should match

## Suggestions
1. **Add tag validation**: Before unsafe casts, verify tag matches expected type to prevent UB:
   ```rust
   match h.tag {
       1 => { /* int handling */ },
       3 => { /* string handling */ },
       _ => panic!("Invalid tag"),
   }
   ```

2. **Consider enum approach**: As the C comment suggests, use a proper enum instead of tag+pointer:
   ```rust
   enum Value { Int(i32), Double(f64), String(CString) }
   ```

3. **Document unsafe invariants**: Add comments about required tag-pointer relationships:
   ```rust
   // INVARIANT: when tag == 1, p must point to a valid i32
   // INVARIANT: when tag == 3, p must point to a valid null-terminated C string
   ```