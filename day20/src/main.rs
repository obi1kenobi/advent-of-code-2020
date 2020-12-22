use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Tile {
    id: i64,
    image: Vec<Vec<char>>,
}

fn parse_tile(tile_data: &str) -> Tile {
    let mut data = tile_data.split("\n");

    let header = data.next().unwrap().trim();
    assert!(header.starts_with("Tile "));
    assert!(header.ends_with(":"));

    let id: i64 = header["Tile ".len()..header.len() - 1].parse().unwrap();
    let image: Vec<Vec<char>> = data.map(|x| x.chars().collect()).collect();

    assert!(image.len() == image[0].len()); // tiles had better be square :)

    Tile { id, image }
}

#[derive(Debug)]
struct TileVariant {
    id: i64,
    variant: usize,
    image: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

impl Edge {
    fn variants() -> impl Iterator<Item = Edge> {
        [Edge::Top, Edge::Right, Edge::Bottom, Edge::Left]
            .iter()
            .copied()
    }

    fn opposite(&self) -> Edge {
        match self {
            Edge::Top => Edge::Bottom,
            Edge::Bottom => Edge::Top,
            Edge::Left => Edge::Right,
            Edge::Right => Edge::Left,
        }
    }

    fn is_adjacent(&self, other: &Edge) -> bool {
        match self {
            Edge::Top | Edge::Bottom => other == &Edge::Left || other == &Edge::Right,
            Edge::Left | Edge::Right => other == &Edge::Top || other == &Edge::Bottom,
        }
    }
}

impl TileVariant {
    fn get_edge(&self, edge: &Edge) -> Vec<char> {
        match edge {
            Edge::Top => self.get_top_edge(),
            Edge::Right => self.get_right_edge(),
            Edge::Bottom => self.get_bottom_edge(),
            Edge::Left => self.get_left_edge(),
        }
    }

    fn get_top_edge(&self) -> Vec<char> {
        self.image[0].clone()
    }

    fn get_bottom_edge(&self) -> Vec<char> {
        self.image[self.image.len() - 1].clone()
    }

    fn get_left_edge(&self) -> Vec<char> {
        self.image.iter().map(|row| row[0]).collect()
    }

