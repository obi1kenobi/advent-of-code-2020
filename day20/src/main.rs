use std::{
    collections::{HashMap, HashSet},
    fs, todo,
};

#[derive(Debug)]
struct Tile {
    id: i64,
    image: Vec<Vec<char>>,
}

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day20/sample_input.txt",
    )
    .unwrap();

    let tiles: Vec<_> = contents.trim().split("\n\n").map(parse_tile).collect();
    let tiles_by_id: HashMap<i64, &Tile> = tiles.iter().map(|x| (x.id, x)).collect();
    let tile_size = tiles[0].image.len();

    println!("{}", solve_part1(&tiles_by_id, tile_size));
    println!("{}", solve_part2(&tiles_by_id, tile_size));
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

const START_X: [usize; 8] = [0, 0, 1, 1, 1, 0, 0, 1];
const START_Y: [usize; 8] = [0, 1, 1, 0, 0, 0, 1, 1];
const ALL_DX: [i64; 8] = [0, 1, 0, -1, 0, 1, 0, -1];
const ALL_DY: [i64; 8] = [1, 0, -1, 0, 1, 0, -1, 0];
const EDGE_NAMES: [&str; 8] = [
    "top", "right", "bottom", "left", "bottom", "left", "top", "right",
];
const EDGE_ORIENTED_NAMES: [&str; 8] = [
    "top-rightward",
    "right-bottomward",
    "bottom-leftward",
    "left-topward",
    "bottom-rightward",
    "left-bottomward",
    "top-leftward",
    "right-topward",
];

fn get_offset_and_rotation_matrix_from_edge_orientation(
    tile_useful_size: usize,
    bottom_edge_orientation: usize,
) -> (usize, usize, i64, i64, i64, i64) {
    let (mut dxdx, mut dxdy, mut dydx, mut dydy) = (1, 0, 0, 1);
    let (rotation, horizontal_reflection) = match EDGE_ORIENTED_NAMES[bottom_edge_orientation] {
        "top-rightward" => {
            // horizontal reflection
            (0, 1)
        }
        "right-bottomward" => {
            // left rotation + horizontal reflection
            (3, 1)
        }
        "bottom-leftward" => {
            // vertical reflection = double rotation + horizontal reflection
            (2, 1)
        }
        "left-topward" => {
            // right rotation + horizontal reflection
            (1, 1)
        }
        "bottom-rightward" => {
            // already correct as-is
            (0, 0)
        }
        "left-bottomward" => {
            // right rotation
            (1, 0)
        }
        "top-leftward" => {
            // double rotation
            (2, 0)
        }
        "right-topward" => {
            // left rotation
            (3, 0)
        }
        _ => unreachable!(),
    };
    for _ in 0..rotation {
        let (old_dxdx, old_dxdy, old_dydx, old_dydy) = (dxdx, dxdy, dydx, dydy);
        let (rotation_dxdx, rotation_dxdy, rotation_dydx, rotation_dydy) = (0, -1, 1, 0);

        /*
            (rotation_dxdx, rotation_dxdy)  *  (dxdx, dxdy)
            (rotation_dydx, rotation_dydy)     (dydx, dydy)
        */

        dxdx = rotation_dxdx * old_dxdx + rotation_dxdy * old_dydx;
        dxdy = rotation_dxdx * old_dxdy + rotation_dxdy * old_dydy;
        dydx = rotation_dydx * old_dxdx + rotation_dydy * old_dydx;
        dydy = rotation_dydx * old_dxdy + rotation_dydy * old_dydy;
    }
    for _ in 0..horizontal_reflection {
        let (old_dxdx, old_dxdy, old_dydx, old_dydy) = (dxdx, dxdy, dydx, dydy);
        let (reflection_dxdx, reflection_dxdy, reflection_dydx, reflection_dydy) = (-1, 0, 0, 1);

        dxdx = reflection_dxdx * old_dxdx + reflection_dxdy * old_dydx;
        dxdy = reflection_dxdx * old_dxdy + reflection_dxdy * old_dydy;
        dydx = reflection_dydx * old_dxdx + reflection_dydy * old_dydx;
        dydy = reflection_dydx * old_dxdy + reflection_dydy * old_dydy;
    }

    let last_valid_coordinate = tile_useful_size - 1;
    let (offset_x, offset_y) = match (rotation, horizontal_reflection) {
        (0, 0) => (0, 0),
        (0, 1) => (last_valid_coordinate, 0),
        (1, 0) => (0, last_valid_coordinate),
        (1, 1) => (last_valid_coordinate, last_valid_coordinate),
        (2, 0) => (last_valid_coordinate, last_valid_coordinate),
        (2, 1) => (0, last_valid_coordinate),
        (3, 0) => (last_valid_coordinate, 0),
        (3, 1) => (0, 0),
        _ => unreachable!(),
    };

    (offset_x, offset_y, dxdx, dxdy, dydx, dydy)
}

