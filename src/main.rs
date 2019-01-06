use std::io;
use std::fmt;


#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>
}


impl Grid {
    pub fn new() -> Grid {
        let (w, h) = (16, 16);
        let new_cells = (0..w*h)        // Q: Stále pořádně nevím, co přesně tohle je. Jenom range?
            .map(|i|
                if (i % 3 == 0) { Cell::Alive } else { Cell::Dead }
                )
            .collect();     // Q: vs. &self.cells?
        

        return Grid {
            width: w,       // Q: vs. just width, defined by let in the new() scope? Rust shortcut?
            height: h,      // A: Yep. More in "Destructuring Structs" manual section.
            cells: new_cells
        }
    }

    pub fn tick(&mut self) {
        //let mut new_cells = self.cells.clone();

        let mut nm = self.compute_neighbor_matrix_0();
        self.tick_neighbor_matrix(nm);
    }

    pub fn get_index(&self, (x, y): (u32, u32)) -> u32 {
      // Optim: Could the wrapping be done here? But. How many times called, eh. N(?)
      // Optim: This function itself should be done with.
        
        let index = y * self.width + x;
        return index;
    }

    pub fn get_x_y(&self, index: u32) -> (u32, u32) {
        let x = index % self.width;
        let y = (index - x) / self.width;
        
        (x, y)
    }

    pub fn count_alive_neighbors_0(&self, (x, y): (u32, u32)) -> u8 {
       8
    }

    pub fn count_alive_neighbors_from_index(&self, index: u32) -> u8 {
        
        print!("{:?} ", self.get_x_y(index));
        
        8
    }

    pub fn compute_neighbor_matrix_0 (&self) -> Vec<u8> {
        // let mut neighbor_matrix: Vec<u8> = Vec::with_capacity(self.width as usize * self.height as usize);
        let mut neighbor_matrix = vec![0; self.width as usize * self.height as usize];

        // TODO: Ignoring the borders for now.
        // Learning: THIS should be the first implementation to get done.
        /// @ Q: Complexity: quadratic?
        for i in 1..=self.height-2 {
            for j in 1..=self.width-2 {
                let mut count = 0;
                for n in 0..=2 {
                    for m in 0..=2 {
                        if m == 0 && n == 0
                            { continue; } // We don't count current cell as its own neighbor.
                        count += self.cells[self.get_index((i+n-1, j+m-1)) as usize] as u8;
                    }
                }
                // println!("c:{:?} ", count);
                neighbor_matrix[self.get_index((i, j)) as usize] = count;
            }
        }

        for line in neighbor_matrix.as_slice().chunks(self.width as usize)
            { println!("{:?}", line); }
        
        return neighbor_matrix as Vec<u8>;
    }

    /// Changes internal state (cells), doesn't return anything.
    /// Q: Is that good? Side effects?
    fn tick_neighbor_matrix(&mut self, alive_neighbor_matrix: Vec<u8>) {

        let a: Vec<Cell> = self.cells.iter_mut().enumerate()
            .map(|(i, cell)| { (match (*cell, alive_neighbor_matrix[i as usize]) {       // TODO: Z nějakýho důvodu se neprovádí. A: Aha, musím vrátit *cell.
                    (Cell::Alive, 0..=1) => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Alive, 4..=8) => Cell::Dead,
                    (otherwise, _) => otherwise
            })           
            }).collect();
        // Map is zazy, doesn't do anything until consumed. 

        print!("a{:?}", a);

        /*
        // Optim: Why bother with taking the current state into account instead of just working with the number of alive neighbors to construct the return value?
        for (i, cell) in self.cells.iter_mut().enumerate() {
            *cell = match (*cell, alive_neighbor_matrix[i as usize]) {
                    (Cell::Alive, 0..=1) => Cell::Dead,                    
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Alive, 4..=8) => Cell::Dead,
                    (otherwise, _) => otherwise

                /*
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,             // Q: Optim: Isn't this redundant?
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,                        // Q: Optim: Is there a benefit of using if instead of range?
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,    */
            }
        };        */
    }

    pub fn count_alive_neighbors_1(&self, (x, y): (u32, u32)) -> u8 {

        macro_rules! wrp { ( ($xe:expr, $ye:expr) ) => {  (self.wrap_x($xe), self.wrap_y($ye)) } };
    
        // Optim: Why should be run so many times? Once is enough, no?
               
        // let mut neighbors = Vec::new(); //Vec<u8>::with_capacity(9);
        let mut count = 0;

        //for i in 0..=2 {
            //let ix = i - 1;
            // let a = neighbors.append(
            //count += self.cells[self.get_index(wrp!((x-1,y+i))) as usize] as u8;
            //, self.get_index(wrp!((x,y+ix))), self.get_index(wrp!((x+1,y+ix))));
          //  print!("Count: {}", count);
        //}
        // print!("Count: {}", count);

        // Optim: Collection of neighbors mapped onto dead/alive and summed?
        // Optim: Then do 2 or even more in 1 pass. Cause they overlap, redundant calculations then.

        //for i in self.cells.as_slice().chunks() {
        //    (self.cells[get_index(x-1, y+i)], self.cells[get_index(x, y+i)], self.cells[get_index(x+1, y+i)]).sum

            //
            // .map(|| { self.cells[] == Cell::Alive}
            // .sum()
        //}

        // let symbol = (index_x, index_y);

        20
    }

    pub fn count_alive_neighbors_2((x, y): (u32, u32)) -> u8 {
        8
    }
    
    fn dies((x, y): (u32, u32)) -> u8 {
        0
    }

    // Q: Could be optimized? As a closure perhaps? How many times really needs to be run?
    fn wrap_x(&self, x: u32) -> u32 {
        Grid::wrap(x, self.width - 1)
    }

    fn wrap_y(&self, y: u32) -> u32 {
        Grid::wrap(y, self.height - 1)
    }
    
    fn wrap(i: u32, last_index: u32) -> u32 {
        ((i % (last_index)) + (last_index) * (i == 0) as u32) // Q: Optimize "j == 0" with some bitwise operator?
    }


}

impl fmt::Display for Grid {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut j: u32 = 0;

        // Optim: This is the same iterative function as used elsewhere. Could generalize, make a closure?
        for (i, line) in self.cells.as_slice().chunks(self.width as usize).enumerate() {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };       // Q: i as u8 as char - isn't this dangerous? What if i big? Test.
                write!(f, "{:} ", symbol); 
                j += 1;
            }
            j = 0;
            write!(f, "\n")?;
        }
        Ok(())
    }
}


fn main() {

    println!("Hello, Rust!");

    let mut guess = String::new();

    // io::stdin().read_line(&mut guess)
    //  .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let collected_iterator: Vec<i32> = vec![1, 2, 9];
    println!("Collected (0..10) into: {:?}", collected_iterator);
    println!("Collected (0..10) into: {:#?}", (0..6));

    let mut grid = Grid::new();

    // let neighbor = grid.compute_neighbor_matrix_0();
    
    for i in 1..4 {
        grid.tick();
        print!("{:}", grid);
    }

    


}

fn fill_grid(grid: Grid) -> Grid {

    
    return Grid::new();
}



/*
fn fill_vec(vec: Vec<Cell>) -> Vec<Cell> {

    let filled_vec = Vec<Cell>;

    return filled_vec;
}
*/