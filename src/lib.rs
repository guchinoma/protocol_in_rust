mod device;
mod nif;
mod protocols;
mod socket;

pub trait Info {
    fn info(&self) {}
}
