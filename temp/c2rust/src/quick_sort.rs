extern "C" {
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn scanf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn getchar() -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
}
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn display(
    mut arr: *mut core::ffi::c_int,
    mut n: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < n {
        printf(
            b"%d \0" as *const u8 as *const core::ffi::c_char,
            *arr.offset(i as isize),
        );
        i += 1;
    }
    printf(b"\n\0" as *const u8 as *const core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn swap(
    mut first: *mut core::ffi::c_int,
    mut second: *mut core::ffi::c_int,
) {
    let mut temp: core::ffi::c_int = *first;
    *first = *second;
    *second = temp;
}
#[no_mangle]
pub unsafe extern "C" fn partition(
    mut arr: *mut core::ffi::c_int,
    mut lower: core::ffi::c_int,
    mut upper: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = lower - 1 as core::ffi::c_int;
    let mut pivot: core::ffi::c_int = *arr.offset(upper as isize);
    let mut j: core::ffi::c_int = 0;
    j = lower;
    while j < upper {
        if *arr.offset(j as isize) <= pivot {
            i += 1;
            swap(&mut *arr.offset(i as isize), &mut *arr.offset(j as isize));
        }
        j += 1;
    }
    swap(
        &mut *arr.offset((i + 1 as core::ffi::c_int) as isize),
        &mut *arr.offset(upper as isize),
    );
    return i + 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quickSort(
    mut arr: *mut core::ffi::c_int,
    mut lower: core::ffi::c_int,
    mut upper: core::ffi::c_int,
) {
    if upper > lower {
        let mut partitionIndex: core::ffi::c_int = partition(arr, lower, upper);
        quickSort(arr, lower, partitionIndex - 1 as core::ffi::c_int);
        quickSort(arr, partitionIndex + 1 as core::ffi::c_int, upper);
    }
}
unsafe fn main_0() -> core::ffi::c_int {
    let mut n: core::ffi::c_int = 0;
    printf(b"Enter size of array:\n\0" as *const u8 as *const core::ffi::c_char);
    scanf(
        b"%d\0" as *const u8 as *const core::ffi::c_char,
        &mut n as *mut core::ffi::c_int,
    );
    printf(
        b"Enter the elements of the array\n\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut i: core::ffi::c_int = 0;
    let mut arr: *mut core::ffi::c_int = malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t).wrapping_mul(n as size_t),
    ) as *mut core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < n {
        scanf(
            b"%d\0" as *const u8 as *const core::ffi::c_char,
            &mut *arr.offset(i as isize) as *mut core::ffi::c_int,
        );
        i += 1;
    }
    printf(b"Original array: \0" as *const u8 as *const core::ffi::c_char);
    display(arr as *mut core::ffi::c_int, n);
    quickSort(
        arr as *mut core::ffi::c_int,
        0 as core::ffi::c_int,
        n - 1 as core::ffi::c_int,
    );
    printf(b"Sorted array: \0" as *const u8 as *const core::ffi::c_char);
    display(arr as *mut core::ffi::c_int, n);
    getchar();
    free(arr as *mut core::ffi::c_void);
    return 0 as core::ffi::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