    fn get_right_edge(&self) -> Vec<char> {
        self.image.iter().map(|row| row[row.len() - 1]).collect()
    }
}

fn right_rotate_image(image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = image.clone();
    let image_dimension = image.len();
    let max_image_coord = image_dimension - 1;

    assert_eq!(image_dimension, image[0].len());

    for x in 0..image.len() {
        for y in 0..image[0].len() {
            result[y][max_image_coord - x] = image[x][y];
        }
    }

    result
}

fn horizontal_flip_image(image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = image.clone();
    let image_dimension = image.len();
    let max_image_coord = image_dimension - 1;

    assert_eq!(image_dimension, image[0].len());

    for x in 0..image.len() {
        for y in 0..image[0].len() {
            result[max_image_coord - x][y] = image[x][y];
        }
    }

    result
}

fn make_all_tile_variants(tile: &Tile) -> Vec<TileVariant> {
    let original_image = &tile.image;
    let mut result = vec![];

    let mut rotated_image = original_image.clone();
    for rotation_id in 0..=3 {
        result.push(TileVariant {
            id: tile.id,
            variant: rotation_id,
            image: rotated_image.clone(),
        });
        rotated_image = right_rotate_image(&rotated_image);
    }

    let mut flipped_image = horizontal_flip_image(&original_image);
    for rotation_id in 0..=3 {
        result.push(TileVariant {
            id: tile.id,
            variant: 4 + rotation_id,
            image: flipped_image.clone(),
        });
        flipped_image = right_rotate_image(&flipped_image);
    }

    result
}

fn get_all_variants_for_tile_set(tiles: &Vec<Tile>) -> HashMap<(i64, usize), TileVariant> {
    tiles
        .iter()
        .flat_map(|x| make_all_tile_variants(x))
        .map(|x| ((x.id, x.variant), x))
        .collect()
}

fn make_edge_index(
    tile_variants: &HashMap<(i64, usize), TileVariant>,
) -> HashMap<Vec<char>, Vec<((i64, usize), Edge)>> {
    let mut result = HashMap::new();

    for (key, variant) in tile_variants {
        for edge_kind in Edge::variants() {
            let edge = variant.get_edge(&edge_kind);

            result
                .entry(edge)
                .or_insert(Vec::new())
                .push((*key, edge_kind));
        }
    }

    result
}

fn get_other_tile_matches_for_variant_edge(
    edge_index: &HashMap<Vec<char>, Vec<((i64, usize), Edge)>>,
    tile_id: i64,
    edge: &Vec<char>,
    edge_direction: &Edge,
) -> Vec<((i64, usize), Edge)> {
    let opposite_edge = edge_direction.opposite();

    edge_index[edge]
        .iter()
        .filter(|((id, _), edge_dir)| *id != tile_id && *edge_dir == opposite_edge)
        .cloned()
        .collect()
}

fn find_corner_tiles(
    tiles: &Vec<Tile>,
    tile_variants: &HashMap<(i64, usize), TileVariant>,
    edge_index: &HashMap<Vec<char>, Vec<((i64, usize), Edge)>>,
) -> Vec<i64> {
    let mut result = vec![];

    for tile in tiles {
        let mut best_matched_variant_unmatched_directions: Vec<Edge> = Edge::variants().collect();

        for variant_id in 0..8usize {
            let key = (tile.id, variant_id);
            let variant = &tile_variants[&key];

            let mut unmatched_edge_directions: Vec<Edge> = Vec::new();
            for edge_dir in Edge::variants() {
                let possible_matches = get_other_tile_matches_for_variant_edge(
                    edge_index,
                    tile.id,
                    &variant.get_edge(&edge_dir),
                    &edge_dir,
                );
                assert!(possible_matches.len() <= 1);

                if possible_matches.is_empty() {
                    unmatched_edge_directions.push(edge_dir);
                }
            }

            if best_matched_variant_unmatched_directions.len() > unmatched_edge_directions.len() {
                best_matched_variant_unmatched_directions = unmatched_edge_directions;
            }
        }

        assert!(best_matched_variant_unmatched_directions.len() <= 2);
        if best_matched_variant_unmatched_directions.len() == 2 {
            // two unmatched edges -- assert they form a corner
            let (dir_a, dir_b) = (
                best_matched_variant_unmatched_directions[0],
                best_matched_variant_unmatched_directions[1],
            );
            assert!(dir_a.is_adjacent(&dir_b));

            result.push(tile.id);
        }
    }

    result.sort(); // for determinism
    assert_eq!(4, result.len());

    result
}

fn solve_part1(
    tiles: &Vec<Tile>,
    tile_variants: &HashMap<(i64, usize), TileVariant>,
    _tile_size: usize,
) -> i64 {
    let edge_index = make_edge_index(tile_variants);

    find_corner_tiles(tiles, tile_variants, &edge_index)
        .iter()
        .fold(1, |acc, elem| acc * elem)
}

fn int_sqrt(value: usize) -> usize {
    (0..=value).filter(|v| v * v == value).next().unwrap()
}

fn add_top_left_corner_to_tile_map(
    tile_variants: &HashMap<(i64, usize), TileVariant>,
    edge_index: &HashMap<Vec<char>, Vec<((i64, usize), Edge)>>,
    tile_map: &mut Vec<Vec<(i64, usize)>>,
    corner_id: i64,
) {
    for variant_id in 0..8 {
        let tile_variant_key = (corner_id, variant_id);
        let variant = &tile_variants[&tile_variant_key];

        let top_edge_matches = get_other_tile_matches_for_variant_edge(
            edge_index,
            corner_id,
            &variant.get_top_edge(),
            &Edge::Top,
        );
        let left_edge_matches = get_other_tile_matches_for_variant_edge(
            edge_index,
            corner_id,
            &variant.get_left_edge(),
            &Edge::Left,
        );
        if top_edge_matches.is_empty() && left_edge_matches.is_empty() {
            tile_map[0][0] = tile_variant_key;
            return;
        }
    }

    unreachable!();
}

fn find_tile_variant_with_neighbor(
    edge_index: &HashMap<Vec<char>, Vec<((i64, usize), Edge)>>,
    neighbor_tile_variant_key: &(i64, usize),
    neighbor_edge_direction: &Edge,
    neighbor_edge: &Vec<char>,
) -> (i64, usize) {
    let result_tile_edge_direction = neighbor_edge_direction.opposite();

    let possible_results: Vec<_> = get_other_tile_matches_for_variant_edge(
        edge_index,
        neighbor_tile_variant_key.0,
        neighbor_edge,
        neighbor_edge_direction,
    )
    .iter()
    .filter_map(|&(key, direction)| {
        if direction == result_tile_edge_direction {
            Some(key)
        } else {
            None
        }
    })
    .collect();

    assert!(possible_results.len() == 1);
    possible_results[0]
}

fn make_empty_tile_map(tiles: &Vec<Tile>) -> Vec<Vec<(i64, usize)>> {
    let tile_map_side = int_sqrt(tiles.len());
    let mut tile_map: Vec<Vec<(i64, usize)>> = Vec::new();
    let mut tile_map_row: Vec<(i64, usize)> = Vec::new();
    tile_map_row.resize(tile_map_side, (-1, 0));
    tile_map.resize(tile_map_side, tile_map_row);

    tile_map
}

fn fill_tile_map(
    tile_variants: &HashMap<(i64, usize), TileVariant>,
    edge_index: &HashMap<Vec<char>, Vec<((i64, usize), Edge)>>,
    mut tile_map: &mut Vec<Vec<(i64, usize)>>,
    corner_tiles: &Vec<i64>,
) {
    // fill in the top left corner
    add_top_left_corner_to_tile_map(
        tile_variants,
        &edge_index,
        &mut tile_map,
        corner_tiles.first().unwrap().clone(),
    );

    // fill in the top row of the tile map
    for y in 1..tile_map[0].len() {
        let neighbor_tile_variant_key = &tile_map[0][y - 1];
        let neighbor_edge_direction = Edge::Right;
        let neighbor_edge = &tile_variants[neighbor_tile_variant_key].get_right_edge();
        tile_map[0][y] = find_tile_variant_with_neighbor(
            &edge_index,
            neighbor_tile_variant_key,
            &neighbor_edge_direction,
            neighbor_edge,
        );
    }

    // fill in the rest of the tile map
    for x in 1..tile_map.len() {
        for y in 0..tile_map[0].len() {
            let neighbor_tile_variant_key = &tile_map[x - 1][y];
            let neighbor_edge_direction = Edge::Bottom;
            let neighbor_edge = &tile_variants[neighbor_tile_variant_key].get_bottom_edge();

            tile_map[x][y] = find_tile_variant_with_neighbor(
                &edge_index,
                neighbor_tile_variant_key,
                &neighbor_edge_direction,
                neighbor_edge,
            );
        }
    }
}

fn make_combined_image_from_tile_map(
    tile_variants: &HashMap<(i64, usize), TileVariant>,
    tile_map: &Vec<Vec<(i64, usize)>>,
    tile_size: usize,
) -> Vec<Vec<char>> {
    let borderless_tile_size = tile_size - 2;

    let full_image_dimension = borderless_tile_size * tile_map.len();
    let mut full_image: Vec<Vec<char>> = Vec::new();
    let mut full_image_row: Vec<char> = Vec::new();
    full_image_row.resize(full_image_dimension, '?');
    full_image.resize(full_image_dimension, full_image_row);

    for tile_x in 0..tile_map.len() {
        let full_image_offset_x = tile_x * borderless_tile_size;
        for tile_y in 0..tile_map[0].len() {
            let full_image_offset_y = tile_y * borderless_tile_size;

            let tile_variant = &tile_variants[&tile_map[tile_x][tile_y]];
            for img_x in 1..(tile_size - 1) {
                for img_y in 1..(tile_size - 1) {
                    let full_image_x = full_image_offset_x + (img_x - 1);
                    let full_image_y = full_image_offset_y + (img_y - 1);

                    assert_eq!(full_image[full_image_x][full_image_y], '?');
                    full_image[full_image_x][full_image_y] = tile_variant.image[img_x][img_y];
                }
            }
        }
    }

    for x in 0..full_image.len() {
        for y in 0..full_image[0].len() {
            assert_ne!(full_image[x][y], '?');
        }
    }

    full_image
}

fn get_monster_image_indexes() -> (usize, usize, Vec<(usize, usize)>) {
    let monster = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];

