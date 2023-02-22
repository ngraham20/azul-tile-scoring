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

fn north_neighbor(board: &Vec<Tile>, rowsize: usize, node: usize) -> Option<usize> {
    if (node as i64 - rowsize as i64) > 0 {
        Some(node - rowsize)
    } else { None }
}

fn south_neighbor(board: &Vec<Tile>, rowsize: usize, node: usize) -> Option<usize> {
    if (node + rowsize) < board.len() {
        Some(node + rowsize)
    } else { None }
}

fn east_neighbor(board: &Vec<Tile>, rowsize: usize, node: usize) -> Option<usize> {
    let neighbor = node + 1;
    if neighbor < board.len() && neighbor % rowsize != 0 {
        Some(neighbor)
    } else { None }
}

fn west_neighbor(board: &Vec<Tile>, rowsize: usize, node: usize) -> Option<usize> {
    let neighbor = node - 1;
    if (node as i64 - 1) >= 0 && neighbor % rowsize != rowsize - 1 {
        Some(neighbor)
    } else { None }
}

pub fn find_neighbors(board: &Vec<Tile>, node: usize) -> BTreeSet<usize> {
    let rowsize = f64::sqrt(board.len() as f64) as usize;
    let mut neighbors = BTreeSet::<usize>::new();

    // north
    if let Some(neighbor) = north_neighbor(board, rowsize, node) {
        if let Tile::Empty = board[neighbor] {
            neighbors.insert(neighbor);
        }
    }
    // south
    if let Some(neighbor) = south_neighbor(board, rowsize, node) {
        if let Tile::Empty = board[neighbor] {
            neighbors.insert(neighbor);
        }
    }
    // east
    if let Some(neighbor) = east_neighbor(board, rowsize, node) {
        if let Tile::Empty = board[neighbor] {
            neighbors.insert(neighbor);
        }
    } 
    // west
    if let Some(neighbor) = west_neighbor(board, rowsize, node) {
        if let Tile::Empty = board[neighbor] {
            neighbors.insert(neighbor);
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

fn north_points(board: &Vec<Tile>, rowsize: usize, tile: usize) -> usize {
    if let Some(neighbor) = north_neighbor(board, rowsize,  tile) {
        match board[neighbor] {
            Tile::Tile => 1 + north_points(board, rowsize, neighbor),
            _ => 0
        }
    } else { 0 }
}

fn south_points(board: &Vec<Tile>, rowsize: usize, tile: usize) -> usize {
    if let Some(neighbor) = south_neighbor(board, rowsize, tile) {
        match board[neighbor] {
            Tile::Tile => 1 + south_points(board, rowsize, neighbor),
            _ => 0
        }
    } else { 0 } 
}

fn east_points(board: &Vec<Tile>, rowsize: usize, tile: usize) -> usize {
    if let Some(neighbor) = east_neighbor(board, rowsize, tile) {
        match board[neighbor] {
            Tile::Tile => 1 + east_points(board, rowsize, neighbor),
            _ => 0
        }
    } else { 0 }
}

fn west_points(board: &Vec<Tile>, rowsize: usize, tile: usize) -> usize {
    if let Some(neighbor) = west_neighbor(board, rowsize, tile) {
        match board[neighbor] {
            Tile::Tile => 1 + west_points(board, rowsize, neighbor),
            _ => 0
        }
    } else { 0 }
}

pub fn score_placement(board: &Vec<Tile>, tile: usize) -> Option<usize> {
    let mut points = 0usize;
    let rowsize = f64::sqrt(board.len() as f64) as usize;

    // row points
    let rowpoints = {
        east_points(board, rowsize, tile) +
        west_points(board, rowsize, tile)
    };
    let colpoints = {
        north_points(board, rowsize, tile) +
        south_points(board, rowsize, tile)
    };

    if rowpoints > 0 && colpoints > 0 {
        points += 1
    }
    points += rowpoints + colpoints;

    Some(points)
}