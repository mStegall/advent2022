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

    let mut grid = vec![vec![vec![false; size + 1]; size + 1]; size + 1];

    for p in data.iter() {
        grid[p[0]][p[1]][p[2]] = true;
    }

    let checks: Vec<Vec<isize>> = vec![
        vec![1, 0, 0],
        vec![-1, 0, 0],
        vec![0, 1, 0],
        vec![0, -1, 0],
        vec![0, 0, 1],
        vec![0, 0, -1],
    ];

    let mut surface_area = 0;
    for p in data.iter() {
        for c in checks.iter() {
            if let Some(false) = grid
                .get((p[0] as isize + c[0]) as usize)
                .and_then(|s| s.get((p[1] as isize + c[1]) as usize))
                .and_then(|t| t.get((p[2] as isize + c[2]) as usize))
                .or(Some(&false))
            {
                surface_area += 1;
            }
        }
    }

    println!("{surface_area:?}");
}
