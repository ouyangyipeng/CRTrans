extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn putchar(__c: core::ffi::c_int) -> core::ffi::c_int;
    fn fread(
        __ptr: *mut core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: core::ffi::c_long,
        __whence: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn strncmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const core::ffi::c_ushort;
}
pub type size_t = usize;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: core::ffi::c_int,
    pub _IO_read_ptr: *mut core::ffi::c_char,
    pub _IO_read_end: *mut core::ffi::c_char,
    pub _IO_read_base: *mut core::ffi::c_char,
    pub _IO_write_base: *mut core::ffi::c_char,
    pub _IO_write_ptr: *mut core::ffi::c_char,
    pub _IO_write_end: *mut core::ffi::c_char,
    pub _IO_buf_base: *mut core::ffi::c_char,
    pub _IO_buf_end: *mut core::ffi::c_char,
    pub _IO_save_base: *mut core::ffi::c_char,
    pub _IO_backup_base: *mut core::ffi::c_char,
    pub _IO_save_end: *mut core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: core::ffi::c_int,
    pub _flags2: core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: core::ffi::c_ushort,
    pub _vtable_offset: core::ffi::c_schar,
    pub _shortbuf: [core::ffi::c_char; 1],
    pub _lock: *mut core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: core::ffi::c_int,
    pub _unused2: [core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub const SEEK_SET: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SEEK_END: core::ffi::c_int = 2 as core::ffi::c_int;
#[no_mangle]
pub static mut s: *const core::ffi::c_char = 0 as *const core::ffi::c_char;
#[no_mangle]
pub static mut pos: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn skipws() {
    while *(*__ctype_b_loc())
        .offset(
            *s.offset(pos as isize) as core::ffi::c_uchar as core::ffi::c_int as isize,
        ) as core::ffi::c_int
        & _ISspace as core::ffi::c_int as core::ffi::c_ushort as core::ffi::c_int != 0
    {
        pos += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_indent(mut d: core::ffi::c_int) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < d {
        putchar(' ' as i32);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_string() {
    putchar('"' as i32);
    pos += 1;
    while *s.offset(pos as isize) as core::ffi::c_int != 0
        && *s.offset(pos as isize) as core::ffi::c_int != '"' as i32
    {
        if *s.offset(pos as isize) as core::ffi::c_int == '\\' as i32 {
            putchar('\\' as i32);
            pos += 1;
            if *s.offset(pos as isize) != 0 {
                putchar(*s.offset(pos as isize) as core::ffi::c_int);
            }
            pos += 1;
        } else {
            putchar(*s.offset(pos as isize) as core::ffi::c_int);
            pos += 1;
        }
    }
    if *s.offset(pos as isize) as core::ffi::c_int == '"' as i32 {
        putchar('"' as i32);
        pos += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_number() {
    print_indent(0 as core::ffi::c_int);
    let mut start: core::ffi::c_int = pos;
    if *s.offset(pos as isize) as core::ffi::c_int == '-' as i32 {
        pos += 1;
    }
    while *(*__ctype_b_loc())
        .offset(
            *s.offset(pos as isize) as core::ffi::c_uchar as core::ffi::c_int as isize,
        ) as core::ffi::c_int
        & _ISdigit as core::ffi::c_int as core::ffi::c_ushort as core::ffi::c_int != 0
    {
        pos += 1;
    }
    if *s.offset(pos as isize) as core::ffi::c_int == '.' as i32 {
        pos += 1;
        while *(*__ctype_b_loc())
            .offset(
                *s.offset(pos as isize) as core::ffi::c_uchar as core::ffi::c_int
                    as isize,
            ) as core::ffi::c_int
            & _ISdigit as core::ffi::c_int as core::ffi::c_ushort as core::ffi::c_int
            != 0
        {
            pos += 1;
        }
    }
    let mut i: core::ffi::c_int = start;
    while i < pos {
        putchar(*s.offset(i as isize) as core::ffi::c_int);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_array(mut indent: core::ffi::c_int) {
    printf(b"[\n\0" as *const u8 as *const core::ffi::c_char);
    pos += 1;
    skipws();
    let mut first: core::ffi::c_int = 1 as core::ffi::c_int;
    while *s.offset(pos as isize) as core::ffi::c_int != 0
        && *s.offset(pos as isize) as core::ffi::c_int != ']' as i32
    {
        if first == 0 {
            printf(b",\n\0" as *const u8 as *const core::ffi::c_char);
        }
        first = 0 as core::ffi::c_int;
        print_indent(indent + 2 as core::ffi::c_int);
        parse_value(indent + 2 as core::ffi::c_int);
        skipws();
        if *s.offset(pos as isize) as core::ffi::c_int == ',' as i32 {
            pos += 1;
            skipws();
        }
    }
    printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
    print_indent(indent);
    printf(b"]\0" as *const u8 as *const core::ffi::c_char);
    if *s.offset(pos as isize) as core::ffi::c_int == ']' as i32 {
        pos += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_object(mut indent: core::ffi::c_int) {
    printf(b"{\n\0" as *const u8 as *const core::ffi::c_char);
    pos += 1;
    skipws();
    let mut first: core::ffi::c_int = 1 as core::ffi::c_int;
    while *s.offset(pos as isize) as core::ffi::c_int != 0
        && *s.offset(pos as isize) as core::ffi::c_int != '}' as i32
    {
        if first == 0 {
            printf(b",\n\0" as *const u8 as *const core::ffi::c_char);
        }
        first = 0 as core::ffi::c_int;
        print_indent(indent + 2 as core::ffi::c_int);
        if *s.offset(pos as isize) as core::ffi::c_int == '"' as i32 {
            parse_string();
        } else {
            printf(b"ERROR_KEY\0" as *const u8 as *const core::ffi::c_char);
        }
        skipws();
        if *s.offset(pos as isize) as core::ffi::c_int == ':' as i32 {
            pos += 1;
        }
        skipws();
        printf(b": \0" as *const u8 as *const core::ffi::c_char);
        parse_value(indent + 2 as core::ffi::c_int);
        skipws();
        if *s.offset(pos as isize) as core::ffi::c_int == ',' as i32 {
            pos += 1;
            skipws();
        }
    }
    printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
    print_indent(indent);
    printf(b"}\0" as *const u8 as *const core::ffi::c_char);
    if *s.offset(pos as isize) as core::ffi::c_int == '}' as i32 {
        pos += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_value(mut indent: core::ffi::c_int) {
    skipws();
    if *s.offset(pos as isize) as core::ffi::c_int == '"' as i32 {
        parse_string();
    } else if *s.offset(pos as isize) as core::ffi::c_int == '{' as i32 {
        parse_object(indent);
    } else if *s.offset(pos as isize) as core::ffi::c_int == '[' as i32 {
        parse_array(indent);
    } else if *(*__ctype_b_loc())
        .offset(
            *s.offset(pos as isize) as core::ffi::c_uchar as core::ffi::c_int as isize,
        ) as core::ffi::c_int
        & _ISdigit as core::ffi::c_int as core::ffi::c_ushort as core::ffi::c_int != 0
        || *s.offset(pos as isize) as core::ffi::c_int == '-' as i32
    {
        parse_number();
    } else if strncmp(
        s.offset(pos as isize),
        b"true\0" as *const u8 as *const core::ffi::c_char,
        4 as size_t,
    ) == 0 as core::ffi::c_int
    {
        printf(b"true\0" as *const u8 as *const core::ffi::c_char);
        pos += 4 as core::ffi::c_int;
    } else if strncmp(
        s.offset(pos as isize),
        b"false\0" as *const u8 as *const core::ffi::c_char,
        5 as size_t,
    ) == 0 as core::ffi::c_int
    {
        printf(b"false\0" as *const u8 as *const core::ffi::c_char);
        pos += 5 as core::ffi::c_int;
    } else if strncmp(
        s.offset(pos as isize),
        b"null\0" as *const u8 as *const core::ffi::c_char,
        4 as size_t,
    ) == 0 as core::ffi::c_int
    {
        printf(b"null\0" as *const u8 as *const core::ffi::c_char);
        pos += 4 as core::ffi::c_int;
    } else {
        printf(b"null\0" as *const u8 as *const core::ffi::c_char);
        pos += 1;
    };
}
unsafe fn main_0() -> core::ffi::c_int {
    fseek(stdin, 0 as core::ffi::c_long, SEEK_END);
    let mut sz: core::ffi::c_long = ftell(stdin);
    fseek(stdin, 0 as core::ffi::c_long, SEEK_SET);
    let mut buf: *mut core::ffi::c_char = malloc((sz + 1 as core::ffi::c_long) as size_t)
        as *mut core::ffi::c_char;
    if buf.is_null() {
        return 1 as core::ffi::c_int;
    }
    fread(buf as *mut core::ffi::c_void, 1 as size_t, sz as size_t, stdin);
    *buf.offset(sz as isize) = 0 as core::ffi::c_char;
    s = buf;
    pos = 0 as core::ffi::c_int;
    parse_value(0 as core::ffi::c_int);
    putchar('\n' as i32);
    return 0 as core::ffi::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
