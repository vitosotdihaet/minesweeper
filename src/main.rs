mod minesweeper;
use minesweeper::Minesweeper;

fn main() {
    let mut ms = Minesweeper::default().input_starting_info();

    loop {
        ms.print();
        ms.run();
        if !ms.playing {
            break;
        }
    }
}
