pub fn main() {
    let mut pattern = include_str!("./input.txt").chars().cycle();

    let pieces_vec: Vec<Vec<(usize, usize)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    let mut pieces = pieces_vec.iter().cycle();

    let mut board: Vec<[bool; 7]> = vec![[true; 7]];

    let mut max_height = 0;

    for _ in 0..2022 {
        while board.len() < max_height + 9 {
            board.push([false; 7])
        }

        let mut piece_x = 2;
        let mut piece_y = max_height + 4;

        let piece = pieces.next().unwrap();

        loop {
            let direction = pattern.next().unwrap();
            // println!("piece {i}, direction {direction} ({piece_x},{piece_y})");
            if direction == '<' {
                if piece_x != 0 && !has_collision(piece, &board, piece_x - 1, piece_y) {
                    piece_x -= 1
                }
            } else {
                if !has_collision(piece, &board, piece_x + 1, piece_y) {
                    piece_x += 1
                }
            }

            if has_collision(piece, &board, piece_x, piece_y - 1) {
                break;
            }

            piece_y -= 1;
        }

        for (dx, dy) in piece {
            let y = piece_y + dy;
            board[y][piece_x + dx] = true;
            if y > max_height {
                max_height = y;
            }
        }

        // println!(
        //     "{}",
        //     board
        //         .iter()
        //         .rev()
        //         .map(|l| {
        //             let mut s = l
        //                 .map(|c| if c { '#' } else { '.' })
        //                 .iter()
        //                 .collect::<String>();
        //             s.push('\n');
        //             s
        //         })
        //         .collect::<String>()
        // );
    }

    // println!(
    //     "{}",
    //     board
    //         .iter()
    //         .rev()
    //         .map(|l| {
    //             let mut s = l
    //                 .map(|c| if c { '#' } else { '.' })
    //                 .iter()
    //                 .collect::<String>();
    //             s.push('\n');
    //             s
    //         })
    //         .collect::<String>()
    // );
    println!("{max_height}")
}

fn has_collision(
    piece: &Vec<(usize, usize)>,
    board: &Vec<[bool; 7]>,
    piece_x: usize,
    piece_y: usize,
) -> bool {
    for (dx, dy) in piece {
        let x = piece_x + dx;
        if x >= 7 {
            return true;
        }
        let y = piece_y + dy;
        if board[y][x] {
            return true;
        }
    }
    false
}
