type Trees = Vec<Vec<u8>>;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let trees = get_data();

    let visible_trees = get_visible_trees(trees);

    println!("Yo I can see {} trees from my drone", visible_trees);
}

fn part_two() {
    let trees = get_data();

    let highest = get_highest_scenic(&trees);

    println!("Highest scenic score is {}", highest);
}

fn get_data() -> Trees {
    let raw = include_str!("data.txt");

    raw.lines()
        .into_iter()
        .map(|l| {
            let l = l.to_string();
            l.chars()
                .into_iter()
                .map(|n| u8::from_str_radix(&n.to_string(), 10).unwrap())
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn get_visible_trees(trees: Trees) -> u32 {
    let mut total = 0;

    for (i, _) in trees.iter().enumerate() {
        for (j, _) in trees[i].iter().enumerate() {
            if i > 0 && i < trees.len() - 1 {
                if j > 0 && j < trees[i].len() - 1 {
                    if visible_tree(&trees, i, j) {
                        total += 1;
                    }
                } else {
                    total += 1;
                }
            } else {
                total += 1;
            }
        }
    }

    total
}

fn get_highest_scenic(trees: &Trees) -> usize {
    let mut max = 0;

    for (i, _) in trees.iter().enumerate() {
        for (j, _) in trees[i].iter().enumerate() {
            if i > 0 && i < trees.len() - 1 && j > 0 && j < trees[i].len() - 1 {
                let score = get_scenic(trees, i, j);

                if score > max {
                    max = score
                }
            }
        }
    }

    max
}

fn get_scenic(trees: &Trees, i: usize, j: usize) -> usize {
    let visible_r_d = scenic_score_direction(trees, i, j, true, true);
    let visible_r_u = scenic_score_direction(trees, i, j, true, false);
    let visible_l_d = scenic_score_direction(trees, i, j, false, true);
    let visible_l_u = scenic_score_direction(trees, i, j, false, false);

    visible_r_d * visible_r_u * visible_l_d * visible_l_u
}

fn visible_tree(trees: &Trees, i: usize, j: usize) -> bool {
    let visible_r_d = visible_direction(trees, i, j, true, true);
    let visible_r_u = visible_direction(trees, i, j, true, false);
    let visible_l_d = visible_direction(trees, i, j, false, true);
    let visible_l_u = visible_direction(trees, i, j, false, false);

    visible_r_d || visible_r_u || visible_l_d || visible_l_u
}

// i stands for the column and j for the row.
//
// \ j 1 2 3
// i
// 1
// 2
// 3
fn scenic_score_direction(tree: &Trees, i: usize, j: usize, hor: bool, right: bool) -> usize {
    let tree_size = tree[i][j];

    // horizontal
    if hor {
        let mut _j = j;
        if right {
            // don't probe the same tree
            _j = j + 1;

            while _j < tree[i].len() {
                let curr_size = tree[i][_j];
                if tree_size <= curr_size {
                    return _j - j;
                }
                _j += 1;
            }

            // has reached the boundary
            return _j - j - 1;
        } else {
            // don't probe the same tree
            _j = match j.checked_sub(1) {
                Some(_j) => _j,
                None => return 1, // is on the edge
            };

            while _j > 0 {
                let curr_size = tree[i][_j];
                if tree_size <= curr_size {
                    return j - _j;
                }

                _j -= 1;
            }

            return j;
        }
    } else {
        // vertical
        let mut _i = i;
        if right {
            // don't probe the same tree
            _i = i + 1;

            while _i < tree.len() {
                let curr_size = tree[_i][j];
                if tree_size <= curr_size {
                    return _i - i;
                }
                _i += 1;
            }

            return _i - i - 1;
        } else {
            // don't probe the same tree
            _i = match i.checked_sub(1) {
                Some(_i) => _i,
                None => return 1, // on the edge
            };

            while _i > 0 {
                let curr_size = tree[_i][_i];
                if tree_size <= curr_size {
                    return i - _i;
                }

                _i -= 1;
            }

            return i;
        }
    }
}

fn visible_direction(tree: &Trees, i: usize, j: usize, hor: bool, right: bool) -> bool {
    let tree_size = tree[i][j];

    // horizontal
    if hor {
        let mut _j = j;
        if right {
            // don't probe the same tree
            _j = j + 1;

            while _j < tree[i].len() {
                let curr_size = tree[i][_j];
                if tree_size <= curr_size {
                    return false;
                }
                _j += 1;
            }
        } else {
            // don't probe the same tree
            _j = match j.checked_sub(1) {
                Some(_j) => _j,
                None => return true, // on the edge
            };

            loop {
                let curr_size = tree[i][_j];
                if tree_size <= curr_size {
                    return false;
                }

                _j = match _j.checked_sub(1) {
                    Some(_j) => _j,
                    None => return true, // on the edge
                };
            }
        }
    } else {
        // vertical
        let mut _i = i;
        if right {
            // don't probe the same tree
            _i = i + 1;

            while _i < tree.len() {
                let curr_size = tree[_i][j];
                if tree_size <= curr_size {
                    return false;
                }
                _i += 1;
            }
        } else {
            // don't probe the same tree
            _i = match i.checked_sub(1) {
                Some(_i) => _i,
                None => return true, // is on the edge
            };

            loop {
                let curr_size = tree[_i][j];
                if tree_size <= curr_size {
                    return false;
                }

                _i = match _i.checked_sub(1) {
                    Some(_i) => _i,
                    None => return true, // on the edge
                };
            }
        }
    }

    true
}

#[test]
fn test_visible() {
    let trees: Trees = vec![vec![1, 5, 9]];

    assert!(visible_tree(&trees, 0, 0));
    assert!(visible_tree(&trees, 0, 1));
    assert!(visible_tree(&trees, 0, 2));

    let trees: Trees = vec![vec![9, 9, 9], vec![9, 5, 9], vec![9, 9, 9]];

    assert!(visible_tree(&trees, 1, 0));
    assert!(!visible_tree(&trees, 1, 1));
    assert!(visible_tree(&trees, 1, 2));

    let trees: Trees = vec![vec![9, 1, 9], vec![9, 8, 9], vec![9, 9, 9]];

    assert!(visible_tree(&trees, 0, 0));
    assert!(visible_tree(&trees, 0, 1));
    assert!(visible_tree(&trees, 0, 2));

    assert!(visible_tree(&trees, 1, 0));

    assert!(!visible_direction(&trees, 1, 1, true, true));
    assert!(!visible_direction(&trees, 1, 1, true, false));
    assert!(!visible_direction(&trees, 1, 1, false, true));
    assert!(visible_direction(&trees, 1, 1, false, false));
    assert!(visible_tree(&trees, 1, 1));

    assert!(visible_tree(&trees, 1, 2));

    assert!(visible_tree(&trees, 2, 0));
    assert!(visible_tree(&trees, 2, 1));
    assert!(visible_tree(&trees, 2, 2));
}

#[test]
fn test_one() {
    let trees: Trees = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    let visible = get_visible_trees(trees);

    assert_eq!(visible, 21);
}

#[test]
fn test_scenic() {
    #[rustfmt::skip]
    let trees: Trees = vec![
        vec![3, 3, 3], 
        vec![1, 2, 1], 
        vec![3, 3, 3]
    ];

    assert_eq!(scenic_score_direction(&trees, 1, 1, true, true), 1);
    assert_eq!(scenic_score_direction(&trees, 1, 1, true, false), 1);
    assert_eq!(scenic_score_direction(&trees, 1, 1, false, true), 1);
    assert_eq!(scenic_score_direction(&trees, 1, 1, false, false), 1);

    #[rustfmt::skip]
    let trees: Trees = vec![
        vec![3, 3, 3], 
        vec![3, 2, 3], 
        vec![3, 1, 3], 
        vec![3, 3, 3]
    ];

    assert_eq!(scenic_score_direction(&trees, 1, 1, false, true), 2);

    let trees: Trees = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    assert_eq!(scenic_score_direction(&trees, 1, 2, true, true), 2);
    assert_eq!(scenic_score_direction(&trees, 1, 2, true, false), 1);
    assert_eq!(scenic_score_direction(&trees, 1, 2, false, true), 2);
    assert_eq!(scenic_score_direction(&trees, 1, 2, false, false), 1);

    assert_eq!(get_scenic(&trees, 1, 2), 4);

    assert_eq!(scenic_score_direction(&trees, 3, 2, true, true), 2);
    assert_eq!(scenic_score_direction(&trees, 3, 2, true, false), 2);
    assert_eq!(scenic_score_direction(&trees, 3, 2, false, true), 1);
    assert_eq!(scenic_score_direction(&trees, 3, 2, false, false), 2);

    assert_eq!(get_scenic(&trees, 3, 2), 8);
}

#[test]
fn test_wtf_two() {
    #[rustfmt::skip]
    let trees: Trees = vec![
        vec![1, 1, 1, 1],
        vec![1, 2, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
    ];

    assert_eq!(get_scenic(&trees, 1, 1), 4);

    let score = get_highest_scenic(&trees);

    assert_eq!(score, 4);
}

#[test]
fn test_two() {
    let trees: Trees = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    let score = get_highest_scenic(&trees);

    assert_eq!(score, 8);
}
