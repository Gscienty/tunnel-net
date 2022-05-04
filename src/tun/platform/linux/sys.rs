use libc::sockaddr;
use libc::{c_char, c_int, c_short, c_uchar, c_uint, c_ulong, c_void};

pub const IFNAMSIZ: usize = 16;
pub const IFF_TUN: c_short = 0x0001;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Cifmap {
    pub mem_start: c_ulong,
    pub mem_end: c_ulong,
    pub base_addr: c_ulong,
    pub irq: c_uchar,
    pub dma: c_uchar,
    pub port: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Cifsu {
    pub raw_hdlc_proto: *mut c_void,
    pub cisco: *mut c_void,
    pub fr: *mut c_void,
    pub fr_pvc: *mut c_void,
    pub fr_pvc_info: *mut c_void,
    pub sync: *mut c_void,
    pub te1: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Cifsettings {
    pub type_: c_uint,
    pub size: c_uint,
    pub ifsu: Cifsu,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union Cifru {
    pub addr: sockaddr,
    pub dstaddr: sockaddr,
    pub broadaddr: sockaddr,
    pub netmask: sockaddr,
    pub hwaddr: sockaddr,

    pub flags: c_short,
    pub ivalue: c_int,
    pub mtu: c_int,
    pub map: Cifmap,
    pub slave: [c_char; IFNAMSIZ],
    pub newname: [c_char; IFNAMSIZ],
    pub data: *mut c_void,
    pub settings: Cifsettings,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union Cifrn {
    pub name: [c_char; IFNAMSIZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Cifreq {
    pub ifrn: Cifrn,
    pub ifru: Cifru,
}

nix::ioctl_write_ptr!(tunsetiff, b'T', 202, c_int);
