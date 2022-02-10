use crate::newt::sigmoid;

mod newt;

fn main() {
    println!("-6: {} 6: {} 0: {} 1: {}", sigmoid(-6.0), sigmoid(6.0), sigmoid(0.0), sigmoid(1.0));
}