    let monster_map: Vec<Vec<char>> = monster.iter().map(|row| row.chars().collect()).collect();

    let monster_indexes = monster_map
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            std::iter::repeat(row_index).zip(row.iter().enumerate().filter_map(|(col_index, c)| {
                if *c == '#' {
                    Some(col_index)
                } else {
                    None
                }
            }))
        })
        .collect();

    (monster_map.len(), monster_map[0].len(), monster_indexes)
}

fn find_sea_monster_data(
    full_image: &Vec<Vec<char>>,
    monster_x_dim: usize,
    monster_y_dim: usize,
    monster_indexes: &Vec<(usize, usize)>,
) -> Option<usize> {
    let mut monsters_found = 0usize;
    for root_x in 0..(full_image.len() - monster_x_dim) {
        for root_y in 0..(full_image[0].len() - monster_y_dim) {
            let mut monster_found = true;
            for (monster_x, monster_y) in monster_indexes.iter().cloned() {
                let x = root_x + monster_x;
                let y = root_y + monster_y;

                if full_image[x][y] != '#' {
                    monster_found = false;
                    break;
                }
            }

            if monster_found {
                monsters_found += 1;
            }
        }
    }

    if monsters_found == 0 {
        None
    } else {
        let total_roughness: usize = full_image
            .iter()
            .map(|row| -> usize {
                row.iter()
                    .filter_map(|y| if *y == '#' { Some(1usize) } else { None })
                    .sum()
            })
            .sum();
        let monster_roughness = monsters_found * monster_indexes.len();

        assert!(total_roughness >= monster_roughness);

        Some(total_roughness - monster_roughness)
    }
}

