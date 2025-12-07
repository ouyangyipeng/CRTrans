extern "C" {
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn age(
    mut present_date: core::ffi::c_int,
    mut present_month: core::ffi::c_int,
    mut present_year: core::ffi::c_int,
    mut birth_date: core::ffi::c_int,
    mut birth_month: core::ffi::c_int,
    mut birth_year: core::ffi::c_int,
) {
    let mut month: [core::ffi::c_int; 12] = [
        31 as core::ffi::c_int,
        28 as core::ffi::c_int,
        31 as core::ffi::c_int,
        30 as core::ffi::c_int,
        31 as core::ffi::c_int,
        30 as core::ffi::c_int,
        31 as core::ffi::c_int,
        31 as core::ffi::c_int,
        30 as core::ffi::c_int,
        31 as core::ffi::c_int,
        30 as core::ffi::c_int,
        31 as core::ffi::c_int,
    ];
    if birth_date > present_date {
        present_date = present_date
            + month[(birth_month - 1 as core::ffi::c_int) as usize];
        present_month = present_month - 1 as core::ffi::c_int;
    }
    if birth_month > present_month {
        present_year = present_year - 1 as core::ffi::c_int;
        present_month = present_month + 12 as core::ffi::c_int;
    }
    let mut final_date: core::ffi::c_int = present_date - birth_date;
    let mut final_month: core::ffi::c_int = present_month - birth_month;
    let mut final_year: core::ffi::c_int = present_year - birth_year;
    printf(
        b"Present Age Years: %d Months: %d Days: %d\0" as *const u8
            as *const core::ffi::c_char,
        final_year,
        final_month,
        final_date,
    );
}
unsafe fn main_0() -> core::ffi::c_int {
    let mut present_date: core::ffi::c_int = 21 as core::ffi::c_int;
    let mut present_month: core::ffi::c_int = 9 as core::ffi::c_int;
    let mut present_year: core::ffi::c_int = 2019 as core::ffi::c_int;
    let mut birth_date: core::ffi::c_int = 25 as core::ffi::c_int;
    let mut birth_month: core::ffi::c_int = 9 as core::ffi::c_int;
    let mut birth_year: core::ffi::c_int = 1996 as core::ffi::c_int;
    age(present_date, present_month, present_year, birth_date, birth_month, birth_year);
    return 0 as core::ffi::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
