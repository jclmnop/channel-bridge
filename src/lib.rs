#![cfg_attr(not(feature = "std"), no_std)]
#![feature(cfg_version)]
#![cfg_attr(feature = "nightly", feature(type_alias_impl_trait))]
#![cfg_attr(
    all(feature = "nightly", version("1.70")),
    feature(impl_trait_in_assoc_type)
)]

use core::fmt::Debug;

#[cfg(feature = "nightly")]
pub mod asynch;
#[cfg(feature = "notification")]
pub mod notification;

pub trait Sender {
    type Error: Debug;

    type Data;

    fn send<'a>(&'a mut self, data: &'a Self::Data) -> Result<(), Self::Error>;
}

impl<'t, T> Sender for &'t mut T
where
    T: Sender + 't,
{
    type Error = T::Error;

    type Data = T::Data;

    fn send<'a>(&'a mut self, data: &'a Self::Data) -> Result<(), Self::Error> {
        (*self).send(data)
    }
}

pub trait Receiver {
    type Error: Debug;

    type Data;

    fn recv(&mut self) -> Result<Self::Data, Self::Error>;
}

impl<'t, T> Receiver for &'t mut T
where
    T: Receiver + 't,
{
    type Error = T::Error;

    type Data = T::Data;

    fn recv(&mut self) -> Result<Self::Data, Self::Error> {
        (*self).recv()
    }
}
