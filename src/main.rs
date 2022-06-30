use rand::Rng;
use std::io;

const SMILEYS: &[&'static str] = &["0️⃣", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣"];

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Bomb,
    Flagged,
    FlaggedBomb,
}

pub struct Minesweeper {
    playing: bool,
    grid:    Vec<Vec<Cell>>,
    opened:  Vec<(usize, usize)>,
    counts:  Vec<Vec<u8>>,
    width:   usize,
    height:  usize,
}

impl Minesweeper {
    fn new(width: usize, height: usize, number_of_mines: usize) -> Self {
        let playing = true;
        let mut grid = vec![vec![Cell::Empty; width]; height];
        let opened = vec![];
        let mut bombs = vec![];
        let mut counts = vec![vec![0; width]; height];

        let mut rng = rand::thread_rng();
        let mut count = 0;

        while number_of_mines != count {
            let y = rng.gen_range(0..height);
            let x = rng.gen_range(0..width);

            if grid[y][x] != Cell::Bomb {
                grid[y][x] = Cell::Bomb;
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
                            counts[ny as usize][nx as usize] += 1;
                        }
                    }
                }
                // end of cycles
            }
        }

        Minesweeper {playing, grid, opened, counts, width, height}
    }

    fn left_click(&mut self, x: usize, y: usize) {
        match self.grid[y][x] {
            Cell::Bomb => {
                self.opened.push((x, y));
                println!("Epic fail ┻━┻ ︵ヽ(`Д´)ﾉ︵ ┻━┻");
                self.playing = false;
            }
            Cell::Empty => {
                if self.counts[y][x] != 0 {
                    self.opened.push((x, y));
                } else {
                    self.open_empty(x, y)
                }
            }
            _ => {}
        }
    }

    fn right_click(&mut self, x: usize, y: usize) {
        match self.grid[y][x] {
            Cell::Empty       => self.grid[y][x] = Cell::Flagged,
            Cell::Bomb        => self.grid[y][x] = Cell::FlaggedBomb,
            Cell::Flagged     => self.grid[y][x] = Cell::Empty,
            Cell::FlaggedBomb => self.grid[y][x] = Cell::Bomb,
        }
    }

    fn open_empty(&mut self, x: usize, y: usize) {
        if self.opened.contains(&(x, y)) {
            return;
        }
        if self.counts[y][x] != 0 {
            self.opened.push((x, y));
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
                    self.opened.push((x, y));
                    self.open_empty(nx as usize, ny as usize);
                }
            }
        }
    }

    fn run(&mut self) {
        let mut end = true;

        for e in &self.grid {
            if e.contains(&Cell::Bomb) {
                end = false;
                break;
            }
        }

        if end {
            println!("ヾ(≧▽≦*)o");
            self.playing = false;
            return;
        }

        println!("Left or Right click? (L for left, R for right): ");
        let mut input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("Error reading line");
        let click: &str = input_str.trim();

        self.make_a_turn(click);
    }

    fn make_a_turn(&mut self, click: &str) {
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
        if click == "L" {
            self.left_click(x, y)
        } else {
            self.right_click(x, y)
        }
    }

    fn print(&self) {
        print!("#  ");
        for n in 0..self.width { print!("{} ", n + 1) }
        println!();
        
        for y in 0..self.height {
            print!(
                "{}{}|", y + 1,
                " ".repeat( (self.height + 1).to_string().len() - (y + 1).to_string().len() )
            );
            
            for x in 0..self.width {
                if self.grid[y][x] == Cell::Flagged || self.grid[y][x] == Cell::FlaggedBomb {
                    print!("🚩")
                } else if self.opened.contains(&(x, y)) {
                    print!("{} ", SMILEYS[self.counts[y][x] as usize])
                } else {
                    print!("😊")
                }
            }
            println!();
        }
    }
}

fn main() {
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

    let mut ms = Minesweeper::new(width, height, number_of_mines);

    loop {
        ms.print();
        ms.run();
        if !ms.playing {
            break;
        }
    }
}
