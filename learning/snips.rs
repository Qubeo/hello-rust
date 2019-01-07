// Optim: Vs. cells.clone() a iter_into()?
// neighbor_matrix = self.cells.iter()
//    .map(|i| { 0 })// i.clone() as u8 })
//    .collect();

// Q: How to create identically sized matrix from a matrix (vector) of a different, but compatible (through the repr(u8)) type?

// Stdin usage
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess)
    //  .expect("Failed to read line");
    // println!("You guessed: {}", guess);
