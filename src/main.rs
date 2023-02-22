use std::collections::{BTreeSet, HashMap};
use itertools::Itertools;
use std::env;

mod pattern;
use pattern::*;

mod board;
use board::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (startpos, patternsize) = parseargs(args);
    let mut pattern: Pattern = Pattern {
        tiles: BTreeSet::new(),
        frontiers: BTreeSet::new(),
        board: vec![Tile::Empty; 25],
    };
    pattern.frontiers.insert(startpos);
    let mut patterns: Patterns = Patterns::new();
    patterns.insert(pattern.clone());

    // for each size, until the desired size
    for _ in 0..patternsize {
        let mut sizepatterns: Patterns = Patterns::new();

        // loop through each existing pattern of size n
        for pattern in patterns.clone().into_iter() {

            // generate all possible patterns of size n + 1
            sizepatterns.extend(explore_frontiers(pattern.clone()));
        }

        // overwrite previous patterns list to only contain the largest size (n + 1)
        patterns = sizepatterns;
    }

    let mut permutations: Vec<Vec<usize>> = Vec::new();
    let mut permscores: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();

    for pattern in patterns.iter() {
        let display_board = board_from_pattern(&pattern, 25);
        print_board(&display_board);

        permutations.extend(pattern.tiles.clone().into_iter().permutations(pattern.tiles.len()).unique());
    }

    for perm in permutations {
        let score = score_permutation(&perm, 25);
        permscores.entry(score).or_insert(vec![]).push(perm);
    }

    for (score, tiles) in permscores.iter() {
        println!("There are {} permutations with a score of {}", tiles.len(), score);
    }

    let highscore = permscores.keys().max().unwrap();
    let bestpermutation = permscores.get(highscore).unwrap().last().unwrap();
    let mut permboard: Vec<Tile> = vec![Tile::Empty; 25];
    println!("The highest score possible with {} tiles is: {}", patternsize, highscore);
    println!("Here is one of those patterns.");
    for tile in bestpermutation {
        permboard[*tile] = Tile::Tile;
        print_board(&permboard);
    }

}

fn parseargs(args: Vec<String>) -> (usize, usize) {
    let startpos: usize;
    let mut patternsize: usize = 1;
    match args.len() {
        // no args passed
        1 => {
            usage();
            std::process::exit(0);},
        2 => {
            match args[1].parse() {
                Ok(pos) => {startpos = pos;},
                Err(err) => {
                    println!("{}", err);
                    usage();
                    std::process::exit(0);
                }
            };
        },
        3 => {
            match (args[1].parse(), args[2].parse()) {
                (Ok(pos), Ok(size)) => {
                    startpos = pos;
                    patternsize = size;
                },
                (Ok(_), Err(err)) => {
                    println!("{}", err);
                    usage();
                    std::process::exit(0);
                },
                (Err(err), Ok(_)) => {
                    println!("{}", err);
                    usage();
                    std::process::exit(0);
                },
                (Err(err1), Err(err2)) => {
                    println!("{}", err1);
                    println!("{}", err2);
                    usage();
                    std::process::exit(0);
                },
            };
        },
        _ => {
            println!("Too many arguments.");
            usage();
            std::process::exit(0);
        }
    }
    (startpos, patternsize)
}

fn usage() {
    println!("USAGE: cargo run -- <startpos> <tilecount>
Tile Positions
--------------
0  1  2  3  4
5  6  7  8  9
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24
--------------");
}