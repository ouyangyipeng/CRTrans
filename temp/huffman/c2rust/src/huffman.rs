extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn fgetc(__stream: *mut FILE) -> core::ffi::c_int;
    fn fputs(__s: *const core::ffi::c_char, __stream: *mut FILE) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn strdup(__s: *const core::ffi::c_char) -> *mut core::ffi::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub sym: core::ffi::c_int,
    pub freq: core::ffi::c_ulong,
    pub l: *mut Node,
    pub r: *mut Node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Heap {
    pub a: *mut *mut Node,
    pub n: core::ffi::c_int,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const EOF: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const MAXSYM: core::ffi::c_int = 256 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn heap_new(mut cap: core::ffi::c_int) -> *mut Heap {
    let mut h: *mut Heap = malloc(::core::mem::size_of::<Heap>() as size_t) as *mut Heap;
    (*h).a = malloc(
        (cap as size_t).wrapping_mul(::core::mem::size_of::<*mut Node>() as size_t),
    ) as *mut *mut Node;
    (*h).n = 0 as core::ffi::c_int;
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn heap_push(mut h: *mut Heap, mut x: *mut Node) {
    let fresh0 = (*h).n;
    (*h).n = (*h).n + 1;
    let mut i: core::ffi::c_int = fresh0;
    let ref mut fresh1 = *((*h).a).offset(i as isize);
    *fresh1 = x;
    while i != 0 {
        let mut p: core::ffi::c_int = (i - 1 as core::ffi::c_int)
            / 2 as core::ffi::c_int;
        if (**((*h).a).offset(p as isize)).freq <= (**((*h).a).offset(i as isize)).freq {
            break;
        }
        let mut t: *mut Node = *((*h).a).offset(p as isize);
        let ref mut fresh2 = *((*h).a).offset(p as isize);
        *fresh2 = *((*h).a).offset(i as isize);
        let ref mut fresh3 = *((*h).a).offset(i as isize);
        *fresh3 = t;
        i = p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heap_pop(mut h: *mut Heap) -> *mut Node {
    if (*h).n == 0 as core::ffi::c_int {
        return 0 as *mut Node;
    }
    let mut ret: *mut Node = *((*h).a).offset(0 as core::ffi::c_int as isize);
    (*h).n -= 1;
    let ref mut fresh4 = *((*h).a).offset(0 as core::ffi::c_int as isize);
    *fresh4 = *((*h).a).offset((*h).n as isize);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        let mut l: core::ffi::c_int = 2 as core::ffi::c_int * i + 1 as core::ffi::c_int;
        let mut r: core::ffi::c_int = 2 as core::ffi::c_int * i + 2 as core::ffi::c_int;
        let mut smallest: core::ffi::c_int = i;
        if l < (*h).n
            && (**((*h).a).offset(l as isize)).freq
                < (**((*h).a).offset(smallest as isize)).freq
        {
            smallest = l;
        }
        if r < (*h).n
            && (**((*h).a).offset(r as isize)).freq
                < (**((*h).a).offset(smallest as isize)).freq
        {
            smallest = r;
        }
        if smallest == i {
            break;
        }
        let mut t: *mut Node = *((*h).a).offset(i as isize);
        let ref mut fresh5 = *((*h).a).offset(i as isize);
        *fresh5 = *((*h).a).offset(smallest as isize);
        let ref mut fresh6 = *((*h).a).offset(smallest as isize);
        *fresh6 = t;
        i = smallest;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn node_new(
    mut sym: core::ffi::c_int,
    mut freq: core::ffi::c_ulong,
) -> *mut Node {
    let mut n: *mut Node = malloc(::core::mem::size_of::<Node>() as size_t) as *mut Node;
    (*n).sym = sym;
    (*n).freq = freq;
    (*n).r = 0 as *mut Node;
    (*n).l = (*n).r;
    return n;
}
#[no_mangle]
pub static mut codes: [*mut core::ffi::c_char; 256] = [0 as *const core::ffi::c_char
    as *mut core::ffi::c_char; 256];
#[no_mangle]
pub unsafe extern "C" fn gen_codes(
    mut root: *mut Node,
    mut buf: *mut core::ffi::c_char,
    mut depth: core::ffi::c_int,
) {
    if root.is_null() {
        return;
    }
    if ((*root).l).is_null() && ((*root).r).is_null() {
        *buf.offset(depth as isize) = 0 as core::ffi::c_char;
        codes[(*root).sym as usize] = strdup(buf);
        return;
    }
    if !((*root).l).is_null() {
        *buf.offset(depth as isize) = '0' as i32 as core::ffi::c_char;
        gen_codes((*root).l as *mut Node, buf, depth + 1 as core::ffi::c_int);
    }
    if !((*root).r).is_null() {
        *buf.offset(depth as isize) = '1' as i32 as core::ffi::c_char;
        gen_codes((*root).r as *mut Node, buf, depth + 1 as core::ffi::c_int);
    }
}
unsafe fn main_0() -> core::ffi::c_int {
    let mut freq: [core::ffi::c_ulong; 256] = [
        0 as core::ffi::c_int as core::ffi::c_ulong,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut c: core::ffi::c_int = 0;
    let mut total: core::ffi::c_ulong = 0 as core::ffi::c_ulong;
    loop {
        c = fgetc(stdin);
        if !(c != EOF) {
            break;
        }
        freq[c as core::ffi::c_uchar as usize] = (freq[c as core::ffi::c_uchar as usize])
            .wrapping_add(1);
        total = total.wrapping_add(1);
    }
    if total == 0 as core::ffi::c_ulong {
        fprintf(stderr, b"no input\n\0" as *const u8 as *const core::ffi::c_char);
        return 0 as core::ffi::c_int;
    }
    let mut h: *mut Heap = heap_new(MAXSYM);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < MAXSYM {
        if freq[i as usize] != 0 {
            heap_push(h, node_new(i, freq[i as usize]));
        }
        i += 1;
    }
    if (*h).n == 1 as core::ffi::c_int {
        let mut leaf: *mut Node = heap_pop(h);
        codes[(*leaf).sym as usize] = strdup(
            b"0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {
        while (*h).n > 1 as core::ffi::c_int {
            let mut a: *mut Node = heap_pop(h);
            let mut b: *mut Node = heap_pop(h);
            let mut p: *mut Node = node_new(
                -(1 as core::ffi::c_int),
                ((*a).freq).wrapping_add((*b).freq),
            );
            (*p).l = a as *mut Node;
            (*p).r = b as *mut Node;
            heap_push(h, p);
        }
        let mut root: *mut Node = heap_pop(h);
        let mut buf: [core::ffi::c_char; 512] = [0; 512];
        gen_codes(root, buf.as_mut_ptr(), 0 as core::ffi::c_int);
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < MAXSYM {
        if !(codes[i_0 as usize]).is_null() {
            if i_0 >= 32 as core::ffi::c_int && i_0 < 127 as core::ffi::c_int {
                printf(
                    b"'%c' : %s\n\0" as *const u8 as *const core::ffi::c_char,
                    i_0 as core::ffi::c_char as core::ffi::c_int,
                    codes[i_0 as usize],
                );
            } else {
                printf(
                    b"%02x : %s\n\0" as *const u8 as *const core::ffi::c_char,
                    i_0,
                    codes[i_0 as usize],
                );
            }
        }
        i_0 += 1;
    }
    let mut f: *mut FILE = fopen(
        b"/proc/self/fd/0\0" as *const u8 as *const core::ffi::c_char,
        b"r\0" as *const u8 as *const core::ffi::c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"cannot reopen stdin for encoding; codes created\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as core::ffi::c_int;
    }
    loop {
        c = fgetc(f);
        if !(c != EOF) {
            break;
        }
        fputs(codes[c as core::ffi::c_uchar as usize], stdout);
    }
    fclose(f);
    return 0 as core::ffi::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
