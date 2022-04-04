use std::borrow::Cow;
use std::ffi::CStr;
use std::fmt::{self, Write};
use std::mem::zeroed;

unsafe fn name_from_c<'a>(i: &'a [libc::c_char]) -> Cow<'a, str> {
    CStr::from_ptr(i.as_ptr()).to_string_lossy()
}

fn main() {
    run().ok();
}

fn run() -> fmt::Result {
    let mut out = String::new();
    unsafe {
        let mut uts: libc::utsname = zeroed();
        let r = libc::uname(&mut uts);
        if r != -1 {
            writeln!(&mut out, "uts.system = {}", name_from_c(&uts.sysname))?;
            writeln!(&mut out, "uts.node = {}", name_from_c(&uts.nodename))?;
            writeln!(&mut out, "uts.release = {}", name_from_c(&uts.release))?;
            writeln!(&mut out, "uts.version = {}", name_from_c(&uts.version))?;
            writeln!(&mut out, "uts.machine = {}", name_from_c(&uts.machine))?;
        }

        writeln!(&mut out, "pid = {}", libc::getpid())?;
        writeln!(&mut out, "uid = {}", libc::getuid())?;
    }

    for (k, v) in std::env::vars() {
        writeln!(&mut out, "envs.{} = {:?}", k, v)?;
    }

    for (i, arg) in std::env::args().enumerate() {
        writeln!(&mut out, "argv[{}] = {:?}", i, arg)?;
    }

    println!("{}", out);
    Ok(())
}
