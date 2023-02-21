use std::collections::BTreeSet;

use crate::board::find_neighbors;

pub type Patterns = BTreeSet<Pattern>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Tile {
    Tile,
    Frontier,
    Empty
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Pattern {
    pub tiles: BTreeSet<usize>,
    pub frontiers: BTreeSet<usize>,
    pub board: Vec<Tile>,
}

pub fn explore_frontiers(pattern: Pattern) -> Patterns {
    let mut patterns: Patterns = Patterns::new();
    for frontier in pattern.frontiers.iter() {
        // make a copy of the original pattern
        let mut newpattern = pattern.clone();

        // add one frontier to the copy's tiles
        newpattern.tiles.insert(*frontier);

        // add the new tile to the board as well
        newpattern.board[*frontier] = Tile::Tile;

        // remove that frontier from the copy
        newpattern.frontiers.remove(frontier);

        // find the neighbors of the new tile, add them to the frontier
        newpattern.frontiers.extend(find_neighbors(&pattern.board, *frontier));

        // add this new pattern to the collection
        patterns.insert(newpattern);
    }

    patterns
}