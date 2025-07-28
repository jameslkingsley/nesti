use bytesize::ByteSize;

use super::Element;

#[derive(Debug)]
pub struct Bytes(pub u64);
#[derive(Debug)]
pub struct Kilobytes(pub u64);
#[derive(Debug)]
pub struct Megabytes(pub u64);
#[derive(Debug)]
pub struct Gigabytes(pub u64);
#[derive(Debug)]
pub struct Terabytes(pub u64);
#[derive(Debug)]
pub struct Petabytes(pub u64);

impl Element for Bytes {
    type Context = ();

    fn content(&self, _ctx: &Self::Context) -> String {
        ByteSize::b(self.0).to_string()
    }
}

impl Element for Kilobytes {
    type Context = ();

    fn content(&self, _ctx: &Self::Context) -> String {
        ByteSize::kb(self.0).to_string()
    }
}

impl Element for Megabytes {
    type Context = ();

    fn content(&self, _ctx: &Self::Context) -> String {
        ByteSize::mb(self.0).to_string()
    }
}

impl Element for Gigabytes {
    type Context = ();

    fn content(&self, _ctx: &Self::Context) -> String {
        ByteSize::gb(self.0).to_string()
    }
}

impl Element for Terabytes {
    type Context = ();

    fn content(&self, _ctx: &Self::Context) -> String {
        ByteSize::tb(self.0).to_string()
    }
}

impl Element for Petabytes {
    type Context = ();

    fn content(&self, _ctx: &Self::Context) -> String {
        ByteSize::pb(self.0).to_string()
    }
}
