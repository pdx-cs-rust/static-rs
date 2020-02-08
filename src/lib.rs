use std::fs::File;
use std::os::unix::io::FromRawFd;
use std::os::raw::c_int;
use std::io::Write;

pub fn writeln(msg: &'static str) {
    let mut stdout = unsafe {
        File::from_raw_fd(0 as c_int)
    };
    writeln!(stdout, "{}", msg).unwrap();
}

