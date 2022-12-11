use std::collections::HashMap;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = get_data();
    let s = get_size_under(data, 100000);

    println!("{}", s);
}

fn part_two() {
    let data = get_data();
    let s = get_closest_dir(data);

    println!("{}", s);
}

fn get_all_dir(data: Vec<String>) -> HashMap<String, u64> {
    let mut dirs = HashMap::new();
    let mut path = String::new();

    data.iter().for_each(|l| {
        let mut components = l.split(' ');
        match (
            components.next().unwrap(),
            components.next().unwrap(),
            components.next(),
        ) {
            // nothing
            ("$", "cd", Some("/")) => {
                path = "/".to_owned();
            }
            ("$", "cd", Some("..")) => {
                // going out of dir
                path = format!(
                    "{}/",
                    path.split('/')
                        .rev()
                        .skip(2)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .collect::<Vec<_>>()
                        .join("/")
                );
            }
            ("$", "cd", Some(name)) => {
                // all next files will add size to this dir
                path = format!("{path}{name}/");
            }
            ("$", "ls", None) => (),
            ("dir", _name, None) => (),
            (size, _file, None) => {
                let entry = dirs.entry(path.to_owned()).or_insert(0);
                let s = size.parse::<u64>().unwrap();
                *entry += s;

                // also update parents
                let mut curr_path = path.clone();

                while curr_path.len() > 1 {
                    curr_path = format!(
                        "{}/",
                        curr_path
                            .split('/')
                            .rev()
                            .skip(2)
                            .collect::<Vec<_>>()
                            .into_iter()
                            .rev()
                            .collect::<Vec<_>>()
                            .join("/")
                    );

                    let entry = dirs.entry(curr_path.to_owned()).or_insert(0);
                    let s = size.parse::<u64>().unwrap();
                    *entry += s;
                }
            }
            _ => unreachable!(),
        }
    });

    // println!("{:#?}", dirs);

    dirs
}

fn get_size_under(data: Vec<String>, under: u64) -> u64 {
    get_all_dir(data)
        .iter()
        .map(|(_, size)| size)
        .filter(|size| *size <= &under)
        .sum()
}

fn get_closest_dir(data: Vec<String>) -> u64 {
    let dir_map = get_all_dir(data.clone());

    let tot_required = 70000000 - 30000000;
    let min_delete = dir_map.get("/").unwrap() - tot_required;

    *get_all_dir(data)
        .iter()
        .map(|(_, size)| size)
        .filter(|size| *size >= &min_delete)
        .min()
        .unwrap()
}

fn get_data() -> Vec<String> {
    include_str!("data.txt")
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

#[test]
fn dir_one() {
    let data = String::from(
        "$ cd /
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
7214296 k",
    );

    let data = data.lines().map(|l| l.to_owned()).collect::<Vec<String>>();

    assert_eq!(get_size_under(data, 100000), 95437);
}
