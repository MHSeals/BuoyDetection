use std::{
    fs::File,
    io::{Read, Write},
    time::SystemTime, alloc::System,
};

const LOGFILE: &str = "/log";

pub fn log(log_msg: &str){
    let mut past_logfile = String::new();
    let mut logfile = File::open(LOGFILE).expect("Unable to open log file");
    logfile.read_to_string(&mut past_logfile).expect("Unable to read log file");
    let logfile_to_write: String = if past_logfile.len() != 0{
        format!("{}\n\n{}", past_logfile, log_msg)
    }
    else{
        format!("{}", log_msg)
    };
    logfile.write_all(logfile_to_write.as_bytes()).expect("Unable to write to log file");
}
pub fn time_log(log_msg: &str){
    let current_time: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
        Ok(n) => n.as_secs(),
        Err(_) => panic!("Fix your clock. You before Unix Epoch"),
    };
    let log_with_time: String = format!("{}\n------\n{}", current_time, log_msg);
    log(&log_with_time);
}
