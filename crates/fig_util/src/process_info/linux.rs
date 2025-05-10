use std::path::PathBuf;
use std::str::FromStr;

use nix::sys::signal::Signal;

pub trait LinuxExt {
    fn cmdline(&self) -> Option<String>;
}

use super::{
    Pid,
    PidExt,
};

pub fn terminate_process(pid: Pid) -> Result<(), String> {
    let nix_pid = nix::unistd::Pid::from_raw(pid.0);
    nix::sys::signal::kill(nix_pid, Signal::SIGTERM).map_err(|e| format!("Failed to terminate process: {}", e))
}

impl PidExt for Pid {
    fn current() -> Self {
        nix::unistd::getpid().into()
    }

    fn parent(&self) -> Option<Pid> {
        std::fs::read_to_string(format!("/proc/{self}/status"))
            .ok()
            .and_then(|s| {
                s.lines()
                    .find(|line| line.starts_with("PPid:"))
                    .and_then(|line| line.strip_prefix("PPid:"))
                    .map(|line| line.trim())
                    .and_then(|pid_str| Pid::from_str(pid_str).ok())
            })
    }

    fn exe(&self) -> Option<PathBuf> {
        std::path::PathBuf::from(format!("/proc/{self}/exe")).read_link().ok()
    }
}

impl LinuxExt for Pid {
    fn cmdline(&self) -> Option<String> {
        std::fs::read_to_string(format!("/proc/{self}/cmdline"))
            .ok()
            .map(|s| s.replace('\0', ""))
    }
}
