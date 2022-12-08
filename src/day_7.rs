use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{Read, Result},
    rc::Rc, cmp,
};

use regex::Regex;

const INPUT: &str = "inputs/day_7.txt";
const TEST: &str = "inputs/test.txt";
const DISK_SPACE: usize = 70000000;
const UPDATE: usize = 30000000;

pub fn determine_directory_sizes() -> Result<usize> {
    let input = get_input(INPUT)?;
    let root = create_filesystem(input);
    let res = &mut 0;
    get_sizes(root, res);
    Ok(*res)
}

pub fn get_deleted_directory() -> Result<usize> {
    let input = get_input(INPUT)?;
    let root = create_filesystem(input);
    get_sizes(root.clone(), &mut 0);
    let mut res = root.borrow_mut().size;
    let limit = UPDATE - (DISK_SPACE - res);
    delete_directory(root, limit, &mut res);
    Ok(res)
}

fn create_filesystem(input: String) -> INodeHandle {
    let root = INode::dir();
    let mut curr = root.clone();
    let re = Regex::new(r"\d+ ").unwrap();
    for line in input.lines() {
        if line.starts_with("$ cd") {
            curr = match &line[5..] {
                "/" => root.clone(),
                ".." => curr.borrow().parent.clone().unwrap(),
                name => {curr.borrow_mut().children.entry(name.to_string()).or_insert(INode::dir()).clone()}
            };
        } else if line.starts_with("dir") {
            let dir = line.split_once(" ").unwrap().1;
            let node = curr.borrow_mut().children.entry(dir.to_string()).or_insert(INode::dir()).clone();
            node.borrow_mut().parent = Some(curr.clone());
        } else if re.is_match(line) {
            let (size, file) = line.split_once(" ").unwrap();
            let node = curr.borrow_mut().children.entry(file.to_string()).or_insert(INode::file()).clone();
            node.borrow_mut().parent = Some(curr.clone());
            node.borrow_mut().size = size.parse::<usize>().unwrap();
        }
    }
    root
}

type INodeHandle = Rc<RefCell<INode>>;

#[derive(Debug, Default)]
struct INode {
    size: usize,
    is_dir: bool,
    parent: Option<INodeHandle>,
    children: HashMap<String, INodeHandle>,
}

impl INode {
    fn dir() -> INodeHandle {
        Rc::new(RefCell::new(INode {size: 0, is_dir: true, parent: None, children: HashMap::default()}))
    }

    fn file() -> INodeHandle {
        Rc::new(RefCell::new(INode {size: 0, is_dir: false, parent: None, children: HashMap::default()}))
    }
}

fn get_sizes(curr: INodeHandle, res: &mut usize) -> usize {
    let mut size = 0;
    for node in curr.borrow_mut().children.values().cloned() {
        size += get_sizes(node, res);
    }
    size += curr.borrow().size;
    curr.borrow_mut().size = size;
    if curr.borrow().is_dir && size <= 100000 {
        *res += size
    }
    size
}

fn delete_directory(curr: INodeHandle, limit: usize, res: &mut usize) {
    for node in curr.borrow_mut().children.values().cloned() {
        delete_directory(node, limit, res);
    }
    let size = curr.borrow().size;
    if curr.borrow().is_dir && curr.borrow().size >= limit {
        *res = cmp::min(*res, size);
    }
}

fn get_input(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
