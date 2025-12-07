fn replace_right_child(node: &mut Node) -> Option<&mut Node> {
    if node.right.is_none() {
        node.left.as_deref_mut()
    } else {
        None
    }
}

use std::cmp::Ordering;

#[derive(Debug)]
struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

fn insert(tree: &mut Tree, x: i32) {
    let new_node = Box::new(Node {
        data: x,
        left: None,
        right: None,
    });

    if tree.root.is_none() {
        tree.root = Some(new_node);
        return;
    }

    let mut current = tree.root.as_mut().unwrap();
    loop {
        let next = match new_node.data.cmp(&current.data) {
            Ordering::Less => &mut current.left,
            Ordering::Greater | Ordering::Equal => &mut current.right,
        };

        if next.is_some() {
            current = next.as_mut().unwrap();
        } else {
            *next = Some(new_node);
            break;
        }
    }
}

fn deletenode(tree: &mut Tree, x: i32) -> Option<()> {
    let mut prev: Option<*mut Node> = None;
    let mut current: Option<&mut Box<Node>> = tree.root.as_mut();
    
    loop {
        match current.take() {
            Some(mut node) if node.data == x => {
                current = Some(node);
                break;
            }
            Some(mut node) => {
                let cmp = x.cmp(&node.data);
                prev = Some(node.as_mut() as *mut Node);
                current = match cmp {
                    Ordering::Less => node.left.as_mut(),
                    Ordering::Greater => node.right.as_mut(),
                    Ordering::Equal => unreachable!(),
                };
            }
            None => return None,
        }
    }
    
    let p = current?;
    
    let rp = match (&p.left, &p.right) {
        (None, None) => None,
        (Some(_), None) => p.left.take(),
        (None, Some(_)) => p.right.take(),
        (Some(_), Some(_)) => {
            let mut rp = p.right.take().unwrap();
            if rp.left.is_none() {
                rp.left = p.left.take();
                Some(rp)
            } else {
                let mut f = &mut rp;
                let mut s = f.left.take();
                
                while let Some(mut s_node) = s {
                    if s_node.left.is_none() {
                        f.left = s_node.right.take();
                        s_node.left = p.left.take();
                        s_node.right = p.right.take();
                        return Some(());
                    }
                    let next_s = s_node.left.take();
                    f.left = Some(s_node);
                    f = f.left.as_mut().unwrap();
                    s = next_s;
                }
                unreachable!()
            }
        }
    };
    
    unsafe {
        match prev {
            None => tree.root = rp,
            Some(prev_ptr) => {
                let prev_node = &mut *prev_ptr;
                if prev_node.left.as_ref().map(|n| &**n as *const Node) == Some(&**p as *const Node) {
                    prev_node.left = rp;
                } else {
                    prev_node.right = rp;
                }
            }
        }
    }
    
    Some(())
}

fn disp_inorder(q: Option<&Node>) {
    if let Some(node) = q {
        disp_inorder(node.left.as_deref());
        print!("{}\t", node.data);
        disp_inorder(node.right.as_deref());
    }
}

fn disp_preorder(q: Option<&Node>) {
    if let Some(node) = q {
        print!("{}\t", node.data);
        disp_preorder(node.left.as_deref());
        disp_preorder(node.right.as_deref());
    }
}

fn disp_postorder(q: Option<&Node>) {
    if let Some(node) = q {
        disp_postorder(node.left.as_deref());
        disp_postorder(node.right.as_deref());
        print!("{}\t", node.data);
    }
}

fn leafcount(p: Option<&Node>) -> i32 {
    match p {
        Some(node) => {
            let left_is_leaf = node.left.is_none();
            let right_is_leaf = node.right.is_none();
            
            if left_is_leaf && right_is_leaf {
                1
            } else {
                let left_count = leafcount(node.left.as_deref());
                let right_count = leafcount(node.right.as_deref());
                left_count + right_count
            }
        }
        None => 0,
    }
}

fn count_interiornode(root: Option<&Node>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            if node.left.is_none() && node.right.is_none() {
                0
            } else {
                let mut count = 1;
                if let Some(left) = &node.left {
                    count += count_interiornode(Some(left.as_ref()));
                }
                if let Some(right) = &node.right {
                    count += count_interiornode(Some(right.as_ref()));
                }
                count
            }
        }
    }
}

fn count_node(root: Option<&Node>) -> i32 {
    leafcount(root) + count_interiornode(root)
}

use std::io::{self, Write};

fn main() {
    let mut t1 = Tree { root: None };
    
    loop {
        println!("\nEnter the choice from the following:");
        println!("1. Add an element in the binary tree");
        println!("2. Delete an element in the binary tree");
        println!("3. Display in inorder fashion");
        println!("4. Display in preorder fashion");
        println!("5. Display in postorder fashion");
        println!("6. Number of Leaf Nodes");
        println!("7. Number of Interior Nodes");
        println!("8. Total Number of Nodes");
        println!("9. Exit");
        
        print!("Your choice: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let choice: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        
        match choice {
            1 => {
                print!("Enter the element to be added to the binary tree: ");
                io::stdout().flush().unwrap();
                
                let mut ele_input = String::new();
                io::stdin().read_line(&mut ele_input).unwrap();
                
                let ele: i32 = match ele_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number.");
                        continue;
                    }
                };
                
                insert(&mut t1, ele);
                println!("The element {} is inserted", ele);
            }
            2 => {
                print!("Enter the element to be deleted from binary tree: ");
                io::stdout().flush().unwrap();
                
                let mut ele_input = String::new();
                io::stdin().read_line(&mut ele_input).unwrap();
                
                let ele: i32 = match ele_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number.");
                        continue;
                    }
                };
                
                if deletenode(&mut t1, ele).is_some() {
                    println!("Element {} deleted", ele);
                } else {
                    println!("Element not found!");
                }
            }
            3 => {
                print!("\nThe numbers in the inorder fashion are: ");
                disp_inorder(t1.root.as_deref());
                println!();
            }
            4 => {
                print!("\nThe numbers in the preorder fashion are: ");
                disp_preorder(t1.root.as_deref());
                println!();
            }
            5 => {
                print!("\nThe numbers in the postorder fashion are: ");
                disp_postorder(t1.root.as_deref());
                println!();
            }
            6 => {
                let count = leafcount(t1.root.as_deref());
                println!("Number of leaf nodes: {}", count);
            }
            7 => {
                let count = count_interiornode(t1.root.as_deref());
                println!("Number of interior nodes: {}", count);
            }
            8 => {
                let count = count_node(t1.root.as_deref());
                println!("Total number of nodes: {}", count);
            }
            9 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 9.");
            }
        }
    }
}