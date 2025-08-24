#[derive(Clone)]
pub struct ProcessStruct {
    pub pid: i32,

    pub name: String,


    // CPU
    pub cpu: f32,
    pub cpu_user: f32,
    pub cpu_kernel: f32,
    pub priority: i64,
    pub nice: i64,
    pub num_thread:i64,

    // Memory
    pub memory_rss: u64,
    pub memory_vsz: u64,



}
