use std::collections::HashSet;

fn main() {
    let size = 2;
    let startpos = 12;

    let mut board: Vec<Tile> = vec![Tile::Empty;25];
    board[12] = Tile::Tile;
    print_board(&board)
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

fn find_neighbors(board: &Vec<bool>, node: usize) -> HashSet<usize> {
    let rowsize = f64::sqrt(board.len() as f64) as usize;
    let mut neighbors = HashSet::<usize>::new();
    // north
    if node - rowsize > 0 && board[node] == false {
        neighbors.insert(node - rowsize);
    }
    // south
    if node + rowsize < board.len() && board[node] == false {
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

    return neighbors
}

fn gtp_puddle(board: Vec<usize>, patterns: HashSet<HashSet<usize>>, frontier: HashSet<usize>) -> HashSet<HashSet<usize>> {

    // take the current set
    // figure out the frontiers
    return HashSet::new();
}