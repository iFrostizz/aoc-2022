fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let haystacks = get_vec_data();

    let p_sum: u32 = haystacks.into_iter().filter(|h| !h.is_empty())
    .map(|haystack| {
        assert!(haystack.len() % 2 == 0);
        let mid = haystack.len() / 2;

        let haystack_chars = haystack.chars();
        let (h1, h2) = (haystack_chars.clone().take(mid), haystack_chars.skip(mid));

        let h1_chars = h1.collect::<Vec<char>>();
        let mut i = 0;

        let h2 = h2.collect::<Vec<char>>();

        let common_item = loop {
            let item = h1_chars[i];
            if h2.contains(&item) {
                break Some(item);
            }

            i += 1
        }.unwrap_or(' ');

        char_as_priority(common_item) as u32
    }).sum();

    println!("{}", p_sum);
}

fn part_two() {
    let vec_data: Vec<String> = get_vec_data().into_iter()
        .filter(|l| !l.is_empty()).collect() ;

    // sanity check
    assert_eq!(vec_data.len() % 3, 0);

    let p: u32 = vec_data.chunks(3).map(|r_group| {
        let r1_chars = r_group[0].chars().collect::<Vec<char>>();
        let r2_chars = r_group[1].chars().collect::<Vec<char>>();
        let r3_chars = r_group[2].chars().collect::<Vec<char>>();

        let mut i: usize = 0;

        let p_char = loop {
            let c1 = r1_chars[i];
            if r2_chars.contains(&c1) && r3_chars.contains(&c1) {
                break Some(c1)
            }

            i += 1;
        }.unwrap_or(' ');

        char_as_priority(p_char) as u32
    }).sum();

    println!("{}", p);
}

fn get_vec_data() -> Vec<String> {
    include_str!("./data.txt").lines().map(|l| l.to_string()).collect()
}

fn char_as_priority(c: char) -> u16 {
    if c.is_ascii_lowercase() {
        return c as u16 - 97 + 1
    } else {
        return c as u16 - 65 + 27
    }
}
