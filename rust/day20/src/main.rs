//! Solutions to 2020: Advent of Code day 20
//! By Peter Fornwall

use std::fs;

struct Image {
    id: usize,
    pixels: Vec<Vec<bool>>,
}

struct BigImage {
    image_indexes: Vec<Vec<usize>>,
    transform_nrs: Vec<Vec<usize>>,
}

#[derive(Copy, Clone)]
struct Edge<'a> {
    image: &'a Image,
    // 0-top, 1-right, 2-bottom, 3-left
    which_one: usize,
    transform_nr: usize,
}

struct EdgeIterator<'a> {
    edge: &'a Edge<'a>,
    current_start: usize,
    current_end: usize,
}

impl<'a> PartialEq for Edge<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_start >= self.current_end {
            return None;
        }
        let old = self.current_start;
        self.current_start += 1;
        Some(self.edge.get_pixel(old))
    }
}
impl<'a> DoubleEndedIterator for EdgeIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_start >= self.current_end {
            return None;
        }
        self.current_end -= 1;
        Some(self.edge.get_pixel(self.current_end))
    }
}

impl<'a> Edge<'a> {
    fn get_pixel(&self, val: usize) -> bool {
        let pixels = &self.image.pixels;
        if self.which_one == 0 {
            let (new_x, new_y) = transform_point(pixels.len(), self.transform_nr, val, 0);
            return pixels[new_y][new_x];
        } else if self.which_one == 1 {
            let (new_x, new_y) =
                transform_point(pixels.len(), self.transform_nr, pixels.len() - 1, val);
            return pixels[new_y][new_x];
        } else if self.which_one == 2 {
            let (new_x, new_y) =
                transform_point(pixels.len(), self.transform_nr, val, pixels.len() - 1);
            return pixels[new_y][new_x];
        } else if self.which_one == 3 {
            let (new_x, new_y) = transform_point(pixels.len(), self.transform_nr, 0, val);
            return pixels[new_y][new_x];
        }
        panic!();
    }

    fn iter(&'a self) -> EdgeIterator {
        EdgeIterator {
            edge: self,
            current_start: 0,
            current_end: self.image.pixels.len(),
        }
    }
    fn has_common_edge(&self, other_image: &Image) -> bool {
        for edge_type in 0..4 {
            if self.iter().eq(other_image.get_edge(edge_type, 0).iter()) {
                return true;
            }
            if self
                .iter()
                .eq(other_image.get_edge(edge_type, 0).iter().rev())
            {
                return true;
            }
        }
        false
    }

    fn matches_edge_in_another_tile(&self, tiles: &[Image]) -> bool {
        let count = tiles
            .iter()
            .map(|tile| self.has_common_edge(tile))
            .filter(|x| *x)
            .count();
        assert!(count < 3);
        count == 2
    }
}

impl BigImage {
    fn get_pixel(&self, images: &[Image], x: usize, y: usize, flop: usize) -> bool {
        let tile_width = images[0].pixels.len() - 2;
        let big_width = self.image_indexes.len() * tile_width;
        let (new_x, new_y) = transform_point(big_width, flop, x, y);

        let tile_x = new_x / tile_width;
        let tile_x_index = new_x % tile_width + 1;
        let tile_y = new_y / tile_width;
        let tile_y_index = new_y % tile_width + 1;
        let tile_flop = self.transform_nrs[tile_y][tile_x];

        let (flopped_x, flopped_y) = transform_point(
            images[0].pixels.len(),
            tile_flop,
            tile_x_index,
            tile_y_index,
        );

        let image_nr = self.image_indexes[tile_y][tile_x];
        let image = &images[image_nr];

        image.pixels[flopped_y][flopped_x]
    }
}

impl Image {
    fn get_edge(&self, nr: usize, flop: usize) -> Edge {
        Edge {
            image: self,
            which_one: nr,
            transform_nr: flop,
        }
    }

    fn has_common_edge(&self, other_image: &Image) -> bool {
        for edge_type in 0..4 {
            if self.get_edge(edge_type, 0).has_common_edge(other_image) {
                return true;
            }
        }
        false
    }
}

fn transform_point(width: usize, flop: usize, x: usize, y: usize) -> (usize, usize) {
    assert!(x < width);
    assert!(y < width);
    let (mut new_x, new_y) = match flop & 3 {
        0 => (x, y),
        1 => (width - 1 - y, x),
        2 => (width - 1 - x, width - 1 - y),
        3 => (y, width - 1 - x),
        _ => panic!(),
    };
    if (flop & 4) > 0 {
        new_x = width - 1 - new_x;
    }
    (new_x, new_y)
}

fn get_corner_tiles(tiles: &[Image]) -> Vec<usize> {
    let mut corner_tiles = Vec::new();
    for i in 0..tiles.len() {
        let mut found_same = 0;
        for j in 0..tiles.len() {
            if i != j && tiles[i].has_common_edge(&tiles[j]) {
                found_same += 1;
            }
        }
        if found_same == 2 {
            corner_tiles.push(i);
        }
    }
    corner_tiles
}

