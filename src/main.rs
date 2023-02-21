use std::collections::BTreeSet;

fn main() {
    let size = 2;
    let startpos = 12;

    let mut explore_board: Vec<Tile> = vec![Tile::Empty;25];
    explore_board[12] = Tile::Tile;
    let mut frontiers = find_neighbors(&explore_board, 12);
    let mut pattern: BTreeSet<usize> = BTreeSet::new();
    pattern.insert(12);
    let mut patterns = BTreeSet::new();
    patterns.insert(pattern);
    for _ in 0..size-1 {
        patterns.extend(explore_frontiers(&mut explore_board, &pattern, &mut frontiers));
    }
    println!("{:?}", patterns);
    let mut display_board: Vec<Tile> = vec![Tile::Empty;25];
    for pattern in patterns.iter() {
        for idx in pattern.iter() {
            display_board[*idx] = Tile::Tile;
        }
    }
    for frontier in frontiers.iter() {
        display_board[*frontier] = Tile::Frontier;
    }
    print_board(&explore_board);
}
#[derive(Clone, Copy)]
enum Tile {
    Tile,
    Frontier,
    Empty
}

fn print_board(board: &Vec<Tile>) {
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
}

fn find_neighbors(board: &Vec<Tile>, node: usize) -> BTreeSet<usize> {
    let rowsize = f64::sqrt(board.len() as f64) as usize;
    let mut neighbors = BTreeSet::<usize>::new();
    if let Tile::Empty = board[node] {
        return neighbors;
    }
    // north
    if (node as i64 - rowsize as i64) > 0{
        neighbors.insert(node - rowsize);
    }
    // south
    if node + rowsize < board.len() {
        neighbors.insert(node + rowsize);
    }
    // east
    if (node + 1) < board.len() && (node + 1) % rowsize != 0 {
        neighbors.insert(node + 1);
    } 
    // west
    if (node as i64 - 1) >= 0 && (node - 1) % rowsize != board.len() - 1 {
        neighbors.insert(node - 1);
    }

    neighbors
}

fn explore_frontiers(board: &mut Vec<Tile>, pattern: &BTreeSet<usize>, frontiers: &mut BTreeSet<usize>) -> BTreeSet<BTreeSet<usize>> {
    let mut patterns: BTreeSet<BTreeSet<usize>> = BTreeSet::new();
    for frontier in frontiers.clone().iter() {
        let mut newpattern = pattern.clone();
        newpattern.insert(*frontier);
        patterns.insert(newpattern);
        frontiers.remove(frontier);
        board[*frontier] = Tile::Tile;
        frontiers.extend(find_neighbors(board, *frontier));
    }
    
    patterns
}