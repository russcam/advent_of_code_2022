use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day_7.txt");

static FILE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+) (.*)$").unwrap());

enum Command<'a> {
    Ls,
    Cd(&'a str),
}

impl<'a> Command<'a> {
    pub fn parse(input: &'a str) -> Self {
        if let Some(path) = input.strip_prefix("$ cd ") {
            Self::Cd(path)
        } else {
            Self::Ls
        }
    }
}

enum DirEntry {
    Dir(Dir),
    File(File),
}

impl DirEntry {
    pub fn name(&self) -> &str {
        match self {
            DirEntry::Dir(d) => d.name.as_str(),
            DirEntry::File(f) => f.name.as_str(),
        }
    }

    pub fn parent(&self) -> Option<usize> {
        match self {
            DirEntry::Dir(d) => d.parent,
            DirEntry::File(f) => f.parent,
        }
    }

    pub fn dir(&self) -> Option<&Dir> {
        match self {
            DirEntry::Dir(d) => Some(d),
            DirEntry::File(_) => None,
        }
    }

    pub fn dir_mut(&mut self) -> Option<&mut Dir> {
        match self {
            DirEntry::Dir(d) => Some(d),
            DirEntry::File(_) => None,
        }
    }
}

struct Dir {
    name: String,
    idx: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

struct File {
    name: String,
    parent: Option<usize>,
    size: usize,
}

struct Filesystem {
    entries: Vec<DirEntry>,
    cwd: usize,
}

impl Filesystem {
    pub fn init(input: &str) -> Self {
        let mut file_system = Filesystem::new();
        for line in input.lines() {
            if line.starts_with('$') {
                if let Command::Cd(cd) = Command::parse(line) {
                    file_system.cd(cd);
                }
            } else if let Some(name) = line.strip_prefix("dir ") {
                file_system.add_dir(name);
            } else {
                let caps = FILE_REGEX.captures(line).unwrap();
                file_system.add_file(
                    caps.get(2).unwrap().as_str(),
                    caps.get(1).unwrap().as_str().parse().unwrap(),
                );
            }
        }

        file_system
    }

    fn new() -> Self {
        let idx = 0;
        Self {
            entries: vec![DirEntry::Dir(Dir {
                name: "/".into(),
                idx,
                parent: None,
                children: vec![],
            })],
            cwd: idx,
        }
    }

    pub fn add_dir(&mut self, name: &str) {
        if !self.contains_entry(name) {
            let idx = self.entries.len();
            self.entries.push(DirEntry::Dir(Dir {
                name: name.into(),
                idx,
                parent: Some(self.cwd),
                children: vec![],
            }));
            self.current_dir().children.push(idx);
        }
    }

    pub fn add_file(&mut self, name: &str, size: usize) {
        if !self.contains_entry(name) {
            let idx = self.entries.len();
            self.entries.push(DirEntry::File(File {
                name: name.into(),
                parent: Some(self.cwd),
                size,
            }));
            self.current_dir().children.push(idx);
        }
    }

    pub fn cd(&mut self, path: &str) {
        match path {
            "/" => self.cwd = 0,
            ".." => {
                let entry = self.entries.get(self.cwd).unwrap();
                self.cwd = entry.parent().unwrap();
            }
            _ => {
                let dir = self.entries.get(self.cwd).unwrap().dir().unwrap();
                for child in &dir.children {
                    if let Some(c) = self.entries.get(*child) {
                        if c.name() == path {
                            match c {
                                DirEntry::Dir(child_dir) => {
                                    self.cwd = child_dir.idx;
                                    return;
                                }
                                DirEntry::File(_) => panic!("expected dir but found file"),
                            }
                        }
                    }
                }

                panic!("dir {} does not exist", path);
            }
        }
    }

    pub fn total_size(&self) -> usize {
        let set = self.ls();
        set.iter()
            .filter(|(_, size)| *size <= 100_000)
            .map(|(_, size)| *size)
            .sum()
    }

    pub fn smallest_dir(&self) -> (String, usize) {
        let set = self.ls();

        let total_disk = 70_000_000;
        let used_space = set.iter().find(|(name, _)| name == "/").unwrap().1;
        let unused_space = total_disk - used_space;

        set.into_iter()
            .filter(|(_, size)| *size >= 30_000_000 - unused_space)
            .min_by(|(_, x), (_, y)| (*x).cmp(y))
            .unwrap()
    }

    fn current_dir(&mut self) -> &mut Dir {
        self.entries[self.cwd].dir_mut().unwrap()
    }

    fn contains_entry(&mut self, name: &str) -> bool {
        self.entries[self.cwd]
            .dir()
            .unwrap()
            .children
            .iter()
            .filter_map(|c| self.entries.get(*c))
            .any(|e| e.name() == name)
    }

    fn ls(&self) -> HashSet<(String, usize)> {
        let mut set = HashSet::new();
        self.traverse("", &self.entries[0], &mut set);
        set
    }

    fn traverse(&self, path: &str, entry: &DirEntry, set: &mut HashSet<(String, usize)>) -> usize {
        match entry {
            DirEntry::Dir(dir) => {
                let mut sum = 0;
                let mut child_path = String::from(path);
                if !child_path.is_empty() && &child_path != "/" {
                    child_path.push('/');
                }

                child_path.push_str(&dir.name);
                for child in &dir.children {
                    sum += self.traverse(&child_path, &self.entries[*child], set);
                }
                set.insert((child_path, sum));
                sum
            }
            DirEntry::File(file) => file.size,
        }
    }
}

fn main() {
    let file_system = Filesystem::init(INPUT);
    println!(
        "total size of dirs with at most 100_000: {}",
        file_system.total_size()
    );
    println!("smallest dir: {:?}", file_system.smallest_dir());
}

#[cfg(test)]
mod test {
    use crate::Filesystem;

    const TEST_INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn test_part_1() {
        let file_system = Filesystem::init(TEST_INPUT);
        assert_eq!(95437, file_system.total_size());
    }

    #[test]
    fn test_part_2() {
        let file_system = Filesystem::init(TEST_INPUT);
        assert_eq!(24933642, file_system.smallest_dir().1);
    }
}
