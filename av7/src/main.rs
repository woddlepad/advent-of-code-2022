use std::{
    cell::RefCell,
    fmt, fs,
    rc::{Rc, Weak},
};

type WeakRcMutPointer<T> = Weak<RefCell<T>>;
type RcMutPointer<T> = Rc<RefCell<T>>;

struct PrettyNode<'a>(&'a Node<FileSystemItem>);

impl<'a> fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node = self.0;

        match &node.value {
            FileSystemItem::Directory(dir) => {
                writeln!(f, "- {} (dir size={}) ", dir.name, node.calc_size())
            }
            FileSystemItem::File(file) => {
                writeln!(f, "- {} (file, size={})", file.name, file.size,)
            }
        };

        for child in &node.children {
            for line in format!("{:?}", PrettyNode(&child.borrow())).lines() {
                writeln!(f, "  {line}")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct Command {
    name: String,
    args: Vec<String>,
    output: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct File {
    name: String,
    size: i32,
}
#[derive(Clone, Debug, PartialEq)]
struct SizedDirectory {
    dir: Directory,
    size: i32,
}

#[derive(Clone, Debug, PartialEq)]
struct Directory {
    name: String,
}
#[derive(Clone, Debug, PartialEq)]
enum FileSystemItem {
    File(File),
    Directory(Directory),
}

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    children: Vec<RcMutPointer<Node<T>>>,
    parent: Option<WeakRcMutPointer<Node<T>>>,
}

impl<T> From<Node<T>> for RcMutPointer<Node<T>> {
    fn from(value: Node<T>) -> Self {
        Rc::new(RefCell::new(value))
    }
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            children: vec![],
            parent: None,
        }
    }

    fn add(&mut self, value: T, self_ref: RcMutPointer<Node<T>>) {
        let node_ref: Rc<RefCell<Node<T>>> = Node::new(value).into();
        node_ref.borrow_mut().parent = Some(Rc::downgrade(&self_ref));

        self.children.push(node_ref);
    }
}

impl Node<FileSystemItem> {
    fn find_child(&self, name: &str) -> RcMutPointer<Node<FileSystemItem>> {
        match &self.value {
            FileSystemItem::File(_) => panic!(),
            FileSystemItem::Directory(_) => self
                .children
                .iter()
                .filter(|node| match &node.borrow().value {
                    FileSystemItem::File(_) => false,
                    FileSystemItem::Directory(dir) => dir.name == name,
                })
                .next()
                .unwrap()
                .clone(),
        }
    }

    fn calc_size(&self) -> i32 {
        match &self.value {
            FileSystemItem::File(file) => file.size,
            FileSystemItem::Directory(_) => self
                .children
                .iter()
                .map(|c| (*c).borrow().calc_size())
                .sum(),
        }
    }

    fn get_flat_dirs(&self) -> Vec<SizedDirectory> {
        let mut dirs = vec![];
        match &self.value {
            FileSystemItem::File(_) => {}
            FileSystemItem::Directory(dir) => {
                dirs.push(SizedDirectory {
                    dir: dir.clone(),
                    size: self.calc_size(),
                });

                let mut child_dirs: Vec<SizedDirectory> = self
                    .children
                    .iter()
                    .flat_map(|node: &RcMutPointer<Node<FileSystemItem>>| {
                        node.borrow().get_flat_dirs()
                    })
                    .collect();
                dirs.append(&mut child_dirs);
            }
        }
        return dirs;
    }
}

fn parse_file_system_entry(commands: Vec<Command>) -> RcMutPointer<Node<FileSystemItem>> {
    let root = Node::new(FileSystemItem::Directory(Directory {
        name: "/".to_string(),
    }));
    let root_ref: RcMutPointer<Node<FileSystemItem>> = root.into();
    let mut current_node = root_ref.clone();
    for command in commands {
        match &command.name[..] {
            "cd" => {
                let destination = &command.args[0][..];
                match destination {
                    "/" => {}
                    ".." => {
                        let parent = current_node.borrow().parent.clone().unwrap();
                        current_node = parent.upgrade().unwrap();
                    }
                    path => {
                        let child = current_node.borrow().find_child(path).clone();
                        current_node = child;
                    }
                }
            }
            "ls" => {
                for line in command.output {
                    let args: Vec<&str> = line.split(" ").collect();

                    match args[0] {
                        "dir" => (*current_node).borrow_mut().add(
                            FileSystemItem::Directory(Directory {
                                name: args[1].to_string(),
                            }),
                            current_node.clone(),
                        ),
                        _ => (*current_node).borrow_mut().add(
                            FileSystemItem::File(File {
                                name: args[1].to_string(),
                                size: args[0].parse().unwrap(),
                            }),
                            current_node.clone(),
                        ),
                    }
                }
            }
            _ => {}
        }
    }
    return root_ref;
}

fn parse_commands(terminal_log: &str) -> Vec<Command> {
    let mut commands = Vec::<Command>::new();
    let mut current_command: Option<Command> = None;
    let mut current_output: Vec<String> = Vec::new();
    for line in terminal_log.lines() {
        let args: Vec<&str> = line.split(" ").collect();
        if args[0] == "$" {
            if current_command.is_some() {
                let mut command = current_command.unwrap();
                command.output = current_output.clone();
                commands.push(command);
                current_output = Vec::new();
            }
            current_command = Some(Command {
                name: args[1].to_string(),
                args: args[2..].iter().map(|x| x.to_string()).collect(),
                output: Vec::new(),
            });
        } else {
            if current_command.is_some() {
                current_output.push(line.to_string())
            }
        }
    }
    if current_command.is_some() {
        let mut command = current_command.unwrap();
        command.output = current_output.clone();
        commands.push(command);
    }
    return commands;
}

fn main() {
    let file_path = "./input.txt";
    let file_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let commands = parse_commands(&file_content);
    println!("{:?}", commands);

    let root = parse_file_system_entry(commands);
    println!("{:?}", PrettyNode(&root.borrow()));

    let flat_dirs = root.borrow().get_flat_dirs();
    let small_dirs: Vec<&SizedDirectory> = flat_dirs
        .iter()
        .filter(|dir| dir.size < 100_000)
        .map(|dir| dir)
        .collect();
    println!("{:?}", small_dirs);

    println!("{}", small_dirs.len());
}
