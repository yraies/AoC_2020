use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryInto;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Tile {
    id: usize,
    data: [[bool; 10]; 10],
}

type Strip = [bool; 10];

impl Tile {
    #[allow(dead_code)]
    fn print_strip(strip: &Strip) -> String {
        use std::io::Write;
        let mut f = Vec::new();
        for i in 0..10 {
            if strip[i] {
                write!(&mut f, "#").unwrap();
            } else {
                write!(&mut f, ".").unwrap();
            }
        }
        String::from_utf8(f).unwrap()
    }
    fn sort(strip: &mut Strip) -> bool {
        let mut norm = 0;
        let mut rev = 0;
        for i in 0..10 {
            if strip[i] { norm += 2u16.pow(i as u32); }
            if strip[9 - i] { rev += 2u16.pow(i as u32); }
        }
        if rev > norm {
            strip.reverse();
        }
        rev > norm
    }
    fn get_sorted_borders(&self) -> Vec<(bool, Strip)> {
        self.get_unsorted_borders().into_iter().map(|mut x| (Self::sort(&mut x), x)).collect()
    }
    fn get_unsorted_borders(&self) -> Vec<Strip> {
        let mut borders = Vec::with_capacity(4);
        for (ux, uy, ix, iy) in vec!((0, 0, 1, 0), (9, 0, 0, 1), (0, 9, 1, 0), (0, 0, 0, 1)) {
            let mut val = [false; 10];
            for i in 0..10 {
                val[i] = self.data[ux + ix * i][uy + iy * i];
            }
            ;
            borders.push(val);
        }
        borders
    }
    fn flip(&self, horizontal: bool, vertical: bool) -> Tile {
        let mut data = [[false; 10]; 10];
        let rows = if vertical { (0..9).rev() } else { 0..9 };
        let cols = if horizontal { (0..9).rev() } else { 0..9 };
        for (srow,drow) in rows.into_iter().zip(0..9){
            for (scol,dcol) in cols.zip(0..9) {
                data[drow][dcol] = self.data[srow][drow];
            }
        }
        Tile { id: self.id, data }
    }
}

#[aoc_generator(day20)]
pub fn generate_tile_array(input: &str) -> Vec<Tile> {
    input.split("\n\n").map(|block| {
        let mut lines = block.lines();
        let id_str = lines.next().unwrap();
        let id = id_str[5..id_str.len() - 1].parse::<usize>().unwrap();
        let data = lines.map(|line| {
            let mut data_line = [false; 10];
            for (idx, field) in line.chars().enumerate() {
                data_line[idx] = field == '#';
            }
            data_line
        }).collect::<Vec<_>>().try_into().expect("Tiles have to be of size 10x10");
        Tile { id, data }
    }).collect()
}

#[aoc(day20, part1)]
pub fn get_multiplied_corner_ids(input: &Vec<Tile>) -> String {
    let mut border_map = HashMap::new();
    let border_vec: Vec<_> = input.iter().map(|tile| (tile, tile.get_sorted_borders())).collect();

    for (tile, borders) in border_vec {
        for (_, border) in borders {
            let neighbours = border_map.entry(border).or_insert(vec!());
            neighbours.push(tile.id);
        }
    }

    let mut neighbour_map: HashMap<usize, u32> = HashMap::new();
    for (_, neighbours) in border_map {
        for neighbour in &neighbours {
            let e = neighbour_map.entry(*neighbour).or_insert(0u32);
            *e += neighbours.len() as u32;
        }
    }

    let corners = neighbour_map.iter()
        .sorted_by_key(|(_id, neighbours)| **neighbours)
        .take(4).collect::<Vec<_>>();

    format!("{} <- {:?}", corners.iter().fold(1u64, |x, (&y, _)| x * y as u64), corners)
}

#[aoc(day20, part2)]
pub fn find_monster(input: &Vec<Tile>) -> String {
    let mut border_map = HashMap::new();
    let border_vec: Vec<_> = input.iter().map(|tile| (tile, tile.get_sorted_borders())).collect();

    for (tile, borders) in border_vec {
        for (_, border) in borders {
            let neighbours = border_map.entry(border).or_insert(vec!());
            neighbours.push(tile.id);
        }
    }

    let mut neighbour_map: HashMap<usize, u32> = HashMap::new();
    for (_, neighbours) in border_map {
        for neighbour in &neighbours {
            let e = neighbour_map.entry(*neighbour).or_insert(0u32);
            *e += neighbours.len() as u32;
        }
    }

    let mut global_tile_map = HashMap::new();
    global_tile_map.insert((0, 0), input[0].clone());


    format!("{}", 0)
}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works_0() {
        assert_eq!(&super::get_3d_cube_count(&super::generate_plane(".#.\n..#\n###")), "0");
    }
}
*/













