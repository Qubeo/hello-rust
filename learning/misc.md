**Snippets**

Original rules implementation:
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
                    (otherwise, _) => otherwise

**Learnings**


**Resources**

Rust iterator speed considerations: https://medium.com/@veedrac/rust-is-slow-and-i-am-the-cure-32facc0fdcb