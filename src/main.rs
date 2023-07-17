use std::env;
use std::fs::OpenOptions;
use std::io;
use std::os::fd::AsRawFd;
use nix::ioctl_write_int;

const USBDEVFS_MAGIC: u8 = b'U';
const USBDEVFS_RESET_CODE: u8 = 20;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: usbreset device-filename");
        return Ok(());
    }

    let filename = &args[1];
    let fd = OpenOptions::new().write(true).create_new(true).read(false).open(filename)?;

    println!("Resetting USB device {}", filename);

    ioctl_write_int!(usbdevfs_reset, USBDEVFS_MAGIC, USBDEVFS_RESET_CODE);
    let rc = unsafe {
        usbdevfs_reset(fd.as_raw_fd(), 0)
    };
    if rc.is_err() {
        eprintln!("Error in ioctl");
        return Ok(());
    }

    println!("Reset successful");

    Ok(())
}
