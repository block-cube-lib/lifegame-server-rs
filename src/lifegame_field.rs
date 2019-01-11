use crate::lifegame::Cell;

#[derive(Clone)]
pub struct LifegameField {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    next_cells: Vec<Cell>,
}

impl LifegameField {
    pub fn new(width: u32, height: u32) -> LifegameField {
        let mut field = LifegameField {
            width,
            height,
            cells: vec![],
            next_cells: vec![],
        };
        field.reset();
        field
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> Vec<Cell> {
        self.cells.clone()
    }

    pub fn reset(&mut self) {
        let size = (self.width * self.height) as usize;
        self.cells = vec![0; size]
            .into_iter()
            .map(|_| rand::random())
            .map(|b| if b { Cell::Alive } else { Cell::Dead })
            .collect();
        self.next_cells = self.cells.clone();
    }

    pub fn update_cells(&mut self) {
        for i in 0..self.cells.len() {
            let x = (i % self.width as usize) as i32;
            let y = (i / self.width as usize) as i32;
            self.next_cells[i] = self.get_next_cell_state(x, y);
        }
        self.cells = self.next_cells.to_vec();
    }

    fn get_cell_state(&self, x: i32, y: i32) -> Cell {
        let mut x = x;
        let mut y = y;
        let width = self.width as i32;
        let height = self.height as i32;
        x = if x < 0 {
            x + width
        } else if width <= x {
            x % width
        } else {
            x
        };
        y = if y < 0 {
            y + height
        } else if height <= y {
            y % height
        } else {
            y
        };

        let index = width * y + x;
        self.cells[index as usize]
    }

    fn get_next_cell_state(&self, x: i32, y: i32) -> Cell {
        let around_cell_indices = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        let around_alive_count = around_cell_indices
            .into_iter()
            .map(|(ix, iy)| self.get_cell_state(*ix, *iy))
            .filter(move |cell_state| *cell_state == Cell::Alive)
            .count();

        match around_alive_count {
            0..=1 => Cell::Dead,
            2 => self.get_cell_state(x, y),
            3 => Cell::Alive,
            _ => Cell::Dead,
        }
    }
}
