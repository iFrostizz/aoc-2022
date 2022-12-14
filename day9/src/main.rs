fn main() {
    part_one();
}

type Pos = [[bool; 6]; 5];

#[derive(Debug)]
enum Motion {
    Left,
    Right,
    Up,
    Down,
}

type Step = (Motion, usize);

type Moves = Vec<Step>;

fn part_one() {
    let head: Pos = [
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [true, false, false, false, false, false],
    ];

    let moves = get_moves();
    let visited: Pos = get_visited_worm(head, moves);

    let amount = get_amount_visited(&visited);

    println!("{}", amount);
}

fn get_moves() -> Moves {
    let data = include_str!("data.txt");

    data.lines()
        .map(|l| {
            let raw = l.split_once(' ').unwrap();

            let mot = match raw.0 {
                "R" => Motion::Right,
                "L" => Motion::Left,
                "D" => Motion::Down,
                "U" => Motion::Up,
                _ => unreachable!(),
            };

            let val = usize::from_str_radix(raw.1, 10).unwrap();

            (mot, val)
        })
        .collect()
}

fn get_amount_visited(visited: &Pos) -> usize {
    visited
        .into_iter()
        .flat_map(|l| l.into_iter().filter(|el| *el == &true))
        .count()
}

/// Returns a map of visited places
fn get_visited_worm(head_start: Pos, moves: Moves) -> Pos {
    // let mut visited: Pos = [[false; 6]; 6];
    let mut visited = head_start;

    // start at same val
    let mut head_pos = head_start;
    let mut tail_pos = head_start;

    for (i, m) in moves.iter().enumerate() {
        head_pos = move_one_step(&head_pos, m);

        if i > 0 {
            tail_pos = move_follow_tail(&tail_pos, &head_pos, m);
        }

        visited = or_assign(visited, tail_pos);
    }

    visited
}

fn move_follow_tail(tail: &Pos, head: &Pos, step: &Step) -> Pos {
    // 1. make sure that the tail is aligned with the head
    // 2. follow

    let (t_x, t_y) = pos_x_y(tail).unwrap();
    let (h_x, h_y) = pos_x_y(head).unwrap();

    let (x, y) = match step.0 {
        // vertical
        Motion::Up | Motion::Down => (h_x, t_y),
        // horizontal
        Motion::Left | Motion::Right => (t_x, h_y),
    };

    let adjusted_tail = set_pos(x, y);
    let end_tail = move_one_step(&adjusted_tail, step);

    end_tail
}

fn set_pos(x: usize, y: usize) -> Pos {
    let mut pos = [[false; 6]; 5];
    pos[y][x] = true;

    pos
}

fn move_one_step(from: &Pos, step: &Step) -> Pos {
    let mut to: Pos = [[false; 6]; 5];

    let mut from_pos = pos_x_y(from).unwrap();

    // is y, x
    to[from_pos.1][from_pos.0] = false;

    match step.0 {
        Motion::Right => from_pos.0 += step.1,
        Motion::Left => from_pos.0 -= step.1,
        Motion::Down => from_pos.1 += step.1,
        Motion::Up => from_pos.1 -= step.1,
    }

    to[from_pos.1 % 6][from_pos.0 % 6] = true;

    to
}

/// returns (x, y) of the *only* "true" in the map
fn pos_x_y(pos: &Pos) -> Option<(usize, usize)> {
    for (y, line) in pos.iter().enumerate() {
        for (x, col) in line.iter().enumerate() {
            if *col {
                return Some((x, y));
            }
        }
    }

    None
}

fn or_assign(from: Pos, to: Pos) -> Pos {
    let mut pos = from;

    for (i, _) in from.iter().enumerate() {
        for (j, _) in to.iter().enumerate() {
            let val = from[i][j] | to[i][j];
            pos[i][j] |= val;
        }
    }

    pos
}

#[test]
fn cur_pos() {
    let curr = [
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [true, false, false, false, false, false],
    ];

    let pos = pos_x_y(&curr).unwrap();

    pretty_assertions::assert_eq!(pos, (0, 4));
}

#[test]
fn test_step() {
    #[rustfmt::skip]
    let from: Pos = [
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [true, false, false, false, false, false],
    ];

    let step = (Motion::Right, 2);

    let new_map = move_one_step(&from, &step);

    pretty_assertions::assert_eq!(
        new_map,
        [
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, true, false, false, false],
        ]
    );

    pretty_assertions::assert_eq!(
        move_one_step(&new_map, &(Motion::Up, 4)),
        [
            [false, false, true, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
        ]
    );
}

#[test]
fn test_big_step() {
    let head: Pos = [
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [true, false, false, false, false, false],
    ];

    let pos = move_one_step(&head, &(Motion::Right, 10));

    pretty_assertions::assert_eq!(
        pos,
        [
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, true, false],
        ]
    );
}

#[test]
fn test_follow_tail() {
    let head: Pos = [
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, true, false, false, false],
    ];

    let m = (Motion::Right, 1);

    let head = move_one_step(&head, &m);

    let mut tail = head;
    tail[4][2] = false;
    tail[4][1] = true;

    let tail = move_follow_tail(&tail, &head, &m);

    pretty_assertions::assert_eq!(
        tail,
        [
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, true, false, false, false],
        ]
    );

    let m = (Motion::Down, 4);

    let head = move_one_step(&head, &m);

    // sanity check
    pretty_assertions::assert_eq!(
        head,
        [
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, true, false, false],
            [false, false, false, false, false, false],
        ]
    );

    let tail = move_follow_tail(&tail, &head, &m);

    pretty_assertions::assert_eq!(
        tail,
        [
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, false, true, false, false],
            [false, false, false, false, false, false],
        ]
    );
}

#[test]
fn test_steps() {
    let head: Pos = [
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [true, false, false, false, false, false],
    ];

    #[rustfmt::skip]
    let moves: Moves = vec![
        (Motion::Right, 4),
        (Motion::Up, 4),
        (Motion::Left, 3),
        (Motion::Down, 1),
        (Motion::Right, 4),
        (Motion::Down, 1),
        (Motion::Left, 5),
        (Motion::Right, 2),
    ];

    let mut last_pos = head;

    for m in moves {
        last_pos = move_one_step(&last_pos, &m);
    }

    #[rustfmt::skip]
    pretty_assertions::assert_eq!(last_pos, [
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, true, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
    ]);
}

#[test]
fn test_visited() {
    let visited = [
        [false, false, true, true, false, false],
        [false, false, false, true, true, false],
        [false, true, true, true, true, false],
        [false, false, false, false, true, false],
        [true, true, true, true, false, false],
    ];

    let amount = get_amount_visited(&visited);

    assert_eq!(amount, 13);
}

#[test]
fn test_init() {
    let head: Pos = [
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [false, false, false, false, false, false],
        [true, false, false, false, false, false],
    ];

    #[rustfmt::skip]
    let moves: Moves = vec![
        (Motion::Right, 4),
        (Motion::Up, 4),
        (Motion::Left, 3),
        (Motion::Down, 1),
        (Motion::Right, 4),
        (Motion::Down, 1),
        (Motion::Left, 5),
        (Motion::Right, 2),
    ];

    let visited: Pos = get_visited_worm(head, moves);

    pretty_assertions::assert_eq!(
        visited,
        [
            [false, false, true, true, false, false],
            [false, false, false, true, true, false],
            [false, true, true, true, true, false],
            [false, false, false, false, true, false],
            [true, true, true, true, false, false],
        ]
    );
}
