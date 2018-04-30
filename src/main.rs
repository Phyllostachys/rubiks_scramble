extern crate rand;
use rand::Rng;

const MOVEMENTS: [&str; 18] = [
    "F", "B", "L", "R", "U", "D",
    "f", "b", "l", "r", "u", "d",
    "Ff", "Bb", "Ll", "Rr", "Uu", "Dd"
];
const ROTATIONS: [&str; 6] = [" ", " ", " ", "2 ", "' ", "2' "];

fn main() {
    let mut rubiks_type = std::env::args()
        .nth(1)
        .unwrap_or(std::string::String::from("3"))
        .parse::<i32>()
        .unwrap_or(3);
    if rubiks_type < 3 {
        rubiks_type = 3;
    }
    println!("Rubik's Cube type is {0}x{0}", rubiks_type);

    let range = if rubiks_type == 3 {
        5
    } else if rubiks_type < 6 {
        17
    } else {
        MOVEMENTS.len() - 1
    };
    let mut rng = rand::thread_rng();
    let num_moves = 5 * ((rubiks_type + 2).pow(2) / 5);
    let mut moves: Vec<std::string::String> = Vec::new();
    for _ in 0..num_moves {
        let mut a_move = std::string::String::new();
        a_move.push_str(MOVEMENTS[rng.gen_range(0, range)]);
        a_move.push_str(*rng.choose(&ROTATIONS).unwrap_or(&" "));
        moves.push(a_move.trim().to_string());
    }

    for i in 0..moves.len() {
        print!("{:<5} ", moves[i]);
        if ((i + 1) % 5) == 0 {
            println!("");
        }
    }
}
