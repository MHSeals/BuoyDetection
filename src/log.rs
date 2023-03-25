use std::{
    fs::File,
    io::{Read, Write},
    time::SystemTime,
};

const LOGFILE: &str = "/log";

pub fn log(log_msg: &str){
    let mut logfile = File::create(LOGFILE).expect("Unable to open log file");
    logfile.write_all(format!("\n{}", log_msg).as_bytes()).expect("Unable to write to log file");
}
pub fn time_log(log_msg: &str){
    let current_time: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
        Ok(n) => n.as_secs(),
        Err(_) => panic!("Fix your clock. You before Unix Epoch"),
    };
    let log_with_time: String = format!("{}\n------\n{}", current_time, log_msg);
    log(&log_with_time);
}
