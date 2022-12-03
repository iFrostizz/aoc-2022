fn main() {
    let all_cals = include_str!("./data.txt");

    let biggest: u32 = all_cals
        .split("\n\n")
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .map(|cal_chunk| {
            cal_chunk
                .lines()
                .filter(|line| !line.is_empty())
                .fold(0, |tot_cal, cal| {
                    tot_cal + u32::from_str_radix(cal, 10).unwrap()
                })
        })
        .collect::<Vec<u32>>()
        .into_iter()
        .reduce(|max, item| if item > max { item } else { max })
        .unwrap_or(0);

    println!("{}", biggest);
}
