#![allow(dead_code)]

mod plus_one;
mod rotate_array;
mod remove_duplicates;
mod contains_duplicate;
mod move_zeroes;

fn main() {
    let mut input = vec![0, 1, 0, 3, 12];
    move_zeroes::move_zeroes(&mut input);
    println!("{:?}", input);
}