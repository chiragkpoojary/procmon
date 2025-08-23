
mod mymodels {
    pub mod process_struct;
}

mod utils {
    pub mod process;
}

use crate::utils::process;
use mymodels::process_struct::ProcessStruct;
fn main() {
    let processes: Vec<ProcessStruct> = process::process();

    for p in processes {
        println!(
            "PID: {:<6} NAME: {:<20} CPUTime/sc: {:<5} User: {:<5} Kernel: {:<5} VSZ: {:<8} MB RSS: {:<8} MB",
            p.pid,
            p.name,
            p.cpu,
            p.cpu_user,
            p.cpu_kernel,
            p.memory_rss ,  // convert KB → MB
            p.memory_vsz    // convert KB → MB
        );
    }

}
