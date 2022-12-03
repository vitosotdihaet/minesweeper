use rand::Rng;
use std::{io, cmp::max};

const BOMB_COUNT: &[&'static str] = &["0️", "1️", "2️", "3️", "4️", "5️", "6️", "7️", "8️"];

#[derive(Clone, Copy)]
pub struct Cell {
    pub surrounds: u8,
    pub bomb: bool,
    pub flag: bool,
    pub revealed: bool
}

impl Default for Cell {
    fn default() -> Self {
        Cell { 
            surrounds: 0,
            bomb: false,
            flag: false,
            revealed: false }
    }
}

pub struct Minesweeper {
    pub playing: bool,
    pub grid:    Vec<Vec<Cell>>,
    pub width:   usize,
    pub height:  usize,
}

impl Default for Minesweeper {
    fn default() -> Self {
        Minesweeper::new(10, 10, 10)
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, number_of_mines: usize) -> Self {
        let playing = true;
        let mut grid = vec![vec![Cell::default(); width]; height];
        let mut bombs = vec![];

        let mut rng = rand::thread_rng();
        let mut count = 0;

        while number_of_mines != count {
            let y = rng.gen_range(0..height);
            let x = rng.gen_range(0..width);

            if grid[y][x].bomb == false {
                grid[y][x].bomb = true;
                bombs.push((x, y));
                count += 1;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if width as isize > nx && nx >= 0 && height as isize > ny && ny >= 0 {
                            grid[ny as usize][nx as usize].surrounds += 1;
                        }
                    }
                }
            }
        }

        Minesweeper {playing, grid, width, height}
    }

    pub fn open(&mut self, x: usize, y: usize) {
        if !self.grid[y][x].flag && self.playing {
            match self.grid[y][x].bomb {
                true => {
                    self.grid[y][x].revealed = true;
                    self.playing = false;
                }
                false => {
                    if self.grid[y][x].surrounds != 0 {
                        self.grid[y][x].revealed = true;
                    } else {
                        self.open_empty(x, y)
                    }
                }
            }
        }
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        if self.playing {
            self.grid[y][x].flag = !self.grid[y][x].flag;
        }
    }

    fn open_empty(&mut self, x: usize, y: usize) {
        if self.grid[y][x].revealed == true {
            return;
        }
        if self.grid[y][x].surrounds != 0 {
            self.grid[y][x].revealed = true;
            return;
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == dy || dx == -dy {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if self.width as isize > nx && nx >= 0 && self.height as isize > ny && ny >= 0 {
                    self.grid[y][x].revealed = true;
                    self.open_empty(nx as usize, ny as usize);
                }
            }
        }
    }

    pub fn run(&mut self) {
        let mut end = true;

        for line in &self.grid {
            for column in line {
                if column.bomb {
                    end = false;
                    break;
                }
            }
        }

        if end {
            self.playing = false;
            return;
        }

        self.input_a_turn();
    }

    pub fn input_a_turn(&mut self) {
        println!("Left or Right click? (l/r): ");
        let mut input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("Error reading line");
        let click: &str = input_str.trim();

        println!("Input x and y: ");
        let mut input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("Error reading line");

        let mut sub_str = input_str.split_whitespace();
        let mut next_number = || -> usize {
            sub_str
                .next()
                .expect("Not enough input values")
                .parse()
                .expect("Not a number")
        };

        let x = next_number() - 1;
        let y = next_number() - 1;
        if click == "L" || click == "l" {
            self.open(x, y)
        } else {
            self.flag(x, y)
        }
    }

    pub fn input_starting_info(&self) -> Minesweeper {
        // width & height input
        println!("Input width and height of Minesweeper grid: ");
        let mut input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("Error reading line");
    
        let mut sub_str = input_str.split_whitespace();
        let mut next_number = || -> usize {
            sub_str
                .next()
                .expect("Not enough input values")
                .parse()
                .expect("Not a number")
        };
    
        let width = next_number();
        let height = next_number();
    
        // mines input
        println!(
            "Input number of mines (appropriate to this grid is {}): ",
            width * height / 10
        );
        let mut input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("Error reading line");
        let number_of_mines: usize = input_str.trim().parse().expect("Not a number");
    
        Minesweeper::new(width, height, number_of_mines)
    }

    pub fn print(&self) {
        println!();
        let w_length = self.width.to_string().len();
        let h_length = self.height.to_string().len();

        let mut to_print = "#".to_owned() + &" ".repeat(max(0, h_length as isize - w_length as isize + 1) as usize);
        for n in 0..self.width { 
            to_print += &((" ".repeat(w_length - (n + 1).to_string().len() + 1) + &(n + 1).to_string()));
        }
        println!("{}", to_print);
        println!("{}{}", " ".repeat(h_length + 1), "_".repeat(to_print.len() - h_length - 1));
        
        for y in 0..self.height {
            print!(
                "{}{}|", y + 1,
                " ".repeat(h_length - (y + 1).to_string().len() + 1)
            );
            
            for x in 0..self.width {
                if self.grid[y][x].flag {
                    print!("🚩")
                } else if self.grid[y][x].revealed {
                    print!("{}", BOMB_COUNT[self.grid[y][x].surrounds as usize])
                } else {
                    print!("?")
                }
                print!("{}", " ".repeat(w_length))
            }
            println!();
        }

        println!();
    }

}
