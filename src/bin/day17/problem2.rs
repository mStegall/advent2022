pub fn main() {
    let pattern_chars = include_str!("./input.txt");
    let mut pattern = pattern_chars.chars().cycle();

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
    let mut offsets = vec![];

    for _ in 0..15000 as i64 {
        while board.len() < max_height + 9 {
            board.push([false; 7])
        }

        let mut piece_x = 2;
        let mut piece_y = max_height + 4;

        let piece = pieces.next().unwrap();

        loop {
            let direction = pattern.next().unwrap();
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
        offsets.push((
            (piece_x, piece_y as isize - max_height as isize),
            max_height,
        ));

        for (dx, dy) in piece {
            let y = piece_y + dy;
            board[y][piece_x + dx] = true;
            if y > max_height {
                max_height = y;
            }
        }
    }

    let mut i = 0;

    let o = offsets.iter().map(|s| s.0).collect::<Vec<_>>();
    let cycle_start;
    let cycle_size;
    loop {
        let candidate = o[i];
        if let Some((j, _)) = o[i + 2..]
            .iter()
            .enumerate()
            .find(|(_, s)| **s == candidate)
        {
            let start = i;
            let size = j + 2;

            if o[i..i + size] == o[i + size..2 * size + i]
                && o[i..i + size] == o[i + 2 * size..3 * size + i]
            {
                cycle_size = size;
                cycle_start = start;
                break;
            }
        }
        i += 1;
    }

    let cycle_height = offsets[cycle_start + cycle_size - 1].1 - offsets[cycle_start - 1].1;
    let a = offsets[cycle_start - 1].1;
    let r = 1000000000000 - cycle_start - 1;
    let b = (r / cycle_size) * cycle_height;
    let c = offsets[r % cycle_size + cycle_start + 1].1 - offsets[cycle_start - 1].1;

    println!("{}", a + b + c);
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
