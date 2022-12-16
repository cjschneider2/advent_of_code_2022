use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

fn main() {
    let input = include_str!("../input.txt");

    let part_1 = do_part_1(input);

    println!("Part 1: {:?}", part_1);
}

fn do_part_1(input: &str) -> usize {
    let fs = FileSystem::parse(input);
    0
}

struct DEnt {
    name: String,
    size: Option<usize>,
    children: Vec<Rc<RefCell<DEnt>>>,
}

impl DEnt {
    fn new(name: &str, size: Option<usize>) -> Self {
        Self {
            name: name.to_string(),
            size,
            children: vec![],
        }
    }
}

struct FileSystem {
    cd: Weak<RefCell<DEnt>>,
    root: Rc<RefCell<DEnt>>,
}

impl FileSystem {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(DEnt::new("/", None)));
        Self {
            cd: Rc::downgrade(&root),
            root,
        }
    }

    fn cd(&mut self, rel_path: &str) {
        let mut cd = self.cd.upgrade().unwrap();
        // nothing to do if in same dir
        if cd.borrow().name == rel_path { return; }
        // find child and cd
        let child = cd.borrow().children.iter()
            .find(|&&child| child.borrow().name == rel_path);
        match child {
            Some(child) => self.cd = Rc::downgrade(child),
            None => panic!(),
        }
    }

    fn mkdir(&mut self, name: &str) {
        let cd = self.cd.upgrade().unwrap().borrow_mut();
        cd.get_mut().children.push(Rc::new(RefCell::new(DEnt::new(name, None))))
    }

    fn mkdent(&mut self, name: &str, size: usize) {
        let mut dent = self.cd.upgrade().unwrap();
        let mut cd = dent.;
        cd.children.push(Rc::new(RefCell::new(DEnt::new(name, Some(size)))))
    }
}

#[derive(Debug)]
enum Token {
    ChangeDir(String),
    ListDir,
    Dir(String),
    DirEntry(usize, String),
}

impl FileSystem {
    fn parse(input: &str) -> Self {
        // parse out tokens
        let tokens = parse_token_steam(input);
        tokens.iter()
            .fold(FileSystem::new(), |mut fs, token| {
                match token {
                    Token::ChangeDir(dir) => {
                        fs.cd(dir);
                    }
                    Token::ListDir => {}
                    Token::Dir(name) => {
                        fs.mkdir(name);
                    }
                    Token::DirEntry(size, name) => {
                        fs.mkdent(name, *size);
                    }
                }
                fs
            })
    }
}

fn parse_token_steam(input: &str) -> Vec<Token> {
    input.lines().map(|line| {
        let cmd: Vec<&str> = line.split(' ').collect();
        match cmd[0] {
            "$" => match cmd[1] {
                "cd" => Token::ChangeDir(cmd[2].to_string()),
                "ls" => Token::ListDir,
                unknown => panic!("unknown option: {:?}", unknown),
            },
            "dir" => Token::Dir(cmd[1].to_string()),
            size => Token::DirEntry(
                size.parse::<usize>().unwrap(),
                cmd[1].to_string()),
        }
    }).collect()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = include_str!("../test_input.txt");

    use super::*;

    #[test]
    fn test_parsing() {
        let fs = FileSystem::parse(TEST_INPUT);
        assert_eq!(fs.cd.upgrade().unwrap().name, "b");
    }
}