fn solve_part2(
    tiles: &Vec<Tile>,
    tile_variants: &HashMap<(i64, usize), TileVariant>,
    tile_size: usize,
) -> usize {
    let edge_index = make_edge_index(tile_variants);
    let mut tile_map = make_empty_tile_map(tiles);
    let corner_tiles = find_corner_tiles(tiles, tile_variants, &edge_index);

    fill_tile_map(tile_variants, &edge_index, &mut tile_map, &corner_tiles);

    let full_image = make_combined_image_from_tile_map(tile_variants, &tile_map, tile_size);

    let (monster_x_dim, monster_y_dim, monster_indexes) = get_monster_image_indexes();

    let mut rotated_full_image = full_image.clone();
    let mut flipped_full_image = horizontal_flip_image(&full_image);
    let mut rotations: usize = 0;
    loop {
        if let Some(solution) = find_sea_monster_data(
            &rotated_full_image,
            monster_x_dim,
            monster_y_dim,
            &monster_indexes,
        )
        .or_else(|| {
            find_sea_monster_data(
                &flipped_full_image,
                monster_x_dim,
                monster_y_dim,
                &monster_indexes,
            )
        }) {
            break solution;
        }

        rotated_full_image = right_rotate_image(&rotated_full_image);
        flipped_full_image = right_rotate_image(&flipped_full_image);

        rotations += 1;
        assert!(rotations < 4);
    }
}

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day20/input.txt",
    )
    .unwrap();

    let tiles: Vec<_> = contents.trim().split("\n\n").map(parse_tile).collect();
    let tile_variants = get_all_variants_for_tile_set(&tiles);
    let tile_size = tiles[0].image.len();

    println!("{}", solve_part1(&tiles, &tile_variants, tile_size));
    println!("{}", solve_part2(&tiles, &tile_variants, tile_size));
}

#[cfg(test)]
mod tests {
    use crate::{horizontal_flip_image, right_rotate_image};

    #[test]
    fn test_rotation() {
        let original = vec![vec!['1', '2'], vec!['3', '4']];
        let one_rotation = vec![vec!['3', '1'], vec!['4', '2']];

        assert_eq!(one_rotation, right_rotate_image(&original));

        assert_eq!(
            original,
            right_rotate_image(&right_rotate_image(&right_rotate_image(&one_rotation)))
        );
    }

    #[test]
    fn test_reflection() {
        let original = vec![vec!['1', '2'], vec!['3', '4']];
        let reflected = vec![vec!['3', '4'], vec!['1', '2']];

        assert_eq!(reflected, horizontal_flip_image(&original));
        assert_eq!(original, horizontal_flip_image(&reflected));
    }
}
