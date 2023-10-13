use cosmwasm_std::{Addr, HexBinary};
use rand::random;

use crate::bech32::bech32_encode;

pub fn hex(v: &str) -> HexBinary {
    HexBinary::from_hex(v).unwrap()
}

pub fn addr(v: &str) -> Addr {
    Addr::unchecked(v)
}

pub fn gen_bz(len: usize) -> HexBinary {
    let bz: Vec<_> = (0..len).map(|_| random::<u8>()).collect();
    bz.into()
}

pub fn gen_addr(hrp: &str) -> Addr {
    let bz = gen_bz(20);
    bech32_encode(hrp, bz.as_slice()).unwrap()
}
