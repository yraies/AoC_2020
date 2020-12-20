use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day17)]
pub fn generate_plane(input: &str) -> Vec<Vec<Vec<bool>>> {
    vec!(input.lines().map(|line| {
        let mut cols = vec!();
        for c in line.chars() {
            match c {
                '#' => cols.push(true),
                _ => cols.push(false)
            }
        }
        cols
    }).collect())
}

pub fn print_field(input: &Vec<Vec<Vec<bool>>>) {
    let d = (input.len() as i32 - 1) / 2;
    for depth in 0..input.len() {
        println!("Z: {}", depth as i32 - d);
        for row in 0..input[depth].len() {
            for col in 0..input[depth][row].len() {
                if input[depth][row][col] { print!("#") } else { print!(".") }
            }
            println!();
        }
        println!();
    }
    println!();
}

#[aoc(day17, part1)]
pub fn get_3d_cube_count(input: &Vec<Vec<Vec<bool>>>) -> String {
    print_field(input);
    let mut curr_grid = input.clone();

    let offsets: Vec<(i32, i32, i32)> = (-1..2).cartesian_product(-1..2)
        .cartesian_product(-1..2)
        .map(|(ab, c)| (ab.0, ab.1, c))
        .filter(|(a, b, c)| !(*a == 0 && *b == 0 && *c == 0))
        .collect();

    for i in 1..=6 {
        let mut new_grid =
            vec!(vec!(vec!(false; curr_grid[0][0].len() + 2); curr_grid[0].len() + 2); curr_grid.len() + 2);

        let maxdepth = new_grid.len();
        for depth in 0..maxdepth {
            let maxrow = new_grid[depth].len();
            for row in 0..maxrow {
                let maxcol = new_grid[depth][row].len();
                for col in 0..maxcol {
                    let mut count_active = 0;
                    for (dd, dr, dc) in &offsets {
                        let d = depth as i32 + dd;
                        let r = row as i32 + dr;
                        let c = col as i32 + dc;
                        if d >= 1 && d < maxdepth as i32 - 1 &&
                            r >= 1 && r < maxrow as i32 - 1 &&
                            c >= 1 && c < maxcol as i32 - 1 {
                            //println!("on old field");
                            if curr_grid[(d - 1) as usize][(r - 1) as usize][(c - 1) as usize] {
                                count_active += 1;
                            }
                        }
                    }
                    if depth >= 1 && depth < maxdepth - 1 &&
                        row >= 1 && row < maxrow - 1 &&
                        col >= 1 && col < maxcol - 1 {
                        //println!("on old field");
                        if curr_grid[depth - 1][row - 1][col - 1] && (count_active == 3 || count_active == 2) {
                            new_grid[depth][row][col] = true;
                        }
                        if !curr_grid[depth - 1][row - 1][col - 1] && count_active == 3 {
                            new_grid[depth][row][col] = true;
                        }
                    } else if count_active == 3 {
                        new_grid[depth][row][col] = true;
                    }
                }
            }
        }

        println!("After {} cycle(s):\n", i);
        //print_field(&new_grid);
        curr_grid = new_grid;
    }


    format!("{:?}", curr_grid.iter().map(|layer| {
        let layersum = layer.iter().map(|row| {
            let rowsum = row.iter().filter(|val| **val).count();
            rowsum
        }).fold(0, |s, v| s + v);
        layersum
    }).fold(0, |s, v| s + v)
    )
}

#[aoc(day17, part2)]
pub fn get_4d_cube_count(input: &Vec<Vec<Vec<bool>>>) -> String {
    let mut curr_grid = vec!(input.clone());

    let offsets: Vec<(i32, i32, i32, i32)> = (-1..2).cartesian_product(-1..2)
        .cartesian_product(-1..2).cartesian_product(-1..2)
        .map(|(((a, b), c), d)| (a, b, c, d))
        .filter(|(a, b, c, d)| !(*a == 0 && *b == 0 && *c == 0 && *d == 0))
        .collect();

    for i in 1..=6 {
        let mut new_grid =
            vec!(vec!(vec!(vec!(false; curr_grid[0][0][0].len() + 2); curr_grid[0][0].len() + 2); curr_grid[0].len() + 2); curr_grid.len() + 2);

        let maxwub = new_grid.len();
        for wub in 0..maxwub {
            let maxdepth = new_grid[wub].len();
            for depth in 0..maxdepth {
                let maxrow = new_grid[wub][depth].len();
                for row in 0..maxrow {
                    let maxcol = new_grid[wub][depth][row].len();
                    for col in 0..maxcol {
                        let mut count_active = 0;
                        for (dw, dd, dr, dc) in &offsets {
                            let w = wub as i32 + dw;
                            let d = depth as i32 + dd;
                            let r = row as i32 + dr;
                            let c = col as i32 + dc;
                            if w >= 1 && w < maxwub as i32 - 1 &&
                                d >= 1 && d < maxdepth as i32 - 1 &&
                                r >= 1 && r < maxrow as i32 - 1 &&
                                c >= 1 && c < maxcol as i32 - 1 {
                                //println!("on old field");
                                if curr_grid[(w - 1) as usize][(d - 1) as usize][(r - 1) as usize][(c - 1) as usize] {
                                    count_active += 1;
                                }
                            }
                        }
                        if wub >= 1 && wub < maxwub - 1 &&
                            depth >= 1 && depth < maxdepth - 1 &&
                            row >= 1 && row < maxrow - 1 &&
                            col >= 1 && col < maxcol - 1 {
                            //println!("on old field");
                            if curr_grid[wub - 1][depth - 1][row - 1][col - 1] && (count_active == 3 || count_active == 2) {
                                new_grid[wub][depth][row][col] = true;
                            }
                            if !curr_grid[wub - 1][depth - 1][row - 1][col - 1] && count_active == 3 {
                                new_grid[wub][depth][row][col] = true;
                            }
                        } else if count_active == 3 {
                            new_grid[wub][depth][row][col] = true;
                        }
                    }
                }
            }
        }
        println!("After {} cycle(s):\n", i);
        curr_grid = new_grid;
    }

    format!("{:?}", curr_grid.iter().map(|wub| {
        let wubsum = wub.iter().map(|layer| {
            let layersum = layer.iter().map(|row| {
                let rowsum = row.iter().filter(|val| **val).count();
                rowsum
            }).fold(0, |s, v| s + v);
            layersum
        }).fold(0, |s, v| s + v);
        wubsum
    }).fold(0, |s, v| s + v)
    )
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works_0() {
        assert_eq!(&super::get_3d_cube_count(&super::generate_plane(".#.\n..#\n###")), "0");
    }
}

