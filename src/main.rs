
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
use terminal_size::{terminal_size, Height, Width};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo, terminal::EnterAlternateScreen, terminal};

fn main() {

    let (_cols, rows) = terminal_size().unwrap();
    let header_rows: usize = 3;

    let visible_rows: usize = if let Some((Width(_), Height(rows))) = terminal_size() {
        rows as usize - header_rows
    } else {
        10
    };
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










