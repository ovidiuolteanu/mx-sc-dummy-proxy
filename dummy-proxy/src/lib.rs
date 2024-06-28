#![no_std]

use multiversx_sc::imports::*;
pub mod dummy_proxy;

#[multiversx_sc::contract]
pub trait DummyProxyContract: dummy_proxy::DummyProxyModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
