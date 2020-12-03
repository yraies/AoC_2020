use aoc_runner_derive::{aoc, aoc_generator};

pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<bool>>,
}

impl Map {
    fn get_on_torus(&self, y: usize, x: usize) -> bool {
        self.tiles[x % self.height][y % self.width]
    }
    pub fn count_trees(&self, trajectory: (usize, usize)) -> usize {
        let (mut x, mut y) = (0usize, 0usize);
        let (tx, ty) = trajectory;
        let mut treecount: usize = 0;
        while y < self.height {
            //println!("x:{} y:{} tree:{}", x,y, self.get_on_torus(x, y));
            if self.get_on_torus(x, y) { treecount += 1; }
            x += tx;
            y += ty;
        }
        treecount
    }
}

#[aoc_generator(day3)]
pub fn generate_map(input: &str) -> Map {
    let tiles :Vec<Vec<bool>> = input.split("\n").map(|v| {
        v.chars().map(|c| match c {
            '.' => false,
            '#' => true,
            _ => unreachable!()
        }).collect()
    }).collect();

    Map {
        height: tiles.len(),
        width: tiles[0].len(),
        tiles,
    }
}

#[aoc(day3, part1)]
pub fn count_trees_3right_1down(map: &Map) -> usize {
    map.count_trees((3, 1))
}

#[aoc(day3, part2)]
pub fn multiply_counted_trees(map: &Map) -> usize {
    map.count_trees((1, 1)) *
    map.count_trees((3, 1)) *
    map.count_trees((5, 1)) *
    map.count_trees((7, 1)) *
    map.count_trees((1, 2))
}