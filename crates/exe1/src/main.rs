use std::ffi::CStr;
use std::mem::zeroed;

fn main() {
    let (pid, uid, sys) = unsafe {
        let pid = libc::getpid();
        let uid = libc::getuid();
        let mut utsname: libc::utsname = zeroed();
        let r = libc::uname(&mut utsname);
        let sys: String = if r != -1 {
            CStr::from_ptr(utsname.sysname.as_ptr())
                .to_string_lossy()
                .into_owned()
        } else {
            String::from("unknown")
        };
        (pid, uid, sys)
    };
    println!("Hello, pid = {}, uid = {}, system = {}", pid, uid, sys);
}
