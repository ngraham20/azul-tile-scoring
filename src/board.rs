use std::collections::BTreeSet;

use crate::pattern::{Tile, Pattern};

pub fn print_board(board: &Vec<Tile>) {
    println!("---------------");
    let rowsize = f64::sqrt(board.len() as f64) as usize;
    for (idx, item) in board.iter().enumerate() {
        print!("{}", match item {
            Tile::Tile      => " O ",
            Tile::Frontier  => " + ",
            Tile::Empty     => " . "
        });
        if idx % rowsize == rowsize-1 {
            println!();
        }
    }
    println!("---------------");
}

pub fn find_neighbors(board: &Vec<Tile>, node: usize) -> BTreeSet<usize> {
    let rowsize = f64::sqrt(board.len() as f64) as usize;
    let mut neighbors = BTreeSet::<usize>::new();

    // north
    if (node as i64 - rowsize as i64) > 0 {
        if let Tile::Empty = board[node - rowsize] {
            neighbors.insert(node - rowsize);
        }
    }
    // south
    if node + rowsize < board.len() {
        if let Tile::Empty = board[node + rowsize] {
            neighbors.insert(node + rowsize);
        }
    }
    // east
    if (node + 1) < board.len() && (node + 1) % rowsize != 0 {
        if let Tile::Empty = board[node + 1] {
            neighbors.insert(node + 1);
        }
    } 
    // west
    if (node as i64 - 1) >= 0 && (node - 1) % rowsize != rowsize - 1 {
        if let Tile::Empty = board[node - 1] {
            neighbors.insert(node - 1);
        }
    }

    neighbors
}

pub fn board_from_pattern(pattern: &Pattern, boardsize: usize) -> Vec<Tile> {
    let mut board = vec![Tile::Empty; boardsize];

    for tile in pattern.tiles.iter() {
        board[*tile] = Tile::Tile;
    }
    for frontier in pattern.frontiers.iter() {
        board[*frontier] = Tile::Frontier;
    }

    board
}