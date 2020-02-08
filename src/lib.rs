pub fn writeln(msg: &'static str) {
    let mut stdout = unsafe {
        <std::fs::File as std::os::unix::io::FromRawFd>::from_raw_fd(0 as std::os::raw::c_int)
    };
    let _ = write!(&mut stdout as &mut dyn std::io::Write, "{}\n", msg).unwrap();
}

