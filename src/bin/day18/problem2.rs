#[derive(Clone)]
enum State {
    Air,
    Steam,
    Rock,
}

pub fn main() {
    let data = include_str!("./input.txt")
        .lines()
        .map(|l| {
            l.split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let size = data.iter().filter_map(|s| s.iter().max()).max().unwrap();

    let mut grid = vec![vec![vec![State::Air; size + 1]; size + 1]; size + 1];

    for p in data.iter() {
        grid[p[0]][p[1]][p[2]] = State::Rock;
    }

    let checks: Vec<Vec<isize>> = vec![
        vec![1, 0, 0],
        vec![-1, 0, 0],
        vec![0, 1, 0],
        vec![0, -1, 0],
        vec![0, 0, 1],
        vec![0, 0, -1],
    ];

    let mut steam: Vec<Vec<usize>> = vec![vec![0, 0, 0]];

    while let Some(p) = steam.pop() {
        for c in checks.iter() {
            let x = (p[0] as isize + c[0]) as usize;
            let y = (p[1] as isize + c[1]) as usize;
            let z = (p[2] as isize + c[2]) as usize;
            if let Some(State::Air) = grid.get(x).and_then(|s| s.get(y)).and_then(|t| t.get(z)) {
                grid[x][y][z] = State::Steam;
                steam.push(vec![x, y, z]);
            }
        }
    }

    let mut surface_area = 0;
    for p in data.iter() {
        for c in checks.iter() {
            if let Some(State::Steam) = grid
                .get((p[0] as isize + c[0]) as usize)
                .and_then(|s| s.get((p[1] as isize + c[1]) as usize))
                .and_then(|t| t.get((p[2] as isize + c[2]) as usize))
                .or(Some(&State::Steam))
            {
                surface_area += 1;
            }
        }
    }

    println!("{surface_area:?}");
}
