use mio::event::Source;
use std::io::{Read, Write};

pub trait TunDev: Read + Write + Source {
    fn name(&self) -> &str;
}
