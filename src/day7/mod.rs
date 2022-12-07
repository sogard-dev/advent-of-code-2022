use regex::Regex;

pub fn main() {
    println!("Day7");
}

fn parse(s: &str) -> Directory {
    let mut root = Directory {
        name: "".to_string(),
        files: vec![],
        directories: vec![],
    };

    let mut current_path: Vec<String> = vec!["/".to_string()];

    let mut input_iter = s.lines().into_iter();

    let command_indicator = "$";

    let mut next_line = input_iter.next();
    loop {
        if let Some(line) = next_line {
            //println!("Command parsing: {}", line);

            let split: Vec<&str> = line.split(" ").collect();
            if split[0].eq(command_indicator) {
                let command = split[1];
                if command.eq("cd") {
                    let cd_path = split[2];
                    if cd_path.eq("/") {
                        current_path.drain(1..);
                    } else if cd_path.eq("..") {
                        current_path.pop();
                    } else {
                        current_path.push(cd_path.to_string());
                    }

                    next_line = input_iter.next();
                } else if command.eq("ls") {
                    loop {
                        next_line = input_iter.next();
                        if let Some(output_line) = next_line {
                            //println!("Output parsing: {}", output_line);

                            if output_line.starts_with(command_indicator) {
                                break;
                            }

                            let ls_output_split: Vec<&str> = output_line.split(" ").collect();

                            if !ls_output_split[0].eq("dir") {
                                let size = ls_output_split[0].parse::<usize>().unwrap();
                                let name = ls_output_split[1].to_string();

                                let mut node = &mut root;
                                for name in &current_path {
                                    node.create(name);
                                    node = node.get(name);
                                }
                                node.add_file(name, size);
                            }
                        } else {
                            break;
                        }
                    }
                } else {
                    panic!("Unknown command: {}", command);
                }
            } else {
                panic!("Not a command?: {}", line);
            }
        } else {
            break;
        }
    }

    //println!("Parsing done");
    root
}

struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}
impl Directory {
    fn add_subdir(&mut self, name: &str) {
        let new_directory = Directory {
            name: name.to_string(),
            files: vec![],
            directories: vec![],
        };
        self.directories.push(new_directory);
    }

    fn create(&mut self, name: &str) {
        match self.directories.iter().find(|d| d.name.eq(name)) {
            None => {
                let dir = Directory {
                    name: name.to_string(),
                    files: vec![],
                    directories: vec![],
                };
                self.directories.push(dir);
            }
            _ => {}
        }
    }

    fn get(&mut self, name: &str) -> &mut Directory {
        self.directories
            .iter_mut()
            .find(|d| d.name.eq(name))
            .unwrap()
    }

    fn add_file(&mut self, name: String, size: usize) {
        match self.files.iter().find(|f| f.name.eq(&name)) {
            None => self.files.push(File { name, size }),
            _ => {}
        }
    }
}

struct File {
    name: String,
    size: usize,
}

fn problem1(s: &str) -> usize {
    let root = parse(s);
    let mut num = Box::new(0 as usize);
    find_below(&root, 100000, &mut num);

    *num
}

fn problem2(s: &str) -> usize {
    let root = parse(s);

    let disk_space = 70000000 as usize;
    let required_unused = 30000000 as usize;
    let total_used = print(&root, 1);
    let current_unused = disk_space - total_used;
    let space_to_delete = required_unused - current_unused;

    println!("-------------------------------");
    println!("Disk space: {}", disk_space);
    println!("Required unused: {}", required_unused);
    println!("Used: {}", total_used);
    println!("Not used: {}", current_unused);
    println!("To delete: {}", space_to_delete);

    let mut num = Box::new(disk_space as usize);
    find_smallest_above(&root, space_to_delete, &mut num);
    *num

}

fn find_below(node: &Directory, at_most: usize, sum: &mut Box<usize>) -> usize {
    let subdir_size: usize = node
        .directories
        .iter()
        .map(|dir| find_below(dir, at_most, sum))
        .sum();

    let files_size: usize = node.files.iter().map(|f| f.size).sum();

    let total_size = subdir_size + files_size;

    if total_size < at_most {
        println!("Include dir: {} of size {}", node.name, total_size);
        let v = **sum + total_size;
        **sum = v;
    } 

    total_size
}

fn find_smallest_above(node: &Directory, at_most: usize, sum: &mut Box<usize>) -> usize {
    let subdir_size: usize = node
        .directories
        .iter()
        .map(|dir| find_smallest_above(dir, at_most, sum))
        .sum();

    let files_size: usize = node.files.iter().map(|f| f.size).sum();

    let total_size = subdir_size + files_size;

    if total_size > at_most {
        println!("Delete candidate dir: {} of size {}", node.name, total_size);

        if **sum > total_size {
            println!("Setting dir: {} of size {}", node.name, total_size);

            **sum = total_size;
        }
    } 

    total_size
}

fn print(node: &Directory, indent: usize) -> usize {
    println!("{}- {} (dir)", format!("{:indent$}", ""), node.name);

    let subdir_size: usize = node
        .directories
        .iter()
        .map(|dir| print(dir, indent + 1))
        .sum();

    let indent_file = indent + 1;
    let files_size: usize = node
        .files
        .iter()
        .map(|f| {
            println!(
                "{}- {} (file, size={})",
                format!("{:indent_file$}", ""),
                f.name,
                f.size
            );

            f.size
        })
        .sum();

    subdir_size + files_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(95437, problem1(include_str!("test_puzzle.txt")));
        assert_eq!(1367870, problem1(include_str!("puzzle.txt")));

        assert_eq!(24933642, problem2(include_str!("test_puzzle.txt")));
        assert_eq!(549173, problem2(include_str!("puzzle.txt")));
    }
}
