use std::fs::File;
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::io::Write;

/// Use Rust `writeln!()` to print directly to UNIX
/// `stdout`.
pub fn writeln_rs(msg: &str) {
    // Get a `File` for file descriptor 1 (`stdout`).
    let mut stdout = unsafe { File::from_raw_fd(1) };
    // Write the message.
    writeln!(stdout, "{}", msg).unwrap();
    // Make sure to take apart the `File`; this will free
    // its storage without closing the file descriptor in
    // `File::drop()`.
    let _ = stdout.into_raw_fd();
}

/// Use `libc::write()` to print to UNIX `stdout`.
pub fn writeln_c(msg: &str) {
    // Get the `char` buffer that `write()` needs.
    let mut buf: Vec<u8> = msg.bytes().collect();
    // Tack on the newline. Note that there is no need to
    // null-terminate this, because `write()` takes a count.
    buf.push(b'\n');
    // Remember how many characters to write.
    let nbuf = buf.len();
    // Get the buffer pointer `write()` wants.
    let buf = buf.as_ptr() as *const libc::c_void;
    // Do the write.
    let result = unsafe { libc::write(1, buf, nbuf) };
    // Check whether something went wrong with the write.
    if result < 0 {
        eprintln!("write returned {}", result);
        // Use `perror()` to give a more detailed error message.
        unsafe { libc::perror(b"write\0".as_ptr() as *const libc::c_char) };
    }
}

extern {
    // C.f. `writeln_ffi_c.c`.
    fn writeln_ffi_c(msg: *const std::os::raw::c_char);
}


/// Use an external C function to print to UNIX `stdout`.
pub fn writeln_ffi(msg: &str) {
    // Get the `char` buffer that `writeln_ffi_c()` needs.
    let mut buf: Vec<u8> = msg.bytes().collect();
    // Null-terminate the buffer.
    buf.push(b'\0');
    // Get the pointer that the printing routine needs.
    let buf = buf.as_ptr() as *const std::os::raw::c_char;
    // Go.
    unsafe { writeln_ffi_c(buf); }
}
