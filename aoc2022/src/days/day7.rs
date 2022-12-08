use std::{collections::HashMap, str::FromStr};

const INPUT: &str = "input/day7.txt";

pub fn solution() {
    let terminal: Vec<Terminal> = input::parse_lines(INPUT);

    // First, build the filesystem in a HashMap.
    let mut fs: HashMap<String, Vec<File>> = HashMap::new();
    {
        let mut cwd = Path::default();
        for t in terminal {
            match t {
                Terminal::Cd(dir) => {
                    cwd.push(dir.clone());
                }
                Terminal::Ls => (), // Ignored.
                Terminal::File(File::Dir(dir)) => {
                    fs.entry(cwd.path())
                        .or_default()
                        .push(File::Dir(cwd.join(dir).path()));
                }
                Terminal::File(File::File(file, size)) => {
                    fs.entry(cwd.path())
                        .or_default()
                        .push(File::File(file, size));
                }
            }
        }
    }

    // Now build a map of directory -> size.
    let mut sizes: HashMap<&str, u64> = HashMap::new();
    {
        let mut stack = vec!["/"];
        while let Some(path) = stack.pop() {
            let listing = fs.get(path).unwrap();
            // Try to get the size of this directory using the sizes we have so far.
            let subs: Option<Vec<u64>> = listing
                .iter()
                .map(|f| match f {
                    File::Dir(dir) => sizes.get(dir.as_str()).copied(),
                    File::File(_, size) => Some(*size),
                })
                .collect();
            match subs {
                // We were successful! Add this directory to the map.
                Some(subs) => {
                    sizes.insert(path, subs.iter().sum());
                }
                // We don't have enough info. Re-add this path to the stack
                // and then push sub-directories so we process them first.
                None => {
                    stack.push(path);
                    for l in listing.iter() {
                        if let File::Dir(dir) = l {
                            stack.push(dir);
                        }
                    }
                }
            }
        }
    }

    let part1: u64 = sizes.values().copied().filter(|f| f <= &100000).sum();
    println!("Part1: {}", part1);

    let available = 70000000 - sizes.get("/").unwrap();
    let need = 30000000 - available;
    let part2: u64 = sizes
        .values()
        .copied()
        .filter(|f| f >= &need)
        .min()
        .unwrap();
    println!("Part2: {}", part2);
}

#[derive(Debug, Default, Clone)]
struct Path(Vec<String>);

impl Path {
    /// Both push and join assume a single path segment.
    /// I.e. push("foo/bar") is unsupported.
    fn push(&mut self, dir: String) {
        if dir == ".." {
            self.0.pop().expect("path is empty!");
            return;
        }
        if let Some(dir) = dir.strip_prefix('/') {
            self.0.clear();
            if !dir.is_empty() {
                self.0.push(dir.to_owned());
            }
            return;
        }
        self.0.push(dir);
    }

    fn join(&self, dir: String) -> Path {
        let mut p = self.clone();
        p.push(dir);
        p
    }

    fn path(&self) -> String {
        "/".to_owned() + &self.0.join("/")
    }
}

#[derive(Debug)]
enum Terminal {
    Cd(String),
    Ls,
    File(File),
}

#[derive(Debug)]
enum File {
    Dir(String),
    File(String, u64),
}

impl FromStr for Terminal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(dir) = s.strip_prefix("$ cd ") {
            Ok(Terminal::Cd(dir.to_owned()))
        } else if s == "$ ls" {
            Ok(Terminal::Ls)
        } else if let Some(dir) = s.strip_prefix("dir ") {
            Ok(Terminal::File(File::Dir(dir.to_owned())))
        } else {
            let (size, file) = input::split2(s, " ");
            Ok(Terminal::File(File::File(file, size)))
        }
    }
}
