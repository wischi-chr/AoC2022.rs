use std::{
    cell::RefCell,
    iter::Peekable,
    rc::{Rc, Weak},
};

use crate::{
    aoc_general::{PuzzlePart, PuzzleSolver},
    common::{LfEofDropable, LineSplittable, NormalizeLineBreaks},
};

#[derive(Default)]
pub struct Day7;

const TOTAL_DISK_SPACE: u32 = 70_000_000;
const REQUIRED_FREE_SPACE: u32 = 30_000_000;

impl PuzzleSolver for Day7 {
    fn solve(&self, input: &mut dyn Iterator<Item = u8>, part: PuzzlePart) -> String {
        let mut fs = build_file_system(input);

        match part {
            PuzzlePart::Part1 => {
                let mut sum = 0;

                fs.walk_recursively_mut(|n| {
                    if let Node::Folder(f) = n {
                        let folder_size = f.as_ref().borrow_mut().get_size();

                        if folder_size <= 100_000 {
                            sum += folder_size;
                        }
                    }
                });

                sum.to_string()
            }
            PuzzlePart::Part2 => {
                let used_space = fs.root.as_ref().borrow_mut().get_size();
                let free_space = TOTAL_DISK_SPACE - used_space;
                let space_delete = REQUIRED_FREE_SPACE - free_space;

                let mut found_folder_size = u32::MAX;

                fs.walk_recursively_mut(|n| {
                    if let Node::Folder(f) = n {
                        let folder_size = f.as_ref().borrow_mut().get_size();

                        if folder_size < found_folder_size && folder_size >= space_delete {
                            found_folder_size = folder_size;
                        }
                    }
                });

                found_folder_size.to_string()
            }
        }
    }
}

fn build_file_system(input: &mut dyn Iterator<Item = u8>) -> FileSystem {
    let mut lines = input
        .normalize_line_breaks()
        .split_lf_line_breaks()
        .drop_lf_eof()
        .peekable();

    let mut fs = FileSystem::new();

    while let Some(line) = lines.next() {
        match &line[0..4] {
            b"$ cd" => process_cd(&mut fs, &line),
            b"$ ls" => process_ls(&mut fs, &mut lines),
            _ => panic!("unknown command"),
        };
    }

    // force update all cached sizes
    _ = fs.root.as_ref().borrow_mut().get_size();

    fs
}

fn process_cd(fs: &mut FileSystem, line: &[u8]) {
    let name = String::from_utf8_lossy(&line[5..]).to_string();
    fs.cd(&name);
}

fn process_ls<I: Iterator<Item = Vec<u8>>>(fs: &mut FileSystem, lines: &mut Peekable<I>) {
    loop {
        let l = lines.peek();

        let x = match l {
            None => return,
            Some(x) => x,
        };

        if x[0] == b'$' {
            return;
        }

        if x.starts_with(b"dir ") {
            let name = String::from_utf8_lossy(&x[4..]);
            fs.find_or_create_subdirectory(&name);
        } else {
            let space_index = x
                .iter()
                .position(|y| y == &b' ')
                .expect("file line should contain a space");

            let file_size = String::from_utf8_lossy(&x[..space_index])
                .parse::<u32>()
                .expect("file size should be a number.");

            let file = File { size: file_size };

            {
                let mut dir_mut = fs.working_dir.as_ref().borrow_mut();

                dir_mut.children.push(Node::File(file));
                dir_mut.size = None;
            }
        }

        // consume the line we've peeked before
        _ = lines.next();
    }
}

struct File {
    size: u32,
}

struct Folder {
    size: Option<u32>,
    name: String,
    children: Vec<Node>,
    parent: Weak<RefCell<Folder>>,
}

enum Node {
    File(File),
    Folder(Rc<RefCell<Folder>>),
}

struct FileSystem {
    root: Rc<RefCell<Folder>>,
    working_dir: Rc<RefCell<Folder>>,
}

impl Folder {
    pub fn get_size(&mut self) -> u32 {
        if let Some(x) = self.size {
            return x;
        }

        let sum = self
            .children
            .iter()
            .map(|x| match x {
                Node::File(f) => f.size,
                Node::Folder(f) => f.as_ref().borrow_mut().get_size(),
            })
            .sum();

        self.size = Some(sum);
        sum
    }
}

impl FileSystem {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(Folder {
            size: None,
            name: String::new(),
            children: vec![],
            parent: Weak::new(),
        }));

        {
            let weak_root = Rc::downgrade(&root);
            let mut root_mut = root.as_ref().borrow_mut();
            root_mut.parent = weak_root;
        }

        FileSystem {
            root: root.clone(),
            working_dir: root,
        }
    }

    /// Change into a directory (and create if it doesn't yet exist).
    pub fn cd(&mut self, name: &str) {
        if name == "/" {
            // edge case for root directory
            self.working_dir = self.root.clone();
            return;
        }

        if name == ".." {
            let parent = self.working_dir.as_ref().borrow().parent.upgrade();

            if let Some(x) = parent {
                self.working_dir = x;
                return;
            }

            return;
        }

        let found = self.find_or_create_subdirectory(&name);

        self.working_dir = found;
    }

    fn find_or_create_subdirectory(&mut self, name: &str) -> Rc<RefCell<Folder>> {
        let found = self.find_subdirectory(name);

        if let Some(d) = found {
            return d;
        }

        let folder = Folder {
            size: None,
            name: name.to_string(),
            children: vec![],
            parent: Rc::downgrade(&self.working_dir),
        };

        let folder = Rc::new(RefCell::new(folder));

        {
            let mut dir_mut = self.working_dir.as_ref().borrow_mut();
            dir_mut.children.push(Node::Folder(folder.clone()));
            dir_mut.size = None;
        }

        folder
    }

    fn find_subdirectory(&mut self, name: &str) -> Option<Rc<RefCell<Folder>>> {
        for node in self.working_dir.borrow().children.iter() {
            if let Node::Folder(f) = node {
                if f.borrow().name == name {
                    return Some(f.clone());
                }
            }
        }

        None
    }

    pub fn walk_recursively_mut<F>(&mut self, mut process: F)
    where
        F: FnMut(&mut Node) -> (),
    {
        let mut x = Node::Folder(self.root.clone());
        self.walk_recursively_mut_internal(&mut process, &mut x);
    }

    fn walk_recursively_mut_internal<F>(&mut self, process: &mut F, node: &mut Node)
    where
        F: FnMut(&mut Node) -> (),
    {
        match node {
            Node::File(_) => process(node),
            Node::Folder(_) => {
                process(node);

                if let Node::Folder(f) = node {
                    let mut folder = f.as_ref().borrow_mut();

                    for child in folder.children.iter_mut() {
                        self.walk_recursively_mut_internal(process, child);
                    }
                }
            }
        }
    }
}
