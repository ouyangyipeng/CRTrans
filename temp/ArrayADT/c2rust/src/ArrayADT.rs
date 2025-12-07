extern "C" {
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn scanf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct myArray {
    pub total_size: core::ffi::c_int,
    pub used_size: core::ffi::c_int,
    pub ptr: *mut core::ffi::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn CreateArray(
    mut a: *mut myArray,
    mut tSize: core::ffi::c_int,
    mut uSize: core::ffi::c_int,
) {
    (*a).total_size = tSize;
    (*a).used_size = uSize;
    (*a).ptr = malloc(
        (tSize as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
    ) as *mut core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn show(mut a: *mut myArray) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*a).used_size {
        printf(
            b"%d\n\0" as *const u8 as *const core::ffi::c_char,
            *((*a).ptr).offset(i as isize),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn setVal(mut a: *mut myArray) {
    let mut n: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*a).used_size {
        printf(b"Enter %d element:\0" as *const u8 as *const core::ffi::c_char, i);
        scanf(
            b"%d\0" as *const u8 as *const core::ffi::c_char,
            &mut n as *mut core::ffi::c_int,
        );
        *((*a).ptr).offset(i as isize) = n;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn set(mut a: *mut myArray, mut index: core::ffi::c_int) {
    *((*a).ptr).offset(index as isize) = 32 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get(mut a: *mut myArray, mut n: core::ffi::c_int) {
    printf(
        b"The value in the array in this index is %d\n\0" as *const u8
            as *const core::ffi::c_char,
        *((*a).ptr).offset(n as isize),
    );
}
unsafe fn main_0() -> core::ffi::c_int {
    let mut marks: myArray = myArray {
        total_size: 0,
        used_size: 0,
        ptr: 0 as *mut core::ffi::c_int,
    };
    CreateArray(&mut marks, 10 as core::ffi::c_int, 2 as core::ffi::c_int);
    printf(b"We are running setVal now\n\0" as *const u8 as *const core::ffi::c_char);
    setVal(&mut marks);
    printf(b"We are running show now\n\0" as *const u8 as *const core::ffi::c_char);
    show(&mut marks);
    set(&mut marks, 0 as core::ffi::c_int);
    printf(b"We are running show now\n\0" as *const u8 as *const core::ffi::c_char);
    show(&mut marks);
    get(&mut marks, 0 as core::ffi::c_int);
    return 0 as core::ffi::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
