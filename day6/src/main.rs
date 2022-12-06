fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let data = get_data();

    let first = get_pos_first_non_dup(data, 14);

    println!("{:#?}", first);
}

fn part_one() {
    let data = get_data();

    let first = get_pos_first_non_dup(data, 4);

    println!("{:#?}", first);
}

fn get_pos_first_non_dup(data: String, dis_char_len: usize) -> Option<usize> {
    let data_chars = data.chars().collect::<Vec<char>>();

    data_chars
        .iter()
        .enumerate()
        .filter(|(i, _)| *i < data_chars.len() - dis_char_len)
        .find(|(i, _)| {
            let els = data_chars
                .iter()
                .skip(*i)
                .take(dis_char_len)
                .cloned()
                .collect();

            !has_dups(els)
        })
        .map(|(i, _)| i + dis_char_len)
}

fn has_dups(seq: Vec<char>) -> bool {
    let mut uniques: Vec<char> = Vec::new();

    for c in seq {
        if !uniques.contains(&c) {
            uniques.push(c);
        } else {
            return true;
        }
    }

    false
}

fn get_data() -> String {
    include_str!("data.txt").to_owned()
}

#[test]
fn non_dup() {
    assert!(!has_dups(vec!['a', 'b', 'c', 'd']));
    assert!(has_dups(vec!['a', 'a', 'c', 'd']));
    assert_eq!(
        get_pos_first_non_dup(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 4),
        Some(5)
    );
    assert_eq!(
        get_pos_first_non_dup(String::from("nppdvjthqldpwncqszvftbrmjlhg"), 4),
        Some(6)
    );
}

fn non_dup_two() {
    assert_eq!(
        get_pos_first_non_dup(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 14),
        Some(23)
    );
    assert_eq!(
        get_pos_first_non_dup(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 14),
        Some(19)
    );
}
