use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum FSObject<'a> {
    Root {
        children: RefCell<HashMap<&'a str, Rc<FSObject<'a>>>>,
    },
    Directory {
        parent: Weak<FSObject<'a>>,
        name: &'a str,
        children: RefCell<HashMap<&'a str, Rc<FSObject<'a>>>>,
    },
    File {
        parent: Weak<FSObject<'a>>,
        name: &'a str,
        size: usize,
    },
}

fn add_directory<'a>(parent: Rc<FSObject<'a>>, name: &'a str) {
    let dir = Rc::new(FSObject::Directory {
        parent: Rc::downgrade(&parent),
        name: name,
        children: RefCell::new(HashMap::new()),
    });

    if let FSObject::Directory { children, .. } = &*parent {
        let mut map = children.borrow_mut();
        if !map.contains_key(name) {
            map.insert(name, dir);
        }
    } else if let FSObject::Root { children } = &*parent {
        let mut map = children.borrow_mut();
        if !map.contains_key(name) {
            map.insert(name, dir);
        }
    }
}

fn add_file<'a>(parent: Rc<FSObject<'a>>, name: &'a str, size: usize) {
    let f = Rc::new(FSObject::File {
        parent: Rc::downgrade(&parent),
        name: name,
        size: size,
    });

    if let FSObject::Directory { children, .. } = &*parent {
        let mut map = children.borrow_mut();
        if !map.contains_key(name) {
            map.insert(name, f);
        }
    } else if let FSObject::Root { children } = &*parent {
        let mut map = children.borrow_mut();
        if !map.contains_key(name) {
            map.insert(name, f);
        }
    }
}

fn find_dir<'a>(parent: Rc<FSObject<'a>>, search_name: &str) -> Option<Rc<FSObject<'a>>> {
    if let FSObject::Directory { children, .. } = &*parent {
        return children.borrow().get(search_name).map(|s| Rc::clone(s));
    } else if let FSObject::Root { children } = &*parent {
        return children.borrow().get(search_name).map(|s| Rc::clone(s));
    }

    None
}

fn calc_size<'a>(node: Rc<FSObject<'a>>, dirs: &mut Vec<(&'a str, usize)>) -> usize {
    if let FSObject::Directory { children, name, .. } = &*node {
        let size = children
            .borrow()
            .values()
            .map(|n| calc_size(Rc::clone(n), dirs))
            .sum();
        dirs.push((name, size));
        return size;
    } else if let FSObject::Root { children } = &*node {
        let size = children
            .borrow()
            .values()
            .map(|n| calc_size(Rc::clone(n), dirs))
            .sum();
        dirs.push(("/", size));
        return size;
    } else if let FSObject::File { size, .. } = &*node {
        return *size;
    }

    0
}

pub fn main() {
    let data = include_str!("./input.txt");

    let root = Rc::new(FSObject::Root {
        children: RefCell::new(HashMap::new()),
    });

    let mut working_dir = Rc::clone(&root);

    let mut stupid_check = 0;

    for line in data.lines() {
        if line == "$ cd /" {
            working_dir = Rc::clone(&root);
        } else if line == "$ cd .." {
            if let FSObject::Directory { parent, .. } = &*Rc::clone(&working_dir) {
                if let Some(parent) = parent.upgrade() {
                    working_dir = parent;
                }
            }
        } else if line.starts_with("$ cd ") {
            if let Some(dir) =
                find_dir(Rc::clone(&working_dir), line.strip_prefix("$ cd ").unwrap())
            {
                working_dir = dir
            }
        } else if line == "$ ls" {
        } else if line.starts_with("dir") {
            add_directory(Rc::clone(&working_dir), line.strip_prefix("dir ").unwrap())
        } else {
            if let Some((size, name)) = line.split_once(' ') {
                stupid_check += size.parse::<usize>().unwrap();
                add_file(
                    Rc::clone(&working_dir),
                    name,
                    size.parse::<usize>().unwrap(),
                )
            }
        }
    }

    let mut dirs = vec![];

    calc_size(Rc::clone(&root), &mut dirs);

    dirs.sort_by_key(|(_, s)| *s);

    let used = dirs.last().unwrap().1;

    let needed = 30000000 - (70000000 - used);

    let dir_delete = dirs.iter().find(|(_, s)| *s > needed);

    println!(
        "used: {used}\nneed: {needed}\nhave: {:?}\nshould delete: {dir_delete:?}",
        dirs
    );
}
