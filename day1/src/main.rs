fn main() {
    first_part();
    second_part();
}

fn first_part() {
    let all_cals = include_str!("./data.txt");

    let biggest: u32 = total_cal_elves(all_cals)
        .into_iter()
        .reduce(|max, item| if item > max { item } else { max })
        .unwrap_or(0);

    println!("{}", biggest);
}

fn second_part() {
    let all_cals = include_str!("./data.txt");

    let mut total_cals = total_cal_elves(all_cals);
    total_cals.sort_by(|a, b| b.cmp(a));

    let sum = total_cals[0] + total_cals[1] + total_cals[2];

    println!("{}", sum);
}

fn total_cal_elves(all_cals: &str) -> Vec<u32> {
    all_cals
        .split("\n\n")
        .map(|l| l.to_string())
        .map(|cal_chunk| {
            cal_chunk
                .lines()
                .filter(|line| !line.is_empty())
                .fold(0, |tot_cal, cal| tot_cal + cal.parse::<u32>().unwrap())
        })
        .collect()
}
