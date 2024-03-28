mod device;
mod nif;
mod socket;

pub trait Info {
    fn info(&self) {}
}
