fn main() {
    part_one();
}

type Map = [[bool; 6]; 5];

#[derive(Debug, PartialEq)]
enum Motion {
    Left,
    Right,
    Up,
    Down,
}

type Step = (Motion, usize);

type Moves = Vec<Step>;

fn part_one() {
    let data = get_moves();

    let tail_visited = get_tail_visited(&data);

    println!("{}", tail_visited);
}

fn get_tail_visited(moves: &Moves) -> usize {
    let map = move_on_map(moves);
    map.into_iter()
        .flat_map(|line| line.into_iter().filter(|el| *el == true))
        .filter(|el| *el == true)
        .count()
}

fn move_on_map(moves: &Moves) -> Map {
    let mut map = [[false; 6]; 5];
    map[4][0] = true;

    let mut head_pos = Position::default();
    let mut tail_pos = Position::default();

    move_steps(moves, &mut map, &mut head_pos, &mut tail_pos);

    map
}

fn move_steps(moves: &Moves, map: &mut Map, head_pos: &mut Position, tail_pos: &mut Position) {
    for m in moves {
        let motion = &m.0;
        let steps = &m.1;

        // println!("{:?}", m);

        for _ in 0..*steps {
            move_step(motion, head_pos, tail_pos);
            // println!("H {:?} T {:?}", head_pos, tail_pos);

            assert!(head_pos.x < 6);
            assert!(head_pos.y < 5);
            assert!(tail_pos.x < 6);
            assert!(tail_pos.y < 5);

            map[tail_pos.y][tail_pos.x] = true;
        }
    }
}

#[derive(Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 4 }
    }
}

impl Motion {
    fn is_horizontal(&self) -> bool {
        *self == Motion::Left || *self == Motion::Right
    }
}

fn move_step(motion: &Motion, head_pos: &mut Position, tail_pos: &mut Position) {
    // move the head
    match motion {
        Motion::Right => {
            head_pos.x = (head_pos.x + 1) % 6;
        }
        Motion::Left => {
            head_pos.x = head_pos.x.checked_sub(1).unwrap_or(5) % 6;
        }
        Motion::Down => {
            head_pos.y = (head_pos.y + 1) % 5;
        }
        Motion::Up => {
            head_pos.y = head_pos.y.checked_sub(1).unwrap_or(4) % 5;
        }
    }

    if head_pos.x.abs_diff(tail_pos.x) > 1 || head_pos.y.abs_diff(tail_pos.y) > 1 {
        match motion {
            Motion::Right => {
                tail_pos.x = (tail_pos.x + 1) % 6;
                tail_pos.y = head_pos.y
            }
            Motion::Left => {
                tail_pos.x = tail_pos.x.checked_sub(1).unwrap_or(5) % 6;
                tail_pos.y = head_pos.y
            }
            Motion::Down => {
                tail_pos.x = head_pos.x;
                tail_pos.y = (tail_pos.y + 1) % 5;
            }
            Motion::Up => {
                tail_pos.x = head_pos.x;
                tail_pos.y = tail_pos.y.checked_sub(1).unwrap_or(4) % 5;
            }
        }
    }
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

#[test]
fn test_move() {
    let mut head_pos = Position::default();
    let mut tail_pos = Position::default();

    move_step(&Motion::Up, &mut head_pos, &mut tail_pos);

    pretty_assertions::assert_eq!(head_pos, Position { x: 0, y: 3 });
    pretty_assertions::assert_eq!(tail_pos, Position { x: 0, y: 4 });

    move_step(&Motion::Right, &mut head_pos, &mut tail_pos);

    pretty_assertions::assert_eq!(head_pos, Position { x: 1, y: 3 });
    pretty_assertions::assert_eq!(tail_pos, Position { x: 0, y: 4 });
}

#[test]
fn test_move_one() {
    let mut head_pos = Position::default();
    let mut tail_pos = Position::default();

    let moves = vec![
        (Motion::Right, 4),
        (Motion::Up, 4),
        (Motion::Left, 3),
        (Motion::Down, 1),
        (Motion::Right, 4),
        (Motion::Down, 1),
        (Motion::Left, 5),
        (Motion::Right, 2),
    ];

    let mut map = [[false; 6]; 5];

    move_steps(&moves, &mut map, &mut head_pos, &mut tail_pos);

    pretty_assertions::assert_eq!(head_pos, Position { x: 2, y: 2 });
    pretty_assertions::assert_eq!(tail_pos, Position { x: 1, y: 2 });
}

#[test]
fn test_part_one() {
    let moves = vec![
        (Motion::Right, 4),
        (Motion::Up, 4),
        (Motion::Left, 3),
        (Motion::Down, 1),
        (Motion::Right, 4),
        (Motion::Down, 1),
        (Motion::Left, 5),
        (Motion::Right, 2),
    ];

    let map = move_on_map(&moves);

    pretty_assertions::assert_eq!(
        map,
        [
            [false, false, true, true, false, false],
            [false, false, false, true, true, false],
            [false, true, true, true, true, false],
            [false, false, false, false, true, false],
            [true, true, true, true, false, false],
        ]
    );

    let visited = get_tail_visited(&moves);

    assert_eq!(visited, 13);
}
