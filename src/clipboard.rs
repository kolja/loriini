use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn set(text: &str) -> io::Result<()> {
    for (bin, args) in candidates() {
        match try_write(bin, args, text) {
            Ok(()) => return Ok(()),
            Err(_) => continue,
        }
    }
    Err(io::Error::other(
        "no clipboard backend available",
    ))
}

#[cfg(target_os = "macos")]
fn candidates() -> &'static [(&'static str, &'static [&'static str])] {
    &[("pbcopy", &[])]
}

#[cfg(target_os = "windows")]
fn candidates() -> &'static [(&'static str, &'static [&'static str])] {
    &[("clip", &[])]
}

#[cfg(all(unix, not(target_os = "macos")))]
fn candidates() -> &'static [(&'static str, &'static [&'static str])] {
    if std::env::var_os("WAYLAND_DISPLAY").is_some() {
        &[("wl-copy", &[])]
    } else {
        &[
            ("xclip", &["-selection", "clipboard"]),
            ("xsel", &["--clipboard", "--input"]),
        ]
    }
}

fn try_write(bin: &str, args: &[&str], text: &str) -> io::Result<()> {
    let mut child = Command::new(bin)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    child
        .stdin
        .as_mut()
        .expect("stdin was piped")
        .write_all(text.as_bytes())?;
    let status = child.wait()?;
    if !status.success() {
        return Err(io::Error::other(
            format!("{} exited with {}", bin, status),
        ));
    }
    Ok(())
}
