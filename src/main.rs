use std::collections::BTreeSet;

fn main() {
    let size = 4;
    let startpos = 12;
    println!("Tile Positions
--------------
0  1  2  3  4
5  6  7  8  9
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24
--------------");
    let mut explore_board: Vec<Tile> = vec![Tile::Empty;25];
    explore_board[12] = Tile::Tile;
    let mut pattern: Pattern = Pattern {
        tiles: BTreeSet::new(),
        frontiers: BTreeSet::new(),
        board: vec![Tile::Empty; 25],
    };
    pattern.frontiers.insert(12);
    let mut patterns: Patterns = Patterns::new();
    patterns.insert(pattern.clone());

    // for each size, until the desired size
    for _ in 0..size {
        let mut sizepatterns: Patterns = Patterns::new();

        // loop through each existing pattern of size n
        for pattern in patterns.clone().into_iter() {

            // generate all possible patterns of size n + 1
            sizepatterns.extend(explore_frontiers(pattern.clone()));
        }

        // overwrite previous patterns list to only contain the largest size (n + 1)
        patterns = sizepatterns;
    }

    for pattern in patterns.iter() {
        let display_board = board_from_pattern(&pattern, 25);
        print_board(&display_board);
    }

}

type Patterns = BTreeSet<Pattern>;
// type Pattern = BTreeSet<usize>;
// type Frontier = BTreeSet<usize>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Tile {
    Tile,
    Frontier,
    Empty
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Pattern {
    tiles: BTreeSet<usize>,
    frontiers: BTreeSet<usize>,
    board: Vec<Tile>,
}

fn print_board(board: &Vec<Tile>) {
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

fn find_neighbors(board: &Vec<Tile>, node: usize) -> BTreeSet<usize> {
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

fn explore_frontiers(pattern: Pattern) -> Patterns {
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

fn board_from_pattern(pattern: &Pattern, boardsize: usize) -> Vec<Tile> {
    let mut board = vec![Tile::Empty; boardsize];

    for tile in pattern.tiles.iter() {
        board[*tile] = Tile::Tile;
    }
    for frontier in pattern.frontiers.iter() {
        board[*frontier] = Tile::Frontier;
    }

    board
}