fn get_opposite_edge(edge_orientation: usize) -> usize {
    // for a given tile orientation and edge, return the opposite edge of the same tile orientation
    (edge_orientation + 4) % 8
}

fn get_post_transform_right_edge(post_transform_bottom_edge: usize) -> usize {
    (post_transform_bottom_edge + 7) % 8
    // match post_transform_bottom_edge {
    //     0 => 7,
    //     1 => 4,
    //     2 => 5,
    //     3 => 6,
    //     4 => 1,
    //     5 => 2,
    //     6 => 3,
    //     7 => 4,
    //     _ => unreachable!(),
    // }
}

fn get_right_neighbor_edge(edge_orientation: usize) -> usize {
    if edge_orientation < 4 {
        (edge_orientation + 1) % 4
    } else {
        ((edge_orientation + 1) % 4) + 4
    }
}

fn get_left_neighbor_edge(edge_orientation: usize) -> usize {
    if edge_orientation < 4 {
        (edge_orientation + 3) % 4
    } else {
        ((edge_orientation + 3) % 4) + 4
    }
}

fn make_matching_edge_map(
    tiles_by_id: &HashMap<i64, &Tile>,
    tile_size: usize,
) -> (
    HashMap<i64, Vec<Vec<char>>>,
    HashMap<Vec<char>, Vec<(i64, usize)>>,
) {
    let mut edges: HashMap<Vec<char>, Vec<(i64, usize)>> = HashMap::new();
    let mut edges_by_tile_id: HashMap<i64, Vec<Vec<char>>> = HashMap::new();

    let max_pos = tile_size - 1;
    let start_x: Vec<_> = START_X
        .iter()
        .map(|&x| if x != 0 { max_pos } else { 0 })
        .collect();
    let start_y: Vec<_> = START_Y
        .iter()
        .map(|&x| if x != 0 { max_pos } else { 0 })
        .collect();

    for tile in tiles_by_id.values() {
        let positions_iter = start_x
            .iter()
            .zip(start_y.iter())
            .zip(ALL_DX.iter())
            .zip(ALL_DY.iter());
        for (orientation, (((corner_x, corner_y), dx), dy)) in positions_iter.enumerate() {
            let mut edge: Vec<char> = Vec::new();
            let mut x = *corner_x;
            let mut y = *corner_y;
            for _ in 0..tile_size {
                edge.push(tile.image[x][y]);
                x = (x as i64 + dx) as usize;
                y = (y as i64 + dy) as usize;
            }

            edges_by_tile_id
                .entry(tile.id)
                .or_insert(Vec::new())
                .push(edge.clone());

            edges
                .entry(edge)
                .or_insert(Vec::new())
                .push((tile.id, orientation));
        }
    }

    for id_options in edges.values() {
        assert!(id_options.len() < 3); // if this is true, it will greatly simplify our solution
    }

    (edges_by_tile_id, edges)
}

fn find_corner_pieces(edges: &HashMap<Vec<char>, Vec<(i64, usize)>>) -> Vec<i64> {
    // we're only looking for corner pieces -- those are the only ones with two unmatched edges
    let mut matched_edges: HashMap<i64, HashSet<&str>> = HashMap::new();
    for matching_edge_info in edges.values() {
        if matching_edge_info.len() < 2 {
            continue;
        }
        for &(tile_id, orientation_id) in matching_edge_info.iter() {
            let orientation = EDGE_NAMES[orientation_id];
            matched_edges
                .entry(tile_id)
                .or_insert(HashSet::new())
                .insert(orientation);
        }
    }

    let corner_ids: Vec<i64> = matched_edges
        .iter()
        .filter(|&(_, value)| value.len() == 2)
        .map(|(&key, _)| key)
        .collect();
    assert!(corner_ids.len() == 4);

    corner_ids
}

