use super::super::super::TunDev;
use super::sys::{tunsetiff, Cifreq, IFF_TUN};
use libc::c_char;
use mio::event::Source;
use mio::unix::SourceFd;
use mio::{Interest, Registry, Token};
use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Result, Write};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::ptr;

pub struct LinuxDev {
    name: String,
    fd: File,
}

impl LinuxDev {
    pub fn new(name: &str) -> Result<Self> {
        let linux_dev = unsafe {
            let fd = OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/net/tun")?;

            let mut req: Cifreq = mem::zeroed();
            let cstring_name = CString::new(name)?;

            ptr::copy_nonoverlapping(
                cstring_name.as_ptr() as *const c_char,
                req.ifrn.name.as_mut_ptr(),
                name.as_bytes().len(),
            );

            req.ifru.flags = IFF_TUN;

            if tunsetiff(fd.as_raw_fd(), &mut req as *mut _ as *mut _)? < 0 {
                return Err(Error::last_os_error());
            };

            LinuxDev {
                name: CStr::from_ptr(req.ifrn.name.as_ptr())
                    .to_string_lossy()
                    .into(),
                fd,
            }
        };

        Ok(linux_dev)
    }

    fn send(&self, buf: &[u8]) -> Result<usize> {
        (&self.fd).write(buf)
    }

    fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        (&self.fd).read(buf)
    }
}

impl TunDev for LinuxDev {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Read for LinuxDev {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.recv(buf)
    }
}

impl Write for LinuxDev {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.send(buf)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Source for LinuxDev {
    fn register(&mut self, registry: &Registry, token: Token, interests: Interest) -> Result<()> {
        SourceFd(&self.fd.as_raw_fd()).register(registry, token, interests)
    }

    fn reregister(&mut self, registry: &Registry, token: Token, interests: Interest) -> Result<()> {
        SourceFd(&self.fd.as_raw_fd()).reregister(registry, token, interests)
    }

    fn deregister(&mut self, registry: &Registry) -> Result<()> {
        SourceFd(&self.fd.as_raw_fd()).deregister(registry)
    }
}
