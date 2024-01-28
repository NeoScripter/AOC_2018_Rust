use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct Node {
    header: Header,
    children: Vec<Rc<RefCell<Node>>>,
    entries: Vec<u32>,
}
#[derive(Debug, Clone, Copy)]
struct Header {
    idx_c: u32,
    idx_m: u32,
}
impl Header {
    fn new(children: u32, entries: u32) -> Header {
        Header {
            idx_c: children,
            idx_m: entries,
        }
    }
}
impl Node {
    fn new(new_header: Header) -> Node {
        Node {
            header: new_header,
            children: Vec::new(),
            entries: Vec::new(),
        }
    }
    fn find_value(&self) -> u32 {
        if self.children.is_empty() {
            self.entries.iter().sum()
        } else {
            self.entries.iter().filter_map(|&entry| {
                self.children.get((entry as usize).saturating_sub(1))
                    .map(|child| child.borrow().find_value())
            }).sum()
        }
    }
    fn sum_entries(&self) -> u32 {
        let mut sum = 0;
        for c in self.children.iter() {
            sum += c.borrow().sum_entries()
        }
        self.entries.iter().sum::<u32>() + sum
    }
    fn print_tree(&self, depth: usize) {
        let indent = " ".repeat(depth * 2);
        println!("{}Node Header: Children count = {}, Metadata entries count = {}", 
                 indent, 
                 self.header.idx_c, 
                 self.header.idx_m);

        if !self.children.is_empty() {
            println!("{}Child Nodes:", indent);
            for child in &self.children {
                child.borrow().print_tree(depth + 1);
            }
        } else {
            println!("{}No child nodes", indent);
        }

        if !self.entries.is_empty() {
            println!("{}Metadata Entries: {:?}", indent, self.entries);
        } else {
            println!("{}No metadata entries", indent);
        }
    }
    fn make_tree(&mut self, data: &[u32], start: usize) -> usize {
        let mut current_index = start;

        // Process each child node
        for _ in 0..self.header.idx_c {
            let child_idx_c = data[current_index];
            let child_idx_m = data[current_index + 1];
            current_index += 2;

            let mut child_node = Node::new(Header::new(child_idx_c, child_idx_m));
            current_index = child_node.make_tree(data, current_index);
            self.children.push(Rc::new(RefCell::new(child_node)));
        }

        // Assign metadata entries
        self.entries.extend_from_slice(&data[current_index..current_index + self.header.idx_m as usize]);
        current_index += self.header.idx_m as usize;

        current_index
    }
}

fn create_tree(input: &str) -> Node {
    let num: Vec<u32> = input.split_whitespace().filter_map(|x| x.parse::<u32>().ok()).collect();
    let idx_c = num[0];
    let idx_m = num[1];
    let header = Header::new(idx_c, idx_m);
    let mut root = Node::new(header);

    root.make_tree(&num, 2);
    root
}
fn part1(input: &str) -> u32 {
    let root = create_tree(input);
    root.sum_entries()
}
fn part2(input: &str) -> u32 {
    let root = create_tree(input);
    root.print_tree(1);
    root.find_value()
}
fn main() {
    let input = include_str!("input8.txt");
    println!("{}", part2(input));
}