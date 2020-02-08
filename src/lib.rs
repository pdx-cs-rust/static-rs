use std::fs::File;
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::io::Write;

pub fn writeln_rs(msg: &str) {
    let mut stdout = unsafe {
        File::from_raw_fd(1)
    };
    writeln!(stdout, "{}", msg).unwrap();
    let _ = stdout.into_raw_fd();
}

pub fn writeln_c(msg: &str) {
    let mut buf: Vec<u8> = msg.bytes().collect();
    buf.push(b'\n');
    let nbuf = buf.len();
    let buf = buf.as_ptr();
    let result = unsafe {
        libc::write(
            1,
            buf as *const libc::c_void,
            nbuf,
        )
    };
    if result < 0 {
        eprintln!("write returned {}", result);
        unsafe { libc::perror(b"write\0".as_ptr() as *const libc::c_char) };
    }
}
