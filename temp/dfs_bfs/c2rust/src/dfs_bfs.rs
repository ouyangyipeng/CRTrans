extern "C" {
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn scanf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}
pub const n: core::ffi::c_int = 10 as core::ffi::c_int;
pub const max: core::ffi::c_int = 10 as core::ffi::c_int;
#[no_mangle]
pub static mut top: core::ffi::c_int = -(1 as core::ffi::c_int);
#[no_mangle]
pub static mut front: core::ffi::c_int = -(1 as core::ffi::c_int);
#[no_mangle]
pub static mut rear: core::ffi::c_int = -(1 as core::ffi::c_int);
#[no_mangle]
pub static mut cq: [core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut adj: [[core::ffi::c_int; 10]; 10] = [[0; 10]; 10];
#[no_mangle]
pub static mut stack: [core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut visited1: [core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut visited2: [core::ffi::c_int; 10] = [0; 10];
#[no_mangle]
pub unsafe extern "C" fn push(mut val: core::ffi::c_int) {
    if top == max - 1 as core::ffi::c_int {
        printf(b"\n overflow\0" as *const u8 as *const core::ffi::c_char);
    } else {
        top += 1;
        stack[top as usize] = val;
    };
}
#[no_mangle]
pub unsafe extern "C" fn pop() -> core::ffi::c_int {
    if top == -(1 as core::ffi::c_int) {
        printf(b"\nunderflow\0" as *const u8 as *const core::ffi::c_char);
        return -(1 as core::ffi::c_int);
    } else {
        let fresh0 = top;
        top = top - 1;
        return stack[fresh0 as usize];
    };
}
#[no_mangle]
pub unsafe extern "C" fn add(mut val: core::ffi::c_int) {
    if rear == max - 1 as core::ffi::c_int && front == 0 as core::ffi::c_int {
        printf(b"\noverflow\0" as *const u8 as *const core::ffi::c_char);
    } else if rear == -(1 as core::ffi::c_int) {
        rear = 0 as core::ffi::c_int;
        front = 0 as core::ffi::c_int;
    } else if rear == max - 1 as core::ffi::c_int {
        rear = 0 as core::ffi::c_int;
    } else {
        rear += 1;
    }
    cq[rear as usize] = val;
}
#[no_mangle]
pub unsafe extern "C" fn delete() -> core::ffi::c_int {
    let mut val: core::ffi::c_int = 0;
    if rear == -(1 as core::ffi::c_int) {
        printf(b"\nempty\0" as *const u8 as *const core::ffi::c_char);
    } else if front == max - 1 as core::ffi::c_int {
        front = 0 as core::ffi::c_int;
    } else if front == rear {
        rear = -(1 as core::ffi::c_int);
        front = rear;
    } else {
        front += 1;
    }
    return cq[front as usize];
}
#[no_mangle]
pub unsafe extern "C" fn dfs(
    mut visited2_0: *mut core::ffi::c_int,
    mut start: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < max {
        *visited2_0.offset(i as isize) = 0 as core::ffi::c_int;
        i += 1;
    }
    push(start);
    printf(b"\t%d\0" as *const u8 as *const core::ffi::c_char, start);
    *visited2_0.offset(start as isize) = 1 as core::ffi::c_int;
    while top != -(1 as core::ffi::c_int) {
        start = pop();
        i = 0 as core::ffi::c_int;
        while i < max {
            if adj[start as usize][i as usize] == 1 as core::ffi::c_int
                && *visited2_0.offset(i as isize) == 0 as core::ffi::c_int
            {
                push(i);
                printf(b"\t%d\0" as *const u8 as *const core::ffi::c_char, i);
                *visited2_0.offset(i as isize) = 1 as core::ffi::c_int;
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn bfs(
    mut visited1_0: *mut core::ffi::c_int,
    mut start: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut u: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < max {
        *visited1_0.offset(i as isize) = 0 as core::ffi::c_int;
        i += 1;
    }
    add(start);
    printf(b"\t%d\0" as *const u8 as *const core::ffi::c_char, start);
    *visited1_0.offset(start as isize) = 1 as core::ffi::c_int;
    *visited1_0.offset(0 as core::ffi::c_int as isize) = 1 as core::ffi::c_int;
    while front != -(1 as core::ffi::c_int) {
        i = 0 as core::ffi::c_int;
        while i < max {
            if adj[start as usize][i as usize] == 1 as core::ffi::c_int
                && *visited1_0.offset(i as isize) == 0 as core::ffi::c_int
            {
                add(i);
            }
            i += 1;
        }
        start = delete();
        if *visited1_0.offset(start as isize) == 0 as core::ffi::c_int {
            printf(b"\t%d\0" as *const u8 as *const core::ffi::c_char, start);
            *visited1_0.offset(start as isize) = 1 as core::ffi::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn initialise() {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < n {
        j = 0 as core::ffi::c_int;
        while j < n {
            adj[i as usize][j as usize] = 0 as core::ffi::c_int;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn addedge(mut i: core::ffi::c_int, mut j: core::ffi::c_int) {
    adj[i as usize][j as usize] = 1 as core::ffi::c_int;
    adj[j as usize][i as usize] = 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn removeedge(mut i: core::ffi::c_int, mut j: core::ffi::c_int) {
    adj[i as usize][j as usize] = 0 as core::ffi::c_int;
    adj[j as usize][i as usize] = 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn adjacent(
    mut i: core::ffi::c_int,
    mut j: core::ffi::c_int,
) -> core::ffi::c_int {
    return adj[i as usize][j as usize];
}
unsafe fn main_0() {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut a: core::ffi::c_int = 0;
    let mut b: core::ffi::c_int = 0;
    let mut start: core::ffi::c_int = 0;
    let mut opt: core::ffi::c_int = 0;
    initialise();
    loop {
        printf(
            b"\n 1=create edge \n 2= delete edge \n 3= bfs \n 4=dfs \n enter choice \n\0"
                as *const u8 as *const core::ffi::c_char,
        );
        scanf(
            b"%d\0" as *const u8 as *const core::ffi::c_char,
            &mut opt as *mut core::ffi::c_int,
        );
        match opt {
            1 => {
                printf(
                    b"\n Enter two numbers to create edge\0" as *const u8
                        as *const core::ffi::c_char,
                );
                scanf(
                    b"%d %d\0" as *const u8 as *const core::ffi::c_char,
                    &mut i as *mut core::ffi::c_int,
                    &mut j as *mut core::ffi::c_int,
                );
                addedge(i, j);
            }
            2 => {
                printf(
                    b"\nenter two numbers to delete edge\0" as *const u8
                        as *const core::ffi::c_char,
                );
                scanf(
                    b"%d %d\0" as *const u8 as *const core::ffi::c_char,
                    &mut i as *mut core::ffi::c_int,
                    &mut j as *mut core::ffi::c_int,
                );
                removeedge(i, j);
            }
            3 => {
                printf(
                    b"\nenter the start for bfs \0" as *const u8
                        as *const core::ffi::c_char,
                );
                scanf(
                    b"%d\0" as *const u8 as *const core::ffi::c_char,
                    &mut start as *mut core::ffi::c_int,
                );
                bfs(visited1.as_mut_ptr(), start);
            }
            4 => {
                printf(
                    b"\nenter start for dfs\0" as *const u8 as *const core::ffi::c_char,
                );
                scanf(
                    b"%d\0" as *const u8 as *const core::ffi::c_char,
                    &mut start as *mut core::ffi::c_int,
                );
                dfs(visited2.as_mut_ptr(), start);
            }
            _ => {
                printf(b"Invalid input\0" as *const u8 as *const core::ffi::c_char);
            }
        }
        printf(b"Continue? \n 1=yes \n 0=no\0" as *const u8 as *const core::ffi::c_char);
        scanf(
            b"%d\0" as *const u8 as *const core::ffi::c_char,
            &mut opt as *mut core::ffi::c_int,
        );
        if !(opt == 1 as core::ffi::c_int) {
            break;
        }
    };
}
pub fn main() {
    unsafe { main_0() }
    ::std::process::exit(0i32);
}
