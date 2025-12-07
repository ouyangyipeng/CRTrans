use std::ffi::CString;

#[derive(Copy, Clone)]
#[repr(C)]
struct Holder {
    tag: i32,
    p: *mut std::ffi::c_void,
}

fn main() {
    // First case: integer
    let mut h = Holder {
        tag: 1,
        p: std::ptr::null_mut(),
    };
    
    // Allocate and store integer
    let a = Box::new(42);
    h.p = Box::into_raw(a) as *mut std::ffi::c_void;
    
    if h.tag == 1 {
        unsafe {
            let val = *(h.p as *const i32);
            println!("int {}", val);
        }
    }
    
    // Free the integer
    unsafe {
        let _ = Box::from_raw(h.p as *mut i32);
    }
    
    // Second case: string
    h.tag = 3;
    
    // Create C string
    let s = CString::new("hello").expect("CString::new failed");
    let ptr = s.into_raw(); // Takes ownership, returns *mut c_char
    
    h.p = ptr as *mut std::ffi::c_void;
    
    if h.tag == 3 {
        unsafe {
            let c_str = std::ffi::CStr::from_ptr(h.p as *const std::ffi::c_char);
            println!("str {}", c_str.to_string_lossy());
        }
    }
    
    // Free the string
    unsafe {
        let _ = CString::from_raw(h.p as *mut std::ffi::c_char);
    }
}