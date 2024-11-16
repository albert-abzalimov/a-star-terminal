/*
 *
 * OPEN   // the set of nodes to be evalauted CLOSED // the set of nodes already evaluated
 * add the start node to OPEN
 * loop
 *  current = node in OPEN with the lowest f_cost
 *  remove current from OPEN
 *  add current to CLOSED
 *
 *  if current is the target node    // i.e we have found the path
 *      return
 *  foreach neighbor of the current node
 *      if neighbor is not traversable or neighbor is in CLOSED
 *          skip to the next neighbor
 *
 *      if new_path to neighbor is shorter OR neighbor is not in open
 *          set f_cost of neighbor
 *          set parent of neighbor to current
 *          if neighbor is not in OPEN
 *              add neighbor to OPEN
*/

use a_star_terminal::{get_distance, read_from_file, Coordinate, Tile, TileType};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut grid = read_from_file();
    grid.draw();

    // create 2 lists
    // one holds open, other closed
    let mut open_set: Vec<Tile> = vec![];
    let mut open_heap = BinaryHeap::new();
    let mut closed_set: Vec<Tile> = vec![];

    // to start lets add the starting tile to open
    open_set.push(grid.start_tile);
    open_heap.push(Reverse(grid.start_tile));

    // while we still have stuff to open
    while !open_heap.is_empty() {
        println!("open_heap shouldnt be empty {:?}", open_heap);
        let current_node = open_heap.pop().unwrap().0;
        println!("the current node is {:?}", current_node);
        // let mut current_node = open_set[0];
        // let mut current_node_index = 0;
        // open_set.iter().enumerate().skip(1).for_each(
        //     |(i, opened_tile)| {
        //         if opened_tile.f_cost() < current_node.f_cost()
        //         || opened_tile.f_cost() == current_node.f_cost()
        //         && opened_tile.h_cost < current_node.h_cost {
        //             current_node = *opened_tile;
        //             current_node_index = i;
        //         }
        //     }
        // );
        // we have landed on the target!!
        if current_node.x == grid.end_tile.x && current_node.y == grid.end_tile.y {
            grid.end_tile.parent = Coordinate(current_node.parent.0, current_node.parent.1);
            println!("FOUND END_TILE");
            grid.retrace_path();
            grid.draw();
            panic!("FOUND PATH!")
        }
        println!("current_node {:?}", current_node);
        println!("BEFORE: open_heap ({}): {:#?}", open_heap.len(), open_heap);
        println!("BEFORE: open_set ({}): {:#?}", open_set.len(), open_set);
        println!(
            "BEFORE: closed_set ({}): {:#?}",
            closed_set.len(),
            closed_set
        );
        for (open_index, open_tile) in open_set.iter().enumerate() {
            if open_tile.x == current_node.x && open_tile.y == current_node.y {
                open_set.remove(open_index);
                break;
            }
        }
        closed_set.push(current_node);
        println!("AFTER: open_set ({}): {:#?}", open_set.len(), open_set);
        println!("AFTER: open_heap ({}): {:#?}", open_heap.len(), open_heap);
        println!(
            "AFTER: closed_set ({}): {:#?}",
            closed_set.len(),
            closed_set
        );

        let neigbors = grid.get_neighbours(current_node);
        // println!("neighbors length: {}", neigbors.len());
        #[allow(clippy::needless_range_loop)]
        for i in 0..neigbors.len() {
            let mut neighbor = neigbors[i];
            // if its a wall, or its closed
            if neighbor.class == TileType::Wall || closed_set.contains(&neighbor) {
                println!("wall or closed");
                continue;
            }
            // get the new movement cost to neigbor
            let new_move_cost = current_node.g_cost + get_distance(&current_node, &neighbor);
            // if its better than the neighbors g_cost then set it i.e we found a better path
            // or if the open set does not yet contain neighbor i.e completely new tile
            if new_move_cost < neighbor.g_cost || !open_set.contains(&neighbor) {
                // set the g_cost and h_cost   -> f_cost
                neighbor.g_cost = new_move_cost;
                neighbor.h_cost = get_distance(&neighbor, &grid.end_tile);
                // store the coordinates of the parent
                neighbor.parent = Coordinate(current_node.x, current_node.y);

                grid.tiles[neighbor.x as usize][neighbor.y as usize] = neighbor;

                if !open_set.contains(&neighbor) {
                    open_heap.push(Reverse(neighbor));
                    open_set.push(neighbor);
                }
            }
        }
    }
}
