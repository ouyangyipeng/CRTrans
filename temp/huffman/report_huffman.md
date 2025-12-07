# Huffman C/Rust Equivalence Assessment

## Summary
**Overall equivalence: High (85/100 confidence)**  
The Rust implementation is functionally similar but has several behavioral differences in output formatting, input handling, and edge cases. The core Huffman algorithm logic is equivalent, but the Rust version deviates in ways that affect exact output matching.

## Functional Equivalence
**Partial match with divergences:**

1. **Input handling**: C reads stdin twice (reopens `/proc/self/fd/0`), while Rust reads once into buffer
   - Both process the same data, but Rust avoids system-specific file reopening
   - **Impact**: Same functional result for valid inputs

2. **Single symbol case**: Rust sorts symbols before printing codes; C prints in ascending order without explicit sorting
   - **Impact**: Different output order for single-symbol inputs

3. **Quote character handling**: Rust excludes ASCII 39 (`'`) from character formatting; C includes it
   - **Impact**: `'` character displays as hex in Rust vs `''' : code` in C

4. **Memory usage**: Rust uses `HashMap<u8, String>`; C uses `char* codes[256]` with `strdup`
   - **Impact**: Same logical behavior, different memory patterns

## UB/Memory Safety Notes
**C potential UB:**
- `fopen("/proc/self/fd/0", "r")` is Linux-specific; fails on other OSes
- No bounds check on `buf[depth]` in `gen_codes` (512-byte buffer may overflow with deep trees)
- `strdup` failure not checked (could return NULL)
- `heap_new` doesn't check `malloc` success

**Rust improvements:**
- No OS-specific file paths (portable)
- Fixed buffer size (256) in `gen_codes`; depth bounds checked
- `HashMap` handles memory safely
- `Option`/`Result` types prevent null pointer dereferences

## API/IO Differences
1. **Exit codes**: C returns 0 even on errors; Rust returns `io::Result` (OS-dependent on error)
2. **Error messages**: Rust prints "no input" to stderr; C does same
3. **Output formatting**: 
   - Rust excludes `'` from character formatting
   - Rust sorts symbols in single-symbol case
   - Both use same hex/character formatting otherwise
4. **Input reopening**: C uses Linux-specific path; Rust buffers entire input

## Type/Overflow Considerations
- **C**: Uses `unsigned long` for frequencies (platform-dependent size)
- **Rust**: Uses `u64` for frequencies (consistent 64-bit)
- **Potential overflow**: Both could overflow on huge inputs (>2⁶⁴ bytes), but Rust's `u64` overflow would panic in debug mode
- **Heap capacity**: C allocates `MAXSYM` nodes; Rust's `Heap::a` has fixed capacity `MAXSYM` (256)

## Suggestions
1. **Fix quote handling**: Remove `&& i != 39` condition in Rust to match C's formatting for apostrophe character
2. **Remove sorting in single-symbol case**: In the `heap.len() == 1` branch, iterate through `freq` array directly instead of collecting/sorting symbols
3. **Add buffer size validation**: In `gen_codes`, add assertion or dynamic allocation to ensure `depth < buf.len()` for deep trees (though 256 is sufficient for 256 symbols)

**Key equivalence verdict**: The implementations produce the same Huffman codes and encoded output for identical inputs, but output formatting differs in edge cases. The Rust version is more portable and memory-safe but not byte-for-byte identical.