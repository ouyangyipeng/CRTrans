# C/Rust Equivalence Assessment: JSON Parser

## Summary
**Overall Verdict**: Functionally equivalent with minor differences in error handling and memory safety.  
**Confidence**: 92/100

## Functional Equivalence
The Rust implementation produces identical output for valid JSON inputs and handles most edge cases similarly. Both parsers:
- Print JSON with indentation (objects/arrays on separate lines)
- Handle strings, numbers, booleans, null, objects, and arrays
- Skip whitespace between tokens
- Print raw number substrings without validation

**Minor Divergences**:
1. **Empty input**: C prints nothing (pos=0, s[0]=0, parse_value prints "null" then increments pos), Rust prints "null" (explicit check for POS >= len)
2. **Invalid key handling**: Both print "ERROR_KEY" for non-string keys, but Rust checks bounds before accessing
3. **Trailing garbage**: C continues parsing (pos++ in else branch), Rust increments pos once and prints "null"

## UB/Memory Safety Notes
**C Code UB Risks**:
- `fread` return value ignored (possible partial read)
- `ftell`/`fseek` on stdin may fail (not a seekable stream)
- No bounds checking before `s[pos]` accesses (potential buffer overread)
- `isspace((unsigned char)s[pos])` with negative `char` values (cast mitigates)
- `strncmp` reads up to 4/5 bytes without length verification

**Rust Improvements**:
- All buffer accesses bounds-checked
- No global mutable state in safe code (though `unsafe` used)
- `Read::read_to_string` handles stdin properly
- ASCII checks use safe methods (`is_ascii_whitespace`, `is_ascii_digit`)

**Rust Safety Issue**: The `unsafe` block and global statics are unnecessary. The parser could be written entirely in safe Rust by passing state as parameters.

## API/IO Differences
- **Input reading**: C uses `fseek`/`ftell` (requires seekable stdin), Rust uses `read_to_string` (works with pipes/files)
- **Exit codes**: Both return 0 on success, C returns 1 on malloc failure
- **Output formatting**: Identical whitespace and newline placement
- **Character encoding**: Both assume ASCII/UTF-8 input; Rust prints bytes as chars (may panic on invalid UTF-8 in string literals)

## Type/Overflow Considerations
- **C**: `pos` as `int` may overflow on large inputs (>INT_MAX bytes)
- **Rust**: `POS` as `usize` safe for all allocatable sizes
- **Indent**: Both use `i32` for indent depth (C's `int`, Rust's `i32`)
- **Number parsing**: Both copy substrings without conversion, avoiding overflow

## Suggestions
1. **Remove unsafe**: Rewrite without global statics - pass `&[u8]` and `&mut usize` as parameters to avoid `unsafe` blocks.
2. **Fix empty input**: Make Rust match C's behavior by moving the length check to after `skipws_global()` in `parse_value`.
3. **Improve error handling**: Both should handle read errors (C's `fread`, Rust's `read_to_string`) and invalid UTF-8 in Rust's string conversion.