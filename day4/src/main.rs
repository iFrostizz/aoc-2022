fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = get_data();

    let contained_pairs = data
        .into_iter()
        .filter(|l| !l.is_empty())
        .filter(|l| {
            let pair: Vec<&str> = l.split(',').collect();
            assert_eq!(pair.len(), 2);

            let (pair1, pair2) = (get_bounds(pair[0]), get_bounds(pair[1]));

            if (pair1.0 <= pair2.0 && pair1.1 >= pair2.1) || (pair1.0 >= pair2.0 && pair1.1 <= pair2.1) {
                true
            } else {
                false
            }
        })
        .collect::<Vec<String>>();

    println!("{}", contained_pairs.len());
}

fn part_two() {
    let data = get_data();

    let contained_pairs = data
        .into_iter()
        .filter(|l| !l.is_empty())
        .filter(|l| {
            let pair: Vec<&str> = l.split(',').collect();
            assert_eq!(pair.len(), 2);

            let (pair1, pair2) = (get_bounds(pair[0]), get_bounds(pair[1]));

            if pair1.0 <= pair2.1 && pair1.1 >= pair2.0 {
                true
            } else {
                false
            }
        })
        .collect::<Vec<String>>();

    println!("{}", contained_pairs.len());

}

fn get_data() -> Vec<String> {
    include_str!("data.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn get_bounds(pair: &str) -> (u16, u16) {
    let bounds: Vec<&str> = pair.split('-').collect();
    assert_eq!(bounds.len(), 2);

    (bounds[0].parse().unwrap(), bounds[1].parse().unwrap())
}
