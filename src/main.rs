use std::io;
use std::fmt;
use rand::Rng;

mod grid;
mod playground;

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

    let rnn = rand::thread_rng().gen::<u8>() % 9;

    println!("Random number: {}", rnn);

    let mut tff: u32ff = u32ff { size: 16, val: 32 };

    tff = tff + 24;

    let a: u32 = tff.into();

    println!("Test u32ff: {:?}", tff);
    println!("Test u32ff a: {:?}", a);

    for i in 0..=4 {
        uni.tick();
        // print!("{}", uni.render());
    }
}