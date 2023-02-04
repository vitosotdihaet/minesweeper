use rand::seq::SliceRandom;
use std::{
    cmp::{min, max},
    fmt
};

const MINE_COUNT_DISPLAY: &[char] = &['0','1','2','3','4','5','6','7','8'];

#[derive(Clone, Copy, Default)]
pub struct Cell {
    pub surrounds: u8,
    pub mine: bool,
    pub flag: bool,
    pub revealed: bool,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = '?';
        if self.flag {
            result = '!';
        } else if self.revealed && self.mine {
            result = 'M';
        } else if self.revealed {
            result = MINE_COUNT_DISPLAY[self.surrounds as usize];
        }
        write!(f, "{}", result)
    }
}

pub struct Minesweeper {
    pub playing: bool,
    pub won: bool,
    pub grid: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
    first_move: bool,
    number_of_mines: usize,
    number_of_revealed_cells: usize,
    number_of_flagged_mines: usize,
}

impl Default for Minesweeper {
    fn default() -> Self {
        Minesweeper::new(10, 10, 10)
    }
}

impl fmt::Display for Minesweeper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        let w_length = self.width.to_string().len();
        let h_length = self.height.to_string().len();

        let mut to_print = "#".to_owned() + &" ".repeat(max(0, h_length as isize - w_length as isize + 1) as usize);
        for n in 0..self.width { 
            to_print += &(" ".repeat(w_length - (n + 1).to_string().len() + 1) + &(n + 1).to_string());
        }
        result.push_str(&to_print);
        result.push('\n');
        result.push_str(&" ".repeat(h_length + 1));
        result.push_str(&"_".repeat(to_print.len() - h_length - 1));
        result.push('\n');

        for y in 0..self.height {
            result.push_str(&((y + 1).to_string() + &" ".repeat(h_length - (y + 1).to_string().len() + 1) + "|"));
            for x in 0..self.width {
                result.push_str(&format!("{}", self.grid[y][x]));
                result.push_str(&" ".repeat(w_length));
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, number_of_mines: usize) -> Self {
        let grid = vec![vec![Cell::default(); width]; height];
        Minesweeper {
            playing: true,
            won: true,
            grid,
            width,
            height,
            first_move: true,
            number_of_mines,
            number_of_revealed_cells: 0,
            number_of_flagged_mines: 0,
        }
    }

    pub fn open(&mut self, x: usize, y: usize) {
        // generate a grid "after" the first move to prevent from failing
        if self.first_move {
            self.generate_grid(x, y);
            self.first_move = false;
        }

        if !self.grid[y][x].flag {
            match self.grid[y][x].mine {
                true => {
                    self.grid[y][x].revealed = true;
                    self.playing = false;
                    self.won = false;
                    return;
                }
                false => {
                    self.open_empty(x, y);
                }
            }
           self.check_for_win();        
        }
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        if self.grid[y][x].revealed {
            return;
        }

        self.grid[y][x].flag = !self.grid[y][x].flag;

        if self.grid[y][x].flag && self.grid[y][x].mine {
            self.number_of_flagged_mines += 1;
        } else if !self.grid[y][x].flag && self.grid[y][x].mine {
            self.number_of_flagged_mines -= 1;
        }

        self.check_for_win();        
    }

    fn open_empty(&mut self, x: usize, y: usize) {
        if self.grid[y][x].revealed {
            return;
        }

        if self.grid[y][x].surrounds != 0 {
            self.number_of_revealed_cells += 1;
            self.grid[y][x].revealed = true;
            return;
        }

        if !self.grid[y][x].revealed {
            self.number_of_revealed_cells += 1;
        }
        self.grid[y][x].revealed = true;

        let xrange = max(0, x as isize - 1) as usize..=min(self.width - 1, x + 1);
        let yrange = max(0, y as isize - 1) as usize..=min(self.height - 1, y + 1);

        for cy in yrange {
            for cx in xrange.clone() {
                self.open_empty(cx, cy);
            }
        }
    }

    fn generate_grid(&mut self, x: usize, y: usize) {
        if self.number_of_mines == self.width * self.height {
            for y in 0..self.height  {
                for x in 0..self.width {
                    self.grid[y][x].mine = true;
                }
            }
            return;
        }

        let mut rng = rand::thread_rng();
        let mut mines: Vec<(usize, usize)> = vec![];
        for cy in 0..self.height {
            for cx in 0..self.width {
                if !(cy == y && cx == x) {
                    mines.push((cx, cy));
                }
            }
        }

        for (cx, cy) in mines.choose_multiple(&mut rng, self.number_of_mines) {
            let (x, y) = (*cx, *cy);
            self.grid[y][x].mine = true;

            let xrange = max(0, x as isize - 1) as usize..=min(self.width - 1, x + 1);
            let yrange = max(0, y as isize - 1) as usize..=min(self.height - 1, y + 1);
            
            for cy in yrange {
                for cx in xrange.clone() {
                    if !(cy == y && cx == x) {
                        self.grid[cy][cx].surrounds += 1;
                    }
                }
            }
        }
    }

    fn check_for_win(&mut self) {
        if self.width * self.height - self.number_of_revealed_cells == self.number_of_flagged_mines {
            self.playing = false;
        }
    }
}