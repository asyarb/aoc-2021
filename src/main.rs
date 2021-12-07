#![feature(drain_filter, array_chunks)]

pub mod four;
pub mod one;
pub mod three;
pub mod two;

fn main() {
    four::part_one();
}
