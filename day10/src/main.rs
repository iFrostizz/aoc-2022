use std::collections::HashMap;

fn main() {
    part_one();
}

#[derive(PartialEq, Eq)]
enum Op {
    No,
    Add(i64),
}

#[derive(Debug)]
enum Pixel {
    Lit,
    Dark,
}

fn get_data() -> Vec<Op> {
    let raw_data = include_str!("data.txt");

    parse_raw_data(raw_data)
}

fn parse_raw_data(data: &str) -> Vec<Op> {
    data.lines()
        .map(|l| {
            // let l = l.to_owned();
            if l == "noop" {
                Op::No
            } else {
                let num = l.split_once(' ').unwrap().1;
                let num = num.parse().unwrap();
                Add(num)
            }
        })
        .collect()
}

fn part_one() {
    let data = get_data();

    let x = get_signal_strength(&data, vec![20, 60, 100, 140, 180, 220]);

    println!("{}", x);
}

fn cycle_instructions(data: Vec<Op>) -> i64 {
    let mut x = 1;

    data.iter().for_each(|op| {
        if let &Op::Add(num) = op {
            x += num;
        }
    });

    x
}

fn get_signal_strength(data: &Vec<Op>, cycles: Vec<i64>) -> i64 {
    let mut x = 1;
    let mut cycle: i64 = 0;
    let mut strength: i64 = 0;

    // remember the last cycles to complete
    let mut buffer: HashMap<i64, i64> = HashMap::new();

    data.iter().for_each(|op| {
        if let &Op::Add(num) = op {
            buffer.insert(cycle + 2, num);

            cycle += 1;

            strength += cycle_signal_strength(x, cycle, &cycles);
            x += check_with_buffer(cycle, &buffer);

            cycle += 1;

            strength += cycle_signal_strength(x, cycle, &cycles);
            x += check_with_buffer(cycle, &buffer);
        } else {
            cycle += 1;

            strength += cycle_signal_strength(x, cycle, &cycles);
            x += check_with_buffer(cycle, &buffer);
        }
    });

    strength
}

fn get_x_at_cycle(data: &Vec<Op>, cycles: i64) -> i64 {
    let mut x = 1;
    let mut cycle: i64 = 0;

    // remember the last cycles to complete
    let mut buffer: HashMap<i64, i64> = HashMap::new();

    for op in data {
        if let &Op::Add(num) = op {
            buffer.insert(cycle + 2, num);

            cycle += 1;

            if cycle == cycles {
                return x;
            }
            x += check_with_buffer(cycle, &buffer);

            cycle += 1;
            if cycle == cycles {
                return x;
            }
            x += check_with_buffer(cycle, &buffer);
        } else {
            cycle += 1;
            if cycle == cycles {
                return x;
            }
            x += check_with_buffer(cycle, &buffer);
        }
    }

    x
}

fn check_with_buffer(cycle: i64, buffer: &HashMap<i64, i64>) -> i64 {
    if let Some(val) = buffer.get(&cycle) {
        *val
    } else {
        0
    }
}

fn cycle_signal_strength(x: i64, cycle: i64, cycles: &Vec<i64>) -> i64 {
    if cycles.contains(&cycle) {
        dbg!(x, cycle);
        cycle * x
    } else {
        0
    }
}

fn make_sprite(data: &Vec<Op>) -> Sprite {
    let sprite = Vec::new();

    sprite
}

type Sprite = Vec<[Pixel; 40]>;

/*fn str_to_sprite(entry: &str) -> Sprite {
    entry
        .lines()
        .map(|l| {
            assert_eq!(l.len(), 40);
            l.chars()
                .into_iter()
                .map(|c| match c {
                    '.' => Pixel::Dark,
                    '#' => Pixel::Lit,
                })
                .collect()
        })
        .collect()
}*/

fn sprite_to_str(entry: Sprite) -> String {
    entry
        .iter()
        .map(|l| {
            l.into_iter()
                .map(|c| match c {
                    Pixel::Dark => '.',
                    Pixel::Lit => '#',
                })
                .collect::<String>()
        })
        .collect()
}

use Op::{Add, No};

#[test]
fn test_smoll() {
    let data = vec![No, Add(3), Add(-5)];

    assert_eq!(get_x_at_cycle(&data, 6), -1);
}

#[test]
fn test_big() {
    let data = parse_raw_data(
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
    );

    assert_eq!(get_x_at_cycle(&data, 20), 21);
    assert_eq!(get_signal_strength(&data, vec![20]), 420);

    assert_eq!(get_signal_strength(&data, vec![60]), 1140);

    assert_eq!(get_signal_strength(&data, vec![100]), 1800);

    assert_eq!(get_signal_strength(&data, vec![140]), 2940);

    assert_eq!(get_x_at_cycle(&data, 180), 16);
    assert_eq!(get_signal_strength(&data, vec![180]), 2880);

    assert_eq!(get_x_at_cycle(&data, 220), 18);
    assert_eq!(get_signal_strength(&data, vec![220]), 3960);

    let x = get_signal_strength(&data, vec![20, 60, 100, 140, 180, 220]);
    assert_eq!(x, 13140);
}

// 2
#[test]
fn test_first_sprite() {
    let data = parse_raw_data(
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
    );
    let screen = make_sprite(&data);

    let desired_screen = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    assert_eq!(sprite_to_str(screen), desired_screen);
}