fn transform_tile_coords_to_final_coords(
    tile_x: usize,
    tile_y: usize,
    to_final_offset_x: usize, // the (x, y) offset where (rel_x, rel_y) = (0, 0) needs to go
    to_final_offset_y: usize,
    to_final_dxdx: i64, // the rotation matrix to convert tile coords to final image coords
    to_final_dxdy: i64,
    to_final_dydx: i64,
    to_final_dydy: i64,
) -> (i64, i64) {
    let rel_x = tile_x as i64 - 1;
    let rel_y = tile_y as i64 - 1;
    let mat_x = (rel_x * to_final_dxdx) + (rel_y * to_final_dxdy);
    let mat_y = (rel_x * to_final_dydx) + (rel_y * to_final_dydy);

    let off_x = mat_x + to_final_offset_x as i64;
    let off_y = mat_y + to_final_offset_y as i64;

    return (off_x, off_y);
}

fn get_final_tile_edge(
    tile: &Tile,
    desired_edge_x: Option<i64>,
    desired_edge_y: Option<i64>,
    to_final_offset_x: usize, // the (x, y) offset where (rel_x, rel_y) = (0, 0) needs to go
    to_final_offset_y: usize,
    to_final_dxdx: i64, // the rotation matrix to convert tile coords to final image coords
    to_final_dxdy: i64,
    to_final_dydx: i64,
    to_final_dydy: i64,
) -> Vec<char> {
    let mut result = Vec::new();
    result.resize(tile.image.len(), '?');

    let max_valid_tile_coord = tile.image.len() - 1;

    // just brute-force check all edges of the tile
    for x in 0..tile.image.len() {
        for y in vec![0, max_valid_tile_coord] {
            let (fin_x, fin_y) = transform_tile_coords_to_final_coords(
                x,
                y,
                to_final_offset_x,
                to_final_offset_y,
                to_final_dxdx,
                to_final_dxdy,
                to_final_dydx,
                to_final_dydy,
            );
            if let Some(des_x) = desired_edge_x {
                if fin_x == des_x {
                    result[(fin_y + 1) as usize] = tile.image[x][y];
                }
            } else if let Some(des_y) = desired_edge_y {
                if fin_y == des_y {
                    result[(fin_x + 1) as usize] = tile.image[x][y];
                }
            }
        }
    }
    for y in 0..tile.image.len() {
        for x in vec![0, max_valid_tile_coord] {
            let (fin_x, fin_y) = transform_tile_coords_to_final_coords(
                x,
                y,
                to_final_offset_x,
                to_final_offset_y,
                to_final_dxdx,
                to_final_dxdy,
                to_final_dydx,
                to_final_dydy,
            );
            if let Some(des_x) = desired_edge_x {
                if fin_x == des_x {
                    result[(fin_y + 1) as usize] = tile.image[x][y];
                }
            } else if let Some(des_y) = desired_edge_y {
                if fin_y == des_y {
                    result[(fin_x + 1) as usize] = tile.image[x][y];
                }
            }
        }
    }

    for val in &result {
        assert!(*val != '?');
    }

    result
}

fn write_tile_to_final_image(
    final_image: &mut Vec<Vec<char>>,
    tile: &Tile,
    tile_xpos: usize,
    tile_ypos: usize,
    to_final_offset_x: usize, // the (x, y) offset where (rel_x, rel_y) = (0, 0) needs to go
    to_final_offset_y: usize,
    to_final_dxdx: i64, // the rotation matrix to convert tile coords to final image coords
    to_final_dxdy: i64,
    to_final_dydx: i64,
    to_final_dydy: i64,
) {
    let tile_useful_size = tile.image.len() - 2;
    for orig_x in 1..=tile_useful_size {
        let rel_x = orig_x as i64 - 1;
        for orig_y in 1..=tile_useful_size {
            let rel_y = orig_y as i64 - 1;
            let mat_x = (rel_x * to_final_dxdx) + (rel_y * to_final_dxdy);
            let mat_y = (rel_x * to_final_dydx) + (rel_y * to_final_dydy);

            let (off_x, off_y) = transform_tile_coords_to_final_coords(
                orig_x,
                orig_y,
                to_final_offset_x,
                to_final_offset_y,
                to_final_dxdx,
                to_final_dxdy,
                to_final_dydx,
                to_final_dydy,
            );
            assert!(off_x >= 0);
            assert!(off_y >= 0);
            assert!(off_x < tile_useful_size as i64);
            assert!(off_y < tile_useful_size as i64);

            let final_x = (tile_xpos * tile_useful_size) + off_x as usize;
            let final_y = (tile_ypos * tile_useful_size) + off_y as usize;

            // must not be overwriting a previous write
            assert!(final_image[final_x][final_y] == '?');
            final_image[final_x][final_y] = tile.image[orig_x][orig_y];
        }
    }
}

