use std::io::{self, BufRead};

use exercise_04::TurtleGraphics;

fn main() {
    let mut turtle_graphics = TurtleGraphics::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        turtle_graphics.parse_command(&line.unwrap());
    }
}
