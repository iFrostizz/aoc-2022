fn main() {
    part_one();
    part_two();
}

#[derive(Debug, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

fn part_one() {
    let data = include_str!("./data.txt");

    let total_score = calculate_score(get_rounds(data));

    println!("{}", total_score);
}

enum Final {
    Lose,
    Draw,
    Win,
}

fn part_two() {
    let data = include_str!("./data.txt");

    let rounds_with_final = get_final_rounds(data);
    let rounds = get_rounds_from_final(rounds_with_final);
    let total_score = calculate_score(rounds);

    println!("{}", total_score);
}

fn calculate_score(rounds: Vec<(RPS, RPS)>) -> u32 {
    rounds.iter().fold(0, |tot, round| {
        let score_shape = match round.1 {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissor => 3,
        };

        let score_t = match round.0 {
            RPS::Rock => match round.1 {
                RPS::Rock => 3,
                RPS::Paper => 6,
                RPS::Scissor => 0,
            },
            RPS::Paper => match round.1 {
                RPS::Rock => 0,
                RPS::Paper => 3,
                RPS::Scissor => 6,
            },
            RPS::Scissor => match round.1 {
                RPS::Rock => 6,
                RPS::Paper => 0,
                RPS::Scissor => 3,
            },
        };

        tot + score_shape + score_t
    })
}

fn get_rounds(data: &str) -> Vec<(RPS, RPS)> {
    data.lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|pair| {
            (
                match pair[0] {
                    "A" => RPS::Rock,
                    "B" => RPS::Paper,
                    "C" => RPS::Scissor,
                    _ => unimplemented!(),
                },
                match pair[1] {
                    "X" => RPS::Rock,
                    "Y" => RPS::Paper,
                    "Z" => RPS::Scissor,
                    _ => unimplemented!(),
                },
            )
        })
        .collect()
}

fn get_final_rounds(data: &str) -> Vec<(RPS, Final)> {
    data.lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|pair| {
            (
                match pair[0] {
                    "A" => RPS::Rock,
                    "B" => RPS::Paper,
                    "C" => RPS::Scissor,
                    _ => unimplemented!(),
                },
                match pair[1] {
                    "X" => Final::Lose,
                    "Y" => Final::Draw,
                    "Z" => Final::Win,
                    _ => unimplemented!(),
                },
            )
        })
        .collect()
}

fn get_rounds_from_final(rounds: Vec<(RPS, Final)>) -> Vec<(RPS, RPS)> {
    rounds
        .into_iter()
        .map(|round| {
            (
                round.0,
                match round.0 {
                    RPS::Rock => match round.1 {
                        Final::Lose => RPS::Scissor,
                        Final::Draw => RPS::Rock,
                        Final::Win => RPS::Paper,
                    },
                    RPS::Paper => match round.1 {
                        Final::Lose => RPS::Rock,
                        Final::Draw => RPS::Paper,
                        Final::Win => RPS::Scissor,
                    },
                    RPS::Scissor => match round.1 {
                        Final::Lose => RPS::Paper,
                        Final::Draw => RPS::Scissor,
                        Final::Win => RPS::Rock,
                    },
                },
            )
        })
        .collect()
}
