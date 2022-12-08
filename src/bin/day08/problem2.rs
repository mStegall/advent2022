pub fn main() {
    let mut field = include_str!("./input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|h| (h as isize, 0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for i in 0..field.len() {
        for j in 0..field.len() {
            let vis_height = field[i][j].0;
            let mut overall_vis = 1;

            let mut dir_vis = 0;
            for i2 in (0..i).rev() {
                dir_vis += 1;
                if field[i2][j].0 >= vis_height {
                    break;
                }
            }
            overall_vis *= dir_vis;

            dir_vis = 0;
            for i2 in i + 1..field.len() {
                dir_vis += 1;
                if field[i2][j].0 >= vis_height {
                    break;
                }
            }
            overall_vis *= dir_vis;

            let mut dir_vis = 0;
            for j2 in (0..j).rev() {
                dir_vis += 1;
                if field[i][j2].0 >= vis_height {
                    break;
                }
            }
            overall_vis *= dir_vis;

            dir_vis = 0;
            for j2 in j + 1..field.len() {
                dir_vis += 1;
                if field[i][j2].0 >= vis_height {
                    break;
                }
            }
            overall_vis *= dir_vis;

            let mut tree = &mut field[i][j];

            tree.1 = overall_vis;
        }
    }

    println!(
        "{:?}",
        field.iter().flat_map(|l| l.iter().map(|(_, v)| v)).max()
    )
}
