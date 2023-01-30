use std::fmt::Display;



#[derive(Copy, Clone)]
struct Cell {
    // Describes the state of the cell's walls, true means wall, false, no wall. 
    // TODO: change this to a bitmask
    pos: (usize, usize),
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl Cell {
    pub fn new(pos: (usize, usize)) -> Self {
        Self {
            pos,
            north: true,
            south: true,
            east: true,
            west: true,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "({}, {})", self.pos.0, self.pos.1)
    }
}

pub struct Maze<const R: usize, const C:usize> {
    cells: [[Cell;C];R],
}


impl<const R: usize, const C:usize> Maze<R,C> {

    pub fn new() -> Self {
        
        let mut cells = [[Cell::new((0,0));C];R];
            
        for row in 0..R {
            for col in 0..C {
                cells[row][col] = Cell::new((row, col));
            }
        }


        Self {
            cells
        }     
    }
}


impl<const R: usize, const C:usize> Maze<R,C> {
    pub fn rows(&self) -> usize {
        R
    }

    pub fn cols(&self) -> usize {
        C
    }
}

impl<const R: usize, const C:usize> Display for Maze<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        writeln!(f, "{} x {} Maze:", self.rows(), self.cols())?;

        for row in 0..self.rows() {
            for col in 0..self.cols() {

                write!(f, "{} ", self.cells[row][col])?;
            }
            write!(f, "\n")?;
        }
        
        
        Ok(())
    }
}


