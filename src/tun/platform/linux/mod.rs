mod dev;
mod sys;

use self::dev::LinuxDev;
use std::io::Result;

pub fn create_tun(name: &str) -> Result<LinuxDev> {
    dev::LinuxDev::new(name)
}
