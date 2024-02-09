use pbc_zk::*;

#[allow(unused)]
const SECRET_NUMBER_VARIABLE_KIND: u8 = 0u8;

#[zk_compute(shortname = 0x60)]
pub fn multiply() -> Sbi32 {
    let a = 32u32;
    let b = 3u32;

    let c = a * b;

    if c > 100u32 {
        return Sbi32::from(0);
    }

    Sbi32::from(1)
}
