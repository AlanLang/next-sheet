use super::cell::Cell;

#[derive(Debug)]
pub struct Sheet {
    pub cell: Vec<Cell>,
}

impl Sheet {
    pub fn new() -> Sheet {
        Sheet {
            cell: Vec::new(),
        }
    }

    pub fn get_cell(&self) -> &Cell {
        &self.cell[0]
    }
}