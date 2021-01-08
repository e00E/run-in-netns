use anyhow::{anyhow, bail, Context, Result};
use std::{
    fs::File,
    os::unix::{io::AsRawFd, process::ExitStatusExt},
    process::{exit, Command, ExitStatus},
};

// Edit this path to point to the target namespace, usually located at `/var/run/netns`
// (see `man ip netns`).
const NAMESPACE_PATH: &str = "/var/run/netns/NAME";

fn main() -> Result<()> {
    enter_network_namespace()?;
    drop_privileges()?;
    let exit_status = execute_command()?;

    if let Some(code) = exit_status.code() {
        exit(code);
    }
    if let Some(signal) = exit_status.signal() {
        exit(128 + signal);
    }
    bail!("failed to get exit code");
}

fn enter_network_namespace() -> Result<()> {
    let netns = File::open(NAMESPACE_PATH)
        .context(format!("failed to open namespace {}", NAMESPACE_PATH))?;
    uapi::setns(netns.as_raw_fd(), uapi::c::CLONE_NEWNET).context(format!(
        "failed to enter network namespace {}",
        NAMESPACE_PATH
    ))?;
    Ok(())
}

fn drop_privileges() -> Result<()> {
    uapi::seteuid(uapi::getuid()).context("failed to reset uid")?;
    uapi::setegid(uapi::getgid()).context("failed to reset gid")?;
    Ok(())
}

fn execute_command() -> Result<ExitStatus> {
    let mut args = std::env::args_os().skip(1);
    let command = args.next().ok_or_else(|| anyhow!("missing argument"))?;
    Command::new(command)
        .args(args)
        .status()
        .context("failed to run command")
}
