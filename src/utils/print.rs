
use crate::mymodels::process_struct::ProcessStruct;
use prettytable::{Table, row};
pub fn print (processes:Vec<ProcessStruct>){

    let mut table = Table::new();

    // Add a row per time
    table.add_row(row![ "PID",
        "NAME                                 ",
        "CPU Time/s",
        "CPU_User",
        "CPU_Kernel",
        "Priority",
        "Nice",
        "Num Threads",
        "VSZ [Resident Set Size](KB)",
        "RSS [Virtual Memory Size] (KB)"]);
    for p in processes {
        table.add_row(row![
            p.pid,
            p.name,
            format!("{:.2}", p.cpu),
            format!("{:.2}", p.cpu_user),
            format!("{:.2}", p.cpu_kernel),
            format!("{:.2}", p.priority),
            format!("{:.2}", p.nice),
                 format!("{:.2}", p.num_thread),
            format!("{:.2}", p.memory_vsz),
            format!("{:.2}", p.memory_rss),
        ]);
    }

    table.printstd();

}