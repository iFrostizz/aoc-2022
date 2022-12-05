fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let parsed_input = parse_puzzle_input();

    let ret = use_cargo_crane(parsed_input);

    let top = ret
        .iter()
        .filter(|stack| stack.last().is_some())
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    println!("{:#?}", top);
}

fn part_two() {
    let parsed_input = parse_puzzle_input();

    let ret = use_sophisticated_cargo_crane(parsed_input);

    let top = ret
        .iter()
        .filter(|stack| stack.last().is_some())
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    println!("{:#?}", top);
}

fn use_sophisticated_cargo_crane(input: ParsedInput) -> Vec<Vec<char>> {
    let mut return_val: Vec<Vec<char>> = input.init_state;

    input.instructions.into_iter().for_each(|inst| {
        let l_len = return_val[inst.from - 1].len();

        let mut drained = return_val[inst.from - 1]
            .drain(l_len - inst.during..)
            .collect::<Vec<char>>();
        return_val[inst.to - 1].append(&mut drained);
    });

    return_val
}

fn use_cargo_crane(input: ParsedInput) -> Vec<Vec<char>> {
    let mut return_val: Vec<Vec<char>> = input.init_state;

    input.instructions.into_iter().for_each(|inst| {
        for _ in 0..inst.during {
            if let Some(popped) = return_val[inst.from - 1].pop() {
                return_val[inst.to - 1].push(popped);
            };
        }
    });

    return_val
}

#[derive(Debug)]
struct Instr {
    during: usize,
    from: usize,
    to: usize,
}

#[derive(Default, Debug)]
struct ParsedInput {
    init_state: Vec<Vec<char>>,
    instructions: Vec<Instr>,
}

fn parse_puzzle_input() -> ParsedInput {
    let data = include_str!("./data.txt");

    // divide the data into init_state and instructions

    let mut raw_init_state: Vec<String> = Vec::new();

    let raw_lines = data.lines().map(|l| l.to_owned()).collect::<Vec<String>>();
    let mut i: usize = 0;
    loop {
        let line = raw_lines[i].clone();
        if line.starts_with(" 1") {
            break;
        } else {
            raw_init_state.push(line.to_owned());
        }

        i += 1;
    }

    let mut inv_init_state: Vec<Vec<Option<char>>> = Vec::new();

    raw_init_state.into_iter().for_each(|l| {
        let line_state = l
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|c| if c[1] != ' ' { Some(c[1]) } else { None })
            .collect::<Vec<Option<char>>>();

        inv_init_state.push(line_state);
    });

    let mut init_state: Vec<Vec<char>> = Vec::new();

    for col_i in 0..inv_init_state[0].len() {
        let mut col_state = Vec::new();

        for lin_i in (0..inv_init_state.len()).rev() {
            if let Some(cr) = inv_init_state[lin_i][col_i] {
                col_state.push(cr);
            }
        }

        init_state.push(col_state);
    }

    let mut lines = data.lines();

    for _ in 0..i + 2 {
        lines.next();
    }

    let raw_lines = lines.map(|l| l.to_owned()).collect::<Vec<String>>();

    let instructions = raw_lines
        .into_iter()
        .map(|l| {
            // let l_chars = l.chars().collect::<Vec<char>>();

            let mut split = l.split_whitespace();

            split.next();
            let during = split.next().unwrap();
            split.next();
            let from = split.next().unwrap();
            split.next();
            let to = split.next().unwrap();

            Instr {
                during: during.parse().unwrap(),
                from: from.parse().unwrap(),
                to: to.parse().unwrap(),
            }
        })
        .collect();

    ParsedInput {
        init_state,
        instructions,
    }

    /*let raw_init_state = data
    .lines()
    .map(|l| l.to_string())
    .try_fold("", |curr_state, line| {
        if line.starts_with(" 1") {
            return curr_state;
        } else {
            // curr_state.push_str(line);

            return (curr_state.to_string() + &line).as_str();
        }
    });*/
}