fn part1(tiles: &[Image]) -> i64 {
    let corner_tiles = get_corner_tiles(tiles);
    let result = corner_tiles.iter().map(|&i| tiles[i].id as i64).product();
    println!("Part1: {}", result);
    result
}

fn part2(tiles: &[Image]) -> i64 {
    let width_in_nr_tiles = (tiles.len() as f64).sqrt() as usize;
    let tile_width = tiles[0].pixels.len();
    let corner_index = get_corner_tiles(tiles)[0];

    let mut used_tiles = vec![false; tiles.len()];
    let mut last_right: Option<Edge> = None;
    let mut last_bottom: Option<Edge> = None;
    let mut image_indexes: Vec<Vec<usize>> = vec![vec![0; width_in_nr_tiles]; width_in_nr_tiles];
    let mut transform_nrs: Vec<Vec<usize>> = vec![vec![0; width_in_nr_tiles]; width_in_nr_tiles];
    for tile_y in 0..width_in_nr_tiles {
        for tile_x in 0..width_in_nr_tiles {
            let mut found_index = usize::MAX;
            let mut found_flop = usize::MAX;
            if tile_x == 0 && tile_y == 0 {
                // First corner, find transform so that right and bottom matches something.
                for flop in 0..8 {
                    let bottom = tiles[corner_index].get_edge(2, flop);
                    let right = tiles[corner_index].get_edge(1, flop);
                    if bottom.matches_edge_in_another_tile(tiles)
                        && right.matches_edge_in_another_tile(tiles)
                    {
                        found_index = corner_index;
                        found_flop = flop;
                        break;
                    }
                }
            } else {
                // Find tile and transform to the right of existing (or under for x==0)
                'outer: for tile_index in 0..tiles.len() {
                    if used_tiles[tile_index] {
                        continue;
                    }
                    for flop in 0..8 {
                        if (tile_x > 0
                            && tiles[tile_index].get_edge(3, flop) == last_right.unwrap())
                            || (tile_x == 0
                                && tiles[tile_index].get_edge(0, flop) == last_bottom.unwrap())
                        {
                            found_index = tile_index;
                            found_flop = flop;
                            break 'outer;
                        }
                    }
                }
            }
            image_indexes[tile_y][tile_x] = found_index;
            transform_nrs[tile_y][tile_x] = found_flop;
            used_tiles[found_index] = true;

            let right = tiles[found_index].get_edge(1, found_flop);
            last_right = Some(right);
            if tile_x == 0 {
                let bottom = tiles[found_index].get_edge(2, found_flop);
                last_bottom = Some(bottom);
            }
        }
    }

    let big_map = BigImage {
        image_indexes,
        transform_nrs,
    };

    let monster = "..................#.
#....##....##....###
.#..#..#..#..#..#...";
    let nr_monster_pixels = monster.chars().filter(|c| *c == '#').count();
    let monster: Vec<&str> = monster.split('\n').collect();
    let monster_width = monster[0].len();
    let monster_height = monster.len();

    let big_map_width = width_in_nr_tiles * (tile_width - 2);

    let mut nr_matches = 0;
    // Check if monster matches for every position and flop of map.
    for flop in 0..8 {
        for check_pos_y in 0..(big_map_width - monster_height) {
            for check_pos_x in 0..(big_map_width - monster_width) {
                let mut is_match = true;
                'monster_search: for y in 0..monster_height {
                    for x in 0..monster_width {
                        if &monster[y][x..(x + 1)] == "#"
                            && !big_map.get_pixel(tiles, check_pos_x + x, check_pos_y + y, flop)
                        {
                            is_match = false;
                            break 'monster_search;
                        }
                    }
                }
                if is_match {
                    nr_matches += 1;
                }
            }
        }
    }

    let mut total_count = 0;
    for y in 0..big_map_width {
        for x in 0..big_map_width {
            if big_map.get_pixel(tiles, x, y, 0) {
                total_count += 1;
            }
        }
    }

    let result = total_count - nr_matches * nr_monster_pixels as i64;
    println!("Part 2: {}", result);
    result
}

fn parse(content: &str) -> Vec<Image> {
    let mut results: Vec<Image> = Vec::new();
    for tile in content.trim().split("\n\n") {
        let mut tile_it = tile.split('\n');
        let tile_id_str = tile_it.next().unwrap().trim();
        let tile_id: usize = tile_id_str[5..(tile_id_str.len() - 1)].parse().unwrap();
        let pixels = tile_it
            .map(|x| x.chars().map(|ch| ch == '#').collect())
            .collect();
        results.push(Image {
            id: tile_id,
            pixels,
        });
    }
    results
}

fn main() {
    let content = fs::read_to_string("../../inputs/day20_input.txt").expect("Cannot open file!");
    let expressions = parse(&content);

    let result_p1 = part1(&expressions);
    assert_eq!(result_p1, 29293767579581);
    let result_p2 = part2(&expressions);
    assert_eq!(result_p2, 1989);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        let v = parse(input);
        let result = part1(&v);
        assert_eq!(result, 20899048083289);
        let result = part2(&v);
        assert_eq!(result, 273);
    }
}
