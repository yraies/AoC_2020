use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::{Display, Formatter};
use itertools::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Tile {
    Free,
    Occupied,
    Floor,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Floor => write!(f, "."),
            Tile::Free => write!(f, "L"),
            Tile::Occupied => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Layout {
    tiles: Vec<Vec<Tile>>,
    occupied: usize,
    width: i64,
    height: i64,
}

impl From<&Vec<Vec<Tile>>> for Layout {
    fn from(foo: &Vec<Vec<Tile>>) -> Self {
        Layout {
            tiles: foo.clone(),
            occupied: foo.iter().flatten().filter(|&&t| t == Tile::Occupied).count(),
            height: foo.len() as i64,
            width: foo[0].len() as i64,
        }
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[i].len() {
                write!(f, "{}", self.tiles[i][j])?;
            }
            if i != self.tiles.len() - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl Layout {
    fn immediate_neighbours(&self, col: i64, row: i64) -> usize {
        let mut acc = 0;
        ((col - 1)..(col - 2)).cartesian_product((row - 1)..(row - 2))
            .filter(|&(x, y)| {
                x < 0 || y < 0 || x >= self.width || y >= self.height || (x == col && row == y)
            })
            .for_each(|(x, y)| {
                acc += if self.tiles[y as usize][x as usize] == Tile::Occupied { 1 } else { 0 };
            });
        acc
    }

    fn sight_neighbours(&self, col: i64, row: i64) -> usize {
        let mut acc = 0;
        (-1..2).cartesian_product(-1..2)
            .filter(|&(x, y)| (x != 0 || y != 0))
            .for_each(|(dx, dy)| {
                let mut i = 0;
                loop {
                    i += 1;
                    let (x, y) = (col + i * dx, row + i * dy);
                    if x < 0 || y < 0 || x >= self.width || y >= self.height {
                        break;
                    } else if self.tiles[y as usize][x as usize] == Tile::Free {
                        break;
                    } else if self.tiles[y as usize][x as usize] == Tile::Occupied {
                        acc += 1;
                        break;
                    }
                }
            });
        acc
    }

    pub fn next(&mut self, threshold: usize, occ_neighbours: fn(&Layout, i64, i64) -> usize) -> bool {
        let mut changed = false;
        let mut tiles = self.tiles.clone();
        for row in 0..self.tiles.len() {
            for col in 0..self.tiles[row].len() {
                //let mut foo = 0;
                if self.tiles[row][col] != Tile::Floor {
                    let occupied_neighbours = occ_neighbours(&self, col as i64, row as i64);
                    //foo = occupied_neighbours;
                    if tiles[row][col] == Tile::Free && occupied_neighbours == 0 {
                        tiles[row][col] = Tile::Occupied;
                        changed = true;
                    } else if tiles[row][col] == Tile::Occupied && occupied_neighbours >= threshold {
                        tiles[row][col] = Tile::Free;
                        changed = true;
                    }
                }
                        //print!("{}", foo);
            }
                //print!("\n");
        }
        //print!("\n");
        self.tiles = tiles;
        self.occupied = self.tiles.iter().flatten().filter(|&&t| t == Tile::Occupied).count();
        changed
    }
}

#[aoc_generator(day11)]
pub fn generate_seat_layout(input: &str) -> Vec<Vec<Tile>> {
    input.lines().map(|line| line.chars().map(|c| match c {
        '.' => Tile::Floor,
        'L' => Tile::Free,
        '#' => Tile::Occupied,
        _ => unreachable!()
    }).collect()).collect()
}

#[aoc(day11, part1)]
pub fn get_stable_occupied_seats_immediate(input: &Vec<Vec<Tile>>) -> String {
    let mut layout = Layout::from(input);
    let mut i = 0;
    //println!("i: {}\n{}\n", i, layout);
    loop {
        let changed = layout.next(4, Layout::immediate_neighbours);
        //println!("{}", changed);
        i += 1;
        //println!("i: {}\n{}\n", i, layout);
        if !changed || i > 100000 {
            break;
        }
    }
    format!("{} {}", i, layout.occupied)
}

#[aoc(day11, part2)]
pub fn get_stable_occupied_seats_sight(input: &Vec<Vec<Tile>>) -> String {
    let mut layout = Layout::from(input);
    let mut i = 0;
    loop {
        let changed = layout.next(5, Layout::sight_neighbours);
        i += 1;
        if !changed || i > 100000 {
            break;
        }
    }
    format!("{} {}", i, layout.occupied)
}