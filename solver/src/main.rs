use board::*;

fn main() {
    let board = Board9x9::empty();
    println!("{:?}", board.out_of_bound_intersections);
    println!("{:?}", board.empty_intersections);
}
