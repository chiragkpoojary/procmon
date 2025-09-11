use crate::mymodels::process_struct::ProcessStruct;
use procfs::process;


pub fn process() -> Vec<ProcessStruct> {
    let mut processes_vec = Vec::new();

    let all_processes = process::all_processes();
    match all_processes {
        Ok(processes) => {
            for proc in processes {
                match proc {
                    Ok(proc) => {
                        match proc.stat() {
                            Ok(stat) => {
                                let process_name = stat.comm;
                                let total_time = stat.utime/procfs::ticks_per_second() + stat.stime/procfs::ticks_per_second();

                                let process_details = ProcessStruct {
                                    pid: proc.pid,
                                    name: process_name,
                                    cpu: total_time as f32,
                                    cpu_user:(stat.utime/procfs::ticks_per_second())as f32,
                                    cpu_kernel:(stat.stime/procfs::ticks_per_second())as f32,
                                    priority:stat.priority,
                                    nice:stat.nice,
                                    num_thread:stat.num_threads,
                                    memory_rss:stat.rss/1024,
                                    memory_vsz:stat.vsize/1024,
                               




                                };

                                processes_vec.push(process_details);
                            }
                            Err(e) => eprintln!(
                                "Cannot read stat for PID {}: {}",
                                proc.pid, e
                            ),
                        }
                    }
                    Err(e) => eprintln!("Error reading process info: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Failed to read processes: {}", e),
    }

    processes_vec.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap());
    processes_vec

}
