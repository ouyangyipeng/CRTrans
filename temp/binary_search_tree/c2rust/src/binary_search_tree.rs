extern "C" {
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn scanf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tnode {
    pub data: core::ffi::c_int,
    pub left: *mut tnode,
    pub right: *mut tnode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree {
    pub root: *mut tnode,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub static mut lcount: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn insert(mut q: *mut tree, mut x: core::ffi::c_int) {
    let mut p: *mut tnode = 0 as *mut tnode;
    let mut prev: *mut tnode = 0 as *mut tnode;
    prev = 0 as *mut tnode;
    p = malloc(::core::mem::size_of::<tnode>() as size_t) as *mut tnode;
    (*p).data = x;
    (*p).right = 0 as *mut tnode;
    (*p).left = (*p).right;
    if ((*q).root).is_null() {
        (*q).root = p;
    } else {
        let mut l: *mut tnode = (*q).root;
        while !l.is_null() {
            prev = l;
            if (*p).data < (*l).data {
                l = (*l).left;
            } else {
                l = (*l).right;
            }
        }
        if (*p).data >= (*prev).data {
            (*prev).right = p;
        } else {
            (*prev).left = p;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn deletenode(mut q: *mut tree, mut x: core::ffi::c_int) {
    let mut p: *mut tnode = 0 as *mut tnode;
    let mut rp: *mut tnode = 0 as *mut tnode;
    let mut prev: *mut tnode = 0 as *mut tnode;
    let mut f: *mut tnode = 0 as *mut tnode;
    let mut s: *mut tnode = 0 as *mut tnode;
    prev = 0 as *mut tnode;
    p = (*q).root;
    while !p.is_null() {
        if (*p).data == x {
            break;
        }
        prev = p;
        if x < (*p).data {
            p = (*p).left;
        } else {
            p = (*p).right;
        }
    }
    if p.is_null() {
        printf(b"Element Not Found!\n\0" as *const u8 as *const core::ffi::c_char);
        return;
    }
    if ((*p).left).is_null() && ((*p).right).is_null() {
        rp = 0 as *mut tnode;
    } else if ((*p).left).is_null() {
        rp = (*p).right;
    } else if ((*p).right).is_null() {
        rp = (*p).left;
    } else {
        rp = (*p).right;
        s = (*rp).left;
        if s.is_null() {
            (*rp).left = (*p).left;
        } else {
            while !s.is_null() {
                f = rp;
                rp = s;
                s = (*s).left;
            }
            (*f).left = (*rp).right;
            (*rp).left = (*p).left;
            (*rp).right = (*p).right;
        }
    }
    if prev.is_null() {
        (*q).root = rp;
    } else if (*prev).left == p {
        (*prev).left = rp;
    } else {
        (*prev).right = rp;
    };
}
#[no_mangle]
pub unsafe extern "C" fn disp_inorder(mut q: *mut tnode) {
    if !q.is_null() {
        disp_inorder((*q).left);
        printf(b"%d\t\0" as *const u8 as *const core::ffi::c_char, (*q).data);
        disp_inorder((*q).right);
    }
}
#[no_mangle]
pub unsafe extern "C" fn disp_preorder(mut q: *mut tnode) {
    if !q.is_null() {
        printf(b"%d\t\0" as *const u8 as *const core::ffi::c_char, (*q).data);
        disp_inorder((*q).left);
        disp_inorder((*q).right);
    }
}
#[no_mangle]
pub unsafe extern "C" fn disp_postorder(mut q: *mut tnode) {
    if !q.is_null() {
        disp_inorder((*q).left);
        disp_inorder((*q).right);
        printf(b"%d\0" as *const u8 as *const core::ffi::c_char, (*q).data);
    }
}
#[no_mangle]
pub unsafe extern "C" fn leafcount(mut p: *mut tnode) -> core::ffi::c_int {
    if !p.is_null() {
        if ((*p).left).is_null() && ((*p).right).is_null() {
            lcount += 1;
        } else {
            leafcount((*p).left);
            leafcount((*p).right);
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn count_interiornode(mut root: *mut tnode) -> core::ffi::c_int {
    let mut count: core::ffi::c_int = 0 as core::ffi::c_int;
    if root.is_null() {
        return 0 as core::ffi::c_int
    } else if ((*root).left).is_null() && ((*root).right).is_null() {
        return 0 as core::ffi::c_int
    }
    if !((*root).left).is_null() || !((*root).right).is_null() {
        count = 1 as core::ffi::c_int;
        if !((*root).left).is_null() {
            count += count_interiornode((*root).left);
        }
        if !((*root).right).is_null() {
            count += count_interiornode((*root).right);
        }
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn count_node(mut root: *mut tnode) -> core::ffi::c_int {
    return leafcount(root) + count_interiornode(root);
}
unsafe fn main_0() -> core::ffi::c_int {
    let mut t1: tree = tree { root: 0 as *mut tnode };
    t1.root = 0 as *mut tnode;
    let mut choice: core::ffi::c_int = 0;
    let mut ele: core::ffi::c_int = 0;
    let mut count: core::ffi::c_int = 0;
    loop {
        printf(
            b"\nEnter the choice from the following :\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        printf(
            b"1.Add an element in the binary tree\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        printf(
            b"2.Delete an element in the binary tree\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        printf(
            b"3.Display in inorder fashion\n\0" as *const u8 as *const core::ffi::c_char,
        );
        printf(
            b"4.Display in preorder fashion\n\0" as *const u8 as *const core::ffi::c_char,
        );
        printf(
            b"5.Display in postorder fashion\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        printf(b"6.Number of Leaf Nodes\n\0" as *const u8 as *const core::ffi::c_char);
        printf(
            b"7.Number of Interior Nodes\n\0" as *const u8 as *const core::ffi::c_char,
        );
        printf(b"8.Total Number of Nodes\n\0" as *const u8 as *const core::ffi::c_char);
        printf(b"9.Exit\n\0" as *const u8 as *const core::ffi::c_char);
        scanf(
            b"%d\0" as *const u8 as *const core::ffi::c_char,
            &mut choice as *mut core::ffi::c_int,
        );
        let mut current_block_30: u64;
        match choice {
            1 => {
                printf(
                    b"Enter the element to be added to the binary tree :\0" as *const u8
                        as *const core::ffi::c_char,
                );
                scanf(
                    b"%d\0" as *const u8 as *const core::ffi::c_char,
                    &mut ele as *mut core::ffi::c_int,
                );
                insert(&mut t1, ele);
                printf(
                    b"The element  %d is inserted\0" as *const u8
                        as *const core::ffi::c_char,
                    ele,
                );
                current_block_30 = 11057878835866523405;
            }
            2 => {
                printf(
                    b"Enter the element to be deleted from binary tree :\0" as *const u8
                        as *const core::ffi::c_char,
                );
                scanf(
                    b"%d\0" as *const u8 as *const core::ffi::c_char,
                    &mut ele as *mut core::ffi::c_int,
                );
                deletenode(&mut t1, ele);
                current_block_30 = 8381896658700674431;
            }
            3 => {
                current_block_30 = 8381896658700674431;
            }
            4 => {
                printf(
                    b"\nThe numbers in the preorder fashion are : \0" as *const u8
                        as *const core::ffi::c_char,
                );
                disp_preorder(t1.root);
                current_block_30 = 11057878835866523405;
            }
            5 => {
                printf(
                    b"\nThe numbers in the postorder fashion are : \0" as *const u8
                        as *const core::ffi::c_char,
                );
                disp_postorder(t1.root);
                current_block_30 = 11057878835866523405;
            }
            6 => {
                count = leafcount(t1.root);
                printf(
                    b"The number of Leaf nodes are : %d\0" as *const u8
                        as *const core::ffi::c_char,
                    count,
                );
                current_block_30 = 11057878835866523405;
            }
            7 => {
                count = count_interiornode(t1.root);
                printf(
                    b"The number of interior nodes are : %d\0" as *const u8
                        as *const core::ffi::c_char,
                    count,
                );
                current_block_30 = 11057878835866523405;
            }
            8 => {
                count = count_node(t1.root);
                printf(
                    b"The total number of nodes are : %d\0" as *const u8
                        as *const core::ffi::c_char,
                    count,
                );
                current_block_30 = 11057878835866523405;
            }
            _ => {
                current_block_30 = 11057878835866523405;
            }
        }
        match current_block_30 {
            8381896658700674431 => {
                printf(
                    b"\nThe numbers in the inorder fashion are : \0" as *const u8
                        as *const core::ffi::c_char,
                );
                disp_inorder(t1.root);
            }
            _ => {}
        }
        if !(choice != 9 as core::ffi::c_int) {
            break;
        }
    }
    return 0 as core::ffi::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
