use std::ffi::{OsStr, OsString};
use std::borrow::Borrow;

pub trait Append<Segment : ?Sized> : Sized
    where
        Segment : ToOwned<Owned = Self>,
        Self : Borrow<Segment>,
{
    fn append (self: Self, s: impl AsRef<Segment>)
               -> Self
    ;
}

impl Append<OsStr> for OsString {
    fn append (mut self: OsString, s: impl AsRef<OsStr>)
               -> Self
    {
        self.push(s);
        self
    }
}
