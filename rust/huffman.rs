use std::collections::HashMap;
use std::io::{self, Read, Write};

const MAXSYM: usize = 256;

struct Heap {
    n: usize,
    a: Vec<Option<Box<Node>>>,
}

impl Heap {
    fn new() -> Self {
        Self {
            n: 0,
            a: vec![None; MAXSYM],
        }
    }

    fn push(&mut self, x: Box<Node>) {
        let i = self.n;
        self.n += 1;
        self.a[i] = Some(x);

        let mut i = i;
        while i > 0 {
            let p = (i - 1) / 2;
            let p_freq = self.a[p].as_ref().unwrap().freq();
            let i_freq = self.a[i].as_ref().unwrap().freq();
            if p_freq <= i_freq {
                break;
            }
            self.a.swap(p, i);
            i = p;
        }
    }

    fn pop(&mut self) -> Option<Box<Node>> {
        if self.n == 0 {
            return None;
        }
        self.n -= 1;
        self.a.swap(0, self.n);
        let result = self.a[self.n].take();

        let mut i = 0;
        while i < self.n {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut smallest = i;

            if left < self.n {
                if let Some(left_node) = &self.a[left] {
                    let left_freq = left_node.freq();
                    let smallest_freq = self.a[smallest].as_ref().unwrap().freq();
                    if left_freq < smallest_freq {
                        smallest = left;
                    }
                }
            }

            if right < self.n {
                if let Some(right_node) = &self.a[right] {
                    let right_freq = right_node.freq();
                    let smallest_freq = self.a[smallest].as_ref().unwrap().freq();
                    if right_freq < smallest_freq {
                        smallest = right;
                    }
                }
            }

            if smallest == i {
                break;
            }
            self.a.swap(i, smallest);
            i = smallest;
        }

        result
    }

    fn len(&self) -> usize {
        self.n
    }
}

fn gen_codes(root: &Node, buf: &mut [u8], depth: usize, codes: &mut HashMap<u8, String>) {
    match root {
        Node::Leaf { sym, .. } => {
            let code = String::from_utf8_lossy(&buf[..depth]).to_string();
            codes.insert(*sym, code);
        }
        Node::Internal { left, right, .. } => {
            if depth < buf.len() {
                buf[depth] = b'0';
            }
            gen_codes(left, buf, depth + 1, codes);

            if depth < buf.len() {
                buf[depth] = b'1';
            }
            gen_codes(right, buf, depth + 1, codes);
        }
    }
}

fn main() -> io::Result<()> {
    let mut freq = [0u64; MAXSYM];
    let mut total = 0u64;

    let mut stdin = io::stdin().lock();
    let mut buffer = Vec::new();
    stdin.read_to_end(&mut buffer)?;

    for &byte in &buffer {
        freq[byte as usize] += 1;
        total += 1;
    }

    if total == 0 {
        eprintln!("no input");
        return Ok(());
    }

    let mut heap = Heap::new();
    for (sym, &freq_val) in freq.iter().enumerate() {
        if freq_val > 0 {
            heap.push(Box::new(Node::Leaf {
                freq: freq_val,
                sym: sym as u8,
            }));
        }
    }

    if heap.len() == 1 {
        let node = heap.pop().unwrap();
        let sym = match *node {
            Node::Leaf { sym, .. } => sym,
            _ => unreachable!(),
        };
        let mut codes = HashMap::new();
        codes.insert(sym, "0".to_string());

        let mut symbols: Vec<u8> = freq
            .iter()
            .enumerate()
            .filter(|(_, &f)| f > 0)
            .map(|(i, _)| i as u8)
            .collect();
        symbols.sort();

        for sym in symbols {
            let code = codes.get(&sym).unwrap();
            let i = sym as usize;
            if (32..127).contains(&i) && i != 39 {
                println!("'{}' : {}", i as u8 as char, code);
            } else {
                println!("{:02x} : {}", i, code);
            }
        }

        let mut stdout = io::stdout().lock();
        for &byte in &buffer {
            if let Some(code) = codes.get(&byte) {
                stdout.write_all(code.as_bytes())?;
            }
        }
        return Ok(());
    }

    while heap.len() > 1 {
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();
        let parent = Box::new(Node::Internal {
            freq: a.freq() + b.freq(),
            left: a,
            right: b,
        });
        heap.push(parent);
    }

    let root = heap.pop().unwrap();
    let mut codes = HashMap::new();
    let mut buf = vec![0u8; 256];
    gen_codes(&root, &mut buf, 0, &mut codes);

    let mut sorted_symbols: Vec<_> = codes.keys().collect();
    sorted_symbols.sort();

    for &sym in &sorted_symbols {
        let code = codes.get(sym).unwrap();
        let i = *sym as usize;
        if (32..127).contains(&i) && i != 39 {
            println!("'{}' : {}", i as u8 as char, code);
        } else {
            println!("{:02x} : {}", i, code);
        }
    }

    let mut stdout = io::stdout().lock();
    for &byte in &buffer {
        if let Some(code) = codes.get(&byte) {
            stdout.write_all(code.as_bytes())?;
        }
    }

    Ok(())
}

#[derive(Debug)]
enum Node {
    Leaf { freq: u64, sym: u8 },
    Internal {
        freq: u64,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Node {
    fn freq(&self) -> u64 {
        match self {
            Node::Leaf { freq, .. } => *freq,
            Node::Internal { freq, .. } => *freq,
        }
    }
}