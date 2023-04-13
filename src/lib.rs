use std::process::{Command, ExitStatus};
use std::io;

pub fn dispatch_external_git(exe: &str, args: &[&str]) -> io::Result<ExitStatus> {
    Command::new(exe).args(args).spawn()?.wait()
}
