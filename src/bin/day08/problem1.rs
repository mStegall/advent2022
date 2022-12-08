pub fn main() {
    let mut field = include_str!("./input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10) )
                .map(|h| (h as isize, false))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for  i in 0..field.len() {
        let mut vis_height = -1;
        
        for j in 0..field.len() {
            let mut tree = &mut field[i][j];
            if tree.0 > vis_height {
                tree.1 = true;
                vis_height = tree.0;
            }
        }

        vis_height = -1;
        
        for j in (0..field.len()).rev() {
            let mut tree = &mut field[i][j];
            if tree.0 > vis_height {
                tree.1 = true;
                vis_height = tree.0;
            }
        }
    }

    for  i in 0..field.len() {
        let mut vis_height = -1;
        
        for j in 0..field.len() {
            let mut tree = &mut field[j][i];
            if tree.0 > vis_height {
                tree.1 = true;
                vis_height = tree.0;
            }
        }

        vis_height = -1;
        
        for j in (0..field.len()).rev() {
            let mut tree = &mut field[j][i];
            if tree.0 > vis_height {
                tree.1 = true;
                vis_height = tree.0;
            }
        }
    }

    println!("{}", field.iter().flat_map(|l| l.iter().filter(|(_,v)| *v)).count())
}
