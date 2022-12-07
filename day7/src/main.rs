use std::{collections::HashMap, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;
    let fs = parse(&input);
    let sizes = list_directory_size(&fs);

    println!("part 1: {:?}", part1(&sizes));
    println!("part 2: {:?}", part2(sizes));

    Ok(())
}

fn part1(sizes: &[usize]) -> usize {
    sizes.iter().filter(|&&s| s <= 100_000).sum()
}

fn part2(mut sizes: Vec<usize>) -> usize {
    sizes.sort_unstable();

    let &root_size = sizes.last().unwrap();
    sizes
        .into_iter()
        .find(|&s| (30_000_000 - (70_000_000 - root_size)) <= s)
        .unwrap()
}

fn list_directory_size(fs: &Filesystem) -> Vec<usize> {
    fs.keys().map(|cwd| directory_size(fs, cwd)).collect()
}

fn directory_size(fs: &Filesystem, cwd: &str) -> usize {
    let dir = fs.get(cwd).unwrap();
    let file_sizes: usize = dir.files.iter().sum();
    let sub_directory_sizes = dir
        .children
        .iter()
        .map(|c| path_add(cwd, c))
        .map(|e| directory_size(fs, &e))
        .sum::<usize>();

    file_sizes + sub_directory_sizes
}

fn parse(input: &str) -> Filesystem {
    let input = input.split('$').skip(1).map(|l| l.trim());
    let mut cwd = Vec::new();
    let mut fs: Filesystem = HashMap::new();

    for l in input {
        match l.split_ascii_whitespace().collect::<Vec<_>>()[..] {
            ["cd", ".."] => {
                cwd.pop();
            }
            ["cd", dir_name] => {
                cwd.push(dir_name.to_string());
                let cwd_path = path(&cwd);
                let dir = Directory::new();
                fs.insert(cwd_path, dir);
            }
            // ls command
            _ => {
                let cwd_path = path(&cwd);
                let current_directory = fs.get_mut(&cwd_path).unwrap();

                for ll in l.lines().skip(1) {
                    if ll.starts_with("dir") {
                        let sub_dir_name = ll.split_ascii_whitespace().nth(1).unwrap().to_string();
                        current_directory.children.push(sub_dir_name)
                    } else {
                        let size = ll
                            .split_ascii_whitespace()
                            .next()
                            .unwrap_or_else(|| panic!("Cannot read file size: {:?}", ll))
                            .parse::<usize>()
                            .expect("Cannot parse file size as an integer");
                        current_directory.files.push(size);
                    }
                }
            }
        }
    }

    fs
}

type Filesystem = HashMap<String, Directory>;

#[derive(Debug)]
struct Directory {
    files: Vec<usize>,
    children: Vec<String>,
}

impl Directory {
    fn new() -> Self {
        Directory {
            files: Vec::new(),
            children: Vec::new(),
        }
    }
}

const SEPARATOR: &'static str = ">";

fn path(cwd: &[String]) -> String {
    cwd.join(SEPARATOR)
}

fn path_add(cwd: &str, dir: &str) -> String {
    [cwd, dir].join(SEPARATOR)
}
