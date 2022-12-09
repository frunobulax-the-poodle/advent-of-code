#![feature(result_option_inspect)]
use std::boxed::Box;
use std::collections::HashMap;
use std::iter::Peekable;
use std::ptr::NonNull;

#[aoc::main]
fn main() {
    let mut root = FsNode::root();
    let mut cmds = input.lines().skip(1);
    root.exec_all(&mut cmds);
    let used = root.build_dir_size();
    let root_dir = FsNode::Dir(root);
    let fst: usize = root_dir
        .dfs()
        .filter_map(|node| match node {
            FsNode::Dir(dir) => dir.data.size.filter(|s| s < &100_000),
            _ => None,
        })
        .sum();
    let max_size = 70_000_000 - 30_000_000;
    let snd = root_dir
        .dfs()
        .filter_map(|node| match node {
            FsNode::Dir(dir) => dir.data.size.filter(|s| *s > used - max_size),
            _ => None,
        })
        .min()
        .unwrap();
    (fst, snd)
}

enum FsNode {
    Dir(Directory),
    File(File),
}

struct FsNodeDFS<'a> {
    stack: Vec<&'a FsNode>,
}

impl<'a> Iterator for FsNodeDFS<'a> {
    type Item = &'a FsNode;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().inspect(|node| match node {
            FsNode::Dir(dir) => self.stack.extend(dir.children.values().map(Box::as_ref)),
            FsNode::File(_f) => (),
        })
    }
}

impl FsNode {
    fn root() -> Directory {
        Directory {
            data: File {
                parent: None,
                size: None,
            },
            children: HashMap::new(),
        }
    }

    fn dfs(&self) -> FsNodeDFS {
        FsNodeDFS { stack: vec![self] }
    }
}

struct Directory {
    data: File,
    children: HashMap<String, Box<FsNode>>,
}

impl Directory {
    fn exec_all<'a, I: Iterator<Item = &'a str>>(&mut self, lines: &mut I) {
        let mut peek = lines.peekable();
        let mut dir = self;
        while let Some(line) = peek.next() {
            let cmd: Vec<_> = line.split_whitespace().skip(1).collect();
            dir = match cmd[0] {
                "cd" => dir.change_dir(cmd[1]),
                "ls" => dir.add_children(&mut peek),
                _ => panic!("unknown commands"),
            };
        }
    }

    fn change_dir(&mut self, name: &str) -> &mut Directory {
        if name == ".." {
            unsafe { self.data.parent.expect("No parent found").as_mut() }
        } else if let Some(boxed) = self.children.get_mut(name) {
            if let FsNode::Dir(d) = boxed.as_mut() {
                d
            } else {
                panic!("{} is not a directory", name);
            }
        } else {
            panic!("{} is not a directory", name);
        }
    }

    fn add_children<'a, I>(&mut self, lines: &mut Peekable<I>) -> &mut Directory
    where
        I: Iterator<Item = &'a str>,
    {
        let ptr = NonNull::new(self as *mut _).unwrap();
        while let Some(line) = lines.next_if(|l| !l.starts_with('$')) {
            let mut parts = line.split_whitespace();
            let p = parts.next().unwrap();
            let name = parts.next().unwrap();
            let child = if p == "dir" {
                FsNode::Dir(Directory {
                    data: File {
                        parent: Some(ptr),
                        size: None,
                    },
                    children: HashMap::new(),
                })
            } else {
                let size: usize = p.parse().unwrap();
                FsNode::File(File {
                    parent: Some(ptr),
                    size: Some(size),
                })
            };
            self.children.insert(name.to_owned(), Box::new(child));
        }
        self
    }
    fn build_dir_size(&mut self) -> usize {
        let size = self
            .children
            .values_mut()
            .map(|boxed| match boxed.as_mut() {
                FsNode::Dir(dir) => dir.build_dir_size(),
                FsNode::File(file) => file.size.unwrap(),
            })
            .sum();
        self.data.size = Some(size);
        size
    }
}

struct File {
    parent: Option<NonNull<Directory>>,
    size: Option<usize>,
}