fn solve_part1(tiles_by_id: &HashMap<i64, &Tile>, tile_size: usize) -> i64 {
    let (_, edges) = make_matching_edge_map(&tiles_by_id, tile_size);
    let corner_ids: Vec<i64> = find_corner_pieces(&edges);

    corner_ids.iter().fold(1, |acc, elem| acc * elem)
}

fn find_adjacent_tile_and_orientation_for_edge(
    fixed_tile: i64,
    fixed_edge: &Vec<char>,
    edges: &HashMap<Vec<char>, Vec<(i64, usize)>>,
) -> (i64, usize) {
    let results: Vec<_> = edges
        .get(fixed_edge)
        .unwrap()
        .iter()
        .filter(|&(tile_id, _)| *tile_id != fixed_tile)
        .cloned()
        .collect();

    println!("{} {:?}", fixed_tile, fixed_edge);

    assert!(results.len() == 1);
    results.first().unwrap().clone()
}

fn solve_part2(tiles_by_id: &HashMap<i64, &Tile>, tile_size: usize) -> i64 {
    let (edges_by_tile_id, edges) = make_matching_edge_map(&tiles_by_id, tile_size);

    let tile_count = tiles_by_id.len();
    let sqrt_tile_count = (0..=tile_count)
        .filter(|&x| x * x == tile_count)
        .next()
        .unwrap();
    let tile_useful_size = tile_size - 2; // -2 since edges are discarded
    let final_img_dimension = tile_useful_size * sqrt_tile_count;
    let mut full_image: Vec<Vec<char>> = Vec::new();
    let mut sample_vec: Vec<char> = Vec::new();
    sample_vec.resize(final_img_dimension, '?'); // illegal char so we can easily see uninitialized data
    full_image.resize(final_img_dimension, sample_vec);

    let mut tile_map: Vec<Vec<i64>> = Vec::new();
    let mut sample_tile_map_vec: Vec<i64> = Vec::new();
    sample_tile_map_vec.resize(sqrt_tile_count, 0);
    tile_map.resize(sqrt_tile_count, sample_tile_map_vec);

    // always in either (left, right) or (upper, lower) order
    let mut tile_tile_relative_orientations: HashMap<(i64, i64), (usize, usize)> = HashMap::new();

    let corner_ids: Vec<i64> = find_corner_pieces(&edges);
    let first_corner_tile_id = 1951; // *corner_ids.iter().min().unwrap(); // min for determinism

    tile_map[0][0] = first_corner_tile_id;
    let first_corner_edges: Vec<_> = edges_by_tile_id
        .get(&first_corner_tile_id)
        .unwrap()
        .iter()
        .filter(|&edge| edges.get(edge).unwrap_or(&Vec::new()).len() == 2)
        .collect();
    // min/max for determinism
    let first_corner_bottom_edge = *first_corner_edges.iter().min().unwrap();
    let first_corner_right_edge = *first_corner_edges.iter().max().unwrap();

    let first_corner_bottom_edge_orientation = edges
        .get(first_corner_bottom_edge)
        .unwrap()
        .iter()
        .filter(|&(tile_id, _)| *tile_id == first_corner_tile_id)
        .map(|&(_, orientation)| orientation)
        .next()
        .unwrap();
    let first_corner_right_edge_orientation = edges
        .get(first_corner_right_edge)
        .unwrap()
        .iter()
        .filter(|&(tile_id, _)| *tile_id == first_corner_tile_id)
        .map(|&(_, orientation)| orientation)
        .next()
        .unwrap();

    // fill in the first column
    let mut fixed_edge = first_corner_bottom_edge;
    let mut fixed_edge_orientation = first_corner_bottom_edge_orientation;
    for tile_x in 1..sqrt_tile_count {
        let fixed_tile = tile_map[tile_x - 1][0];
        let (adjacent_tile, adjacent_orientation) =
            find_adjacent_tile_and_orientation_for_edge(fixed_tile, &fixed_edge, &edges);

        tile_map[tile_x][0] = adjacent_tile;
        tile_tile_relative_orientations.insert(
            (fixed_tile, adjacent_tile),
            (fixed_edge_orientation, adjacent_orientation),
        );

        fixed_edge_orientation = get_opposite_edge(adjacent_orientation);
        fixed_edge = &edges_by_tile_id[&adjacent_tile][fixed_edge_orientation];
    }

    // fill in the first row
    fixed_edge = first_corner_right_edge;
    fixed_edge_orientation = first_corner_right_edge_orientation;
    for tile_y in 1..sqrt_tile_count {
        let fixed_tile = tile_map[0][tile_y - 1];
        let (adjacent_tile, adjacent_orientation) =
            find_adjacent_tile_and_orientation_for_edge(fixed_tile, &fixed_edge, &edges);

        tile_map[0][tile_y] = adjacent_tile;
        tile_tile_relative_orientations.insert(
            (fixed_tile, adjacent_tile),
            (fixed_edge_orientation, adjacent_orientation),
        );

        fixed_edge_orientation = get_opposite_edge(adjacent_orientation);
        fixed_edge = &edges_by_tile_id[&adjacent_tile][fixed_edge_orientation];
    }

    // fill in the rest of the tile map
    for tile_x in 1..sqrt_tile_count {
        let first_tile_in_row = tile_map[tile_x][0];
        let tile_above_first_in_row = tile_map[tile_x - 1][0];
        let (_, first_tile_in_row_upper_orientation) = tile_tile_relative_orientations
            .get(&(tile_above_first_in_row, first_tile_in_row))
            .unwrap();

        fixed_edge_orientation = get_right_neighbor_edge(*first_tile_in_row_upper_orientation);
        fixed_edge = &edges_by_tile_id[&first_tile_in_row][fixed_edge_orientation];
        if edges[fixed_edge].len() == 1 {
            // we picked an edge that happens to be on the rim of the final image
            fixed_edge_orientation = get_left_neighbor_edge(*first_tile_in_row_upper_orientation);
            fixed_edge = &edges_by_tile_id[&first_tile_in_row][fixed_edge_orientation];
            assert!(edges[fixed_edge].len() == 2);
        }

        for tile_y in 1..sqrt_tile_count {
            let fixed_tile = tile_map[tile_x][tile_y - 1];
            let (adjacent_tile, adjacent_orientation) =
                find_adjacent_tile_and_orientation_for_edge(fixed_tile, &fixed_edge, &edges);

            tile_map[tile_x][tile_y] = adjacent_tile;
            tile_tile_relative_orientations.insert(
                (fixed_tile, adjacent_tile),
                (fixed_edge_orientation, adjacent_orientation),
            );

            fixed_edge_orientation = get_opposite_edge(adjacent_orientation);
            fixed_edge = &edges_by_tile_id[&adjacent_tile][fixed_edge_orientation];
        }
    }

    // at this point, tile_map is completely filled in,
    // e.g. [
    //   [1171, 2473, 3079],
    //   [1489, 1427, 2311],
    //   [2971, 2729, 1951]
    // ]
    println!("{:?}", tile_map);

    let mut tile_bottom_edge;
    let mut tile_bottom_edge_orientation;
    let mut column_top_tile_bottom_edge = first_corner_bottom_edge;
    let mut column_top_tile_bottom_edge_orientation = first_corner_bottom_edge_orientation;
    let mut column_top_tile_offset_and_rotation_params = (0, 0, 0, 0, 0, 0);
    for tile_y in 0..sqrt_tile_count {
        tile_bottom_edge = column_top_tile_bottom_edge;
        tile_bottom_edge_orientation = column_top_tile_bottom_edge_orientation;

        for tile_x in 0..sqrt_tile_count {
            let tile_id = tile_map[tile_x][tile_y];

            let offset_and_rotation_params = get_offset_and_rotation_matrix_from_edge_orientation(
                tile_useful_size,
                tile_bottom_edge_orientation,
            );
            if tile_x == 0 {
                column_top_tile_offset_and_rotation_params = offset_and_rotation_params;
            }

            let (
                to_final_offset_x,
                to_final_offset_y,
                to_final_dxdx,
                to_final_dxdy,
                to_final_dydx,
                to_final_dydy,
            ) = offset_and_rotation_params;

            write_tile_to_final_image(
                &mut full_image,
                tiles_by_id[&tile_id],
                tile_x,
                tile_y,
                to_final_offset_x,
                to_final_offset_y,
                to_final_dxdx,
                to_final_dxdy,
                to_final_dydx,
                to_final_dydy,
            );

            println!();
            for v in &full_image {
                println!("  {:?}", v);
            }

            if tile_x + 1 < sqrt_tile_count {
                let (adjacent_tile, adjacent_edge_orientation) =
                    find_adjacent_tile_and_orientation_for_edge(tile_id, tile_bottom_edge, &edges);
                assert!(adjacent_tile == tile_map[tile_x + 1][tile_y]);
                tile_bottom_edge_orientation = get_opposite_edge(adjacent_edge_orientation);
                tile_bottom_edge = &edges_by_tile_id[&adjacent_tile][tile_bottom_edge_orientation];
            }
        }

        if tile_y + 1 < sqrt_tile_count {
            let previous_column_top_tile_id = tile_map[0][tile_y];
            let (off_x, off_y, dxdx, dxdy, dydx, dydy) = column_top_tile_offset_and_rotation_params;
            let previous_column_top_tile_right_edge = &get_final_tile_edge(
                tiles_by_id[&previous_column_top_tile_id],
                None,
                Some(tile_useful_size as i64),
                off_x,
                off_y,
                dxdx,
                dxdy,
                dydx,
                dydy,
            );

            // let previous_column_right_edge_orientation =
            //     get_post_transform_right_edge(column_top_tile_bottom_edge_orientation);
            // let previous_column_right_edge = &edges_by_tile_id[&previous_column_top_tile_id]
            //     [previous_column_right_edge_orientation];

            let (adjacent_tile, adjacent_edge_orientation) =
                find_adjacent_tile_and_orientation_for_edge(
                    previous_column_top_tile_id,
                    previous_column_top_tile_right_edge,
                    &edges,
                );
            assert!(adjacent_tile == tile_map[0][tile_y + 1]);

            for possible_bottom_edge_orientation in 0..8usize {
                let (pos_off_x, pos_off_y, pos_dxdx, pos_dxdy, pos_dydx, pos_dydy) =
                    get_offset_and_rotation_matrix_from_edge_orientation(
                        tile_useful_size,
                        possible_bottom_edge_orientation,
                    );

                let tile_left_edge = get_final_tile_edge(
                    tiles_by_id[&adjacent_tile],
                    None,
                    Some(-1),
                    pos_off_x,
                    pos_off_y,
                    pos_dxdx,
                    pos_dxdy,
                    pos_dydx,
                    pos_dydy,
                );

                if &tile_left_edge == previous_column_top_tile_right_edge {
                    column_top_tile_bottom_edge_orientation = possible_bottom_edge_orientation;
                    break;
                }
            }

            // column_top_tile_bottom_edge_orientation =
            //     get_right_neighbor_edge(adjacent_edge_orientation);
            column_top_tile_bottom_edge =
                &edges_by_tile_id[&adjacent_tile][column_top_tile_bottom_edge_orientation];

            let (pos_off_x, pos_off_y, pos_dxdx, pos_dxdy, pos_dydx, pos_dydy) =
                get_offset_and_rotation_matrix_from_edge_orientation(
                    tile_useful_size,
                    column_top_tile_bottom_edge_orientation,
                );
            let tile_bottom_edge = get_final_tile_edge(
                tiles_by_id[&adjacent_tile],
                Some(tile_useful_size as i64),
                None,
                pos_off_x,
                pos_off_y,
                pos_dxdx,
                pos_dxdy,
                pos_dydx,
                pos_dydy,
            );
            assert!(column_top_tile_bottom_edge == &tile_bottom_edge);
        }
    }

    println!();
    for v in &full_image {
        println!("  {:?}", v);
    }

    todo!()
}
