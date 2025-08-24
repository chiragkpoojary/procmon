
mod mymodels {
    pub mod process_struct;
}

mod utils {
    pub mod process;
    pub mod print;
}


use crate::utils::process;
use crate::utils::print;
use mymodels::process_struct::ProcessStruct;

use std::{io, thread};
use std::time::Duration;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo, terminal::EnterAlternateScreen};

fn main() {
    let (_cols, rows) = crossterm::terminal::size().unwrap();
    let header_rows = 25; // for example
    let visible_rows = rows as usize - header_rows;
println!("rows{}",rows);
    execute!(io::stdout(), EnterAlternateScreen).unwrap();
let offset=0;
    loop {
        execute!(io::stdout(), MoveTo(0,0)).unwrap();
        execute!(io::stdout(), Clear(ClearType::All)).unwrap();

        let processes: Vec<ProcessStruct> = process::process();
        let slice = &processes[offset..(offset + visible_rows).min(processes.len())];
        print::print(slice.to_vec());


        thread::sleep(Duration::from_secs(3));
    }
}










