use crate::nif::{Nif, V4V6Addr, CFD};

pub struct TCP {
    s: V4V6Addr,
    d: V4V6Addr,
    c: Vec<u8>,
}

pub struct UDP {
    s: V4V6Addr,
    d: V4V6Addr,
    c: Vec<u8>,
}

pub struct IP {
    s: V4V6Addr,
    d: V4V6Addr,
    c: Vec<u8>,
}

pub trait Handler: Sync + Send + 'static {
    fn send(&self) {}
}

impl Handler for TCP {
    fn send(&self) {}
}

impl Handler for UDP {
    fn send(&self) {}
}

impl Handler for IP {
    fn send(&self) {}
}

pub fn protocol_init() -> Vec<Box<dyn Handler>> {
    let mut a: Vec<Box<dyn Handler>> = vec![];
    a
}
