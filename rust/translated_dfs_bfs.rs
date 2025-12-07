use std::io::{self, Write};

const MAX: usize = 10;

fn init_queue(rear: &mut i32, front: &mut i32) {
    if *rear == -1 {
        *rear = 0;
        *front = 0;
    }
}

fn push(val: i32, stack: &mut [i32], top: &mut usize) -> Result<(), &'static str> {
    if *top == stack.len() - 1 {
        Err("overflow")
    } else {
        *top += 1;
        stack[*top] = val;
        Ok(())
    }
}

fn pop(stack: &mut [i32], top: &mut i32) -> Option<i32> {
    if *top == -1 {
        eprintln!("\nunderflow");
        None
    } else {
        let value = stack[*top as usize];
        *top -= 1;
        Some(value)
    }
}

fn add(val: i32, cq: &mut [i32], front: &mut isize, rear: &mut isize, max: usize) -> Result<(), &'static str> {
    if *rear == max as isize - 1 && *front == 0 {
        return Err("overflow");
    } else if *rear == -1 {
        *rear = 0;
        *front = 0;
    } else if *rear == max as isize - 1 {
        *rear = 0;
    } else {
        *rear += 1;
    }
    
    let idx = *rear as usize;
    if idx >= cq.len() {
        return Err("index out of bounds");
    }
    cq[idx] = val;
    Ok(())
}

fn delete(cq: &mut [i32], front: &mut isize, rear: &mut isize, max: usize) -> Option<i32> {
    if *rear == -1 {
        return None;
    }

    let val = cq[*front as usize];

    if *front == max as isize - 1 {
        *front = 0;
    } else if *front == *rear {
        *front = -1;
        *rear = -1;
    } else {
        *front += 1;
    }

    Some(val)
}

fn initialise(adj: &mut [[i32; MAX]; MAX]) {
    for row in adj.iter_mut() {
        for cell in row.iter_mut() {
            *cell = 0;
        }
    }
}

fn addedge(adj: &mut [[i32; MAX]; MAX], i: usize, j: usize) {
    if i < MAX && j < MAX {
        adj[i][j] = 1;
    }
    if j < MAX && i < MAX {
        adj[j][i] = 1;
    }
}

fn removeedge(adj: &mut [[i32; MAX]; MAX], i: usize, j: usize) {
    if i < MAX && j < MAX {
        adj[i][j] = 0;
    }
    if j < MAX && i < MAX {
        adj[j][i] = 0;
    }
}

fn adjacent(i: usize, j: usize, adj: &[[i32; MAX]; MAX]) -> i32 {
    adj[i][j]
}

fn dfs(adj: &[[i32; MAX]; MAX], visited2: &mut [i32], start: i32) {
    let start_idx = start as usize;
    
    for v in visited2.iter_mut() {
        *v = 0;
    }
    
    let mut stack_dfs: [i32; MAX] = [0; MAX];
    let mut top_dfs: i32 = -1;
    
    top_dfs += 1;
    stack_dfs[top_dfs as usize] = start;
    
    print!("\t{}", start);
    visited2[start_idx] = 1;
    
    while top_dfs != -1 {
        let current = stack_dfs[top_dfs as usize];
        top_dfs -= 1;
        let current_idx = current as usize;
        
        for i in 0..MAX {
            if adj[current_idx][i] == 1 && visited2[i] == 0 {
                top_dfs += 1;
                stack_dfs[top_dfs as usize] = i as i32;
                print!("\t{}", i);
                visited2[i] = 1;
            }
        }
    }
}

fn bfs(adj: &[[i32; MAX]; MAX], visited1: &mut [i32], start: i32) -> () {
    let mut cq: [i32; MAX] = [0; MAX];
    let mut front: isize = -1;
    let mut rear: isize = -1;

    for v in visited1.iter_mut() {
        *v = 0;
    }

    front = -1;
    rear = -1;

    if let Err(e) = add(start, &mut cq, &mut front, &mut rear, MAX) {
        eprintln!("Queue error: {}", e);
        return;
    }

    visited1[start as usize] = 1;
    print!("\t{}", start);

    while front != -1 {
        let current_start = cq[front as usize];
        
        for i in 0..MAX {
            if adj[current_start as usize][i] == 1 && visited1[i] == 0 {
                if let Err(e) = add(i as i32, &mut cq, &mut front, &mut rear, MAX) {
                    eprintln!("Queue error: {}", e);
                    return;
                }
                print!("\t{}", i);
                visited1[i] = 1;
            }
        }

        if let Some(_) = delete(&mut cq, &mut front, &mut rear, MAX) {
            continue;
        } else {
            break;
        }
    }
    println!();
}

