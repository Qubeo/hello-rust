use std::io;
use std::fmt;

mod grid;
use grid::{*};

/*
#[cfg(feature = "flame_it")]
extern crate flame;
#[cfg(feature = "flame_it")]
#[macro_use] extern crate flamer;
// as well as the following instead of `#[flame]`
#[cfg_attr(feature = "flame_it", flame)] */


fn main() {

    println!("Hello, Rust!");
    let mut uni = Grid::new();

    for i in 0..=4 {
        uni.tick();
        // print!("{}", uni.render());
    }
}