fn main() {
    let mut adj: [[i32; MAX]; MAX] = [[0; MAX]; MAX];
    let mut visited1: [i32; MAX] = [0; MAX];
    let mut visited2: [i32; MAX] = [0; MAX];
    
    initialise(&mut adj);
    
    loop {
        print!("\n 1=create edge \n 2= delete edge \n 3= bfs \n 4=dfs \n enter choice");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let opt: i32 = input.trim().parse().unwrap_or(-1);
        
        match opt {
            1 => {
                print!("\n Enter two numbers to create edge");
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let nums: Vec<&str> = input.trim().split_whitespace().collect();
                
                if nums.len() >= 2 {
                    if let (Ok(i), Ok(j)) = (nums[0].parse::<usize>(), nums[1].parse::<usize>()) {
                        if i < MAX && j < MAX {
                            addedge(&mut adj, i, j);
                            println!("\nEdge created between {} and {}", i, j);
                        } else {
                            println!("\nIndices must be less than {}", MAX);
                        }
                    } else {
                        println!("\nInvalid numbers");
                    }
                } else {
                    println!("\nPlease enter two numbers");
                }
                
                print!("\nContinue? \n 1=yes \n 0=no");
                io::stdout().flush().unwrap();
                let mut cont_input = String::new();
                io::stdin().read_line(&mut cont_input).unwrap();
                let cont: i32 = cont_input.trim().parse().unwrap_or(0);
                if cont == 0 {
                    println!("\nExiting...");
                    break;
                }
            }
            2 => {
                print!("\nenter two numbers to delete edge");
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let nums: Vec<&str> = input.trim().split_whitespace().collect();
                
                if nums.len() >= 2 {
                    if let (Ok(i), Ok(j)) = (nums[0].parse::<usize>(), nums[1].parse::<usize>()) {
                        if i < MAX && j < MAX {
                            removeedge(&mut adj, i, j);
                            println!("\nEdge removed between {} and {}", i, j);
                        } else {
                            println!("\nIndices must be less than {}", MAX);
                        }
                    } else {
                        println!("\nInvalid numbers");
                    }
                } else {
                    println!("\nPlease enter two numbers");
                }
                
                print!("\nContinue? \n 1=yes \n 0=no");
                io::stdout().flush().unwrap();
                let mut cont_input = String::new();
                io::stdin().read_line(&mut cont_input).unwrap();
                let cont: i32 = cont_input.trim().parse().unwrap_or(0);
                if cont == 0 {
                    println!("\nExiting...");
                    break;
                }
            }
            3 => {
                print!("\nenter the start for bfs");
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(start) = input.trim().parse::<i32>() {
                    if (start as usize) < MAX {
                        bfs(&adj, &mut visited1, start);
                    } else {
                        println!("\nStart must be less than {}", MAX);
                    }
                } else {
                    println!("\nInvalid start node");
                }
                
                print!("\nContinue? \n 1=yes \n 0=no");
                io::stdout().flush().unwrap();
                let mut cont_input = String::new();
                io::stdin().read_line(&mut cont_input).unwrap();
                let cont: i32 = cont_input.trim().parse().unwrap_or(0);
                if cont == 0 {
                    println!("\nExiting...");
                    break;
                }
            }
            4 => {
                print!("\nenter the start for dfs");
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(start) = input.trim().parse::<i32>() {
                    if (start as usize) < MAX {
                        dfs(&adj, &mut visited2, start);
                        println!();
                    } else {
                        println!("\nStart must be less than {}", MAX);
                    }
                } else {
                    println!("\nInvalid start node");
                }
                
                print!("\nContinue? \n 1=yes \n 0=no");
                io::stdout().flush().unwrap();
                let mut cont_input = String::new();
                io::stdin().read_line(&mut cont_input).unwrap();
                let cont: i32 = cont_input.trim().parse().unwrap_or(0);
                if cont == 0 {
                    println!("\nExiting...");
                    break;
                }
            }
            _ => {
                print!("\nInvalid input");
                io::stdout().flush().unwrap();
                print!("\nContinue? \n 1=yes \n 0=no");
                io::stdout().flush().unwrap();
                let mut cont_input = String::new();
                io::stdin().read_line(&mut cont_input).unwrap();
                let cont: i32 = cont_input.trim().parse().unwrap_or(0);
                if cont == 0 {
                    println!("\nExiting...");
                    break;
                }
            }
        }
    }
}