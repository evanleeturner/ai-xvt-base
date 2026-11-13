// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d8e25: [u8; 0];
    static mut DAT_009d8e26: [u8; 0];
    static mut DAT_009d8e27: [u8; 0];
    static mut DAT_009d8e33: [u8; 0];
    static mut DAT_009d8e28: [u8; 0];
    static mut DAT_009d8e29: [u8; 0];
    static mut DAT_009d6968: [u8; 0];
    static mut DAT_009ed228: u32;
    static mut DAT_009ed22c: u32;
    static mut DAT_009ed22d: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00435a10(a: u32, b: u32, c: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00435850(param_1: u32, param_2: u32, param_3: i32) -> u32 {
    // SAFETY: all globals and the callee are the game's own memory; table
    // accesses are at the same offsets the original uses.
    unsafe {
        let b25 = (&raw mut DAT_009d8e25).cast::<u8>();
        let b26 = (&raw mut DAT_009d8e26).cast::<u8>();
        let b27 = (&raw mut DAT_009d8e27).cast::<u8>();
        let b33 = (&raw mut DAT_009d8e33).cast::<u8>();
        let b28 = (&raw mut DAT_009d8e28).cast::<u8>();
        let b29 = (&raw mut DAT_009d8e29).cast::<u8>();
        let b68 = (&raw mut DAT_009d6968).cast::<u8>();

        let mut i5: i32 = 8;
        let mut u2: u32 = (param_2 & 0xffff).wrapping_mul(0x2b3);
        let mut i4: i32 = ((param_2 & 0xffff) as i32).wrapping_mul(0x566);
        let mut local_4: i32 = 0;
        loop {
            let t = *b25.offset(i4 as isize);
            u2 = (u2 >> 8) | u32::from(t);
            if t == 0x02 {
                u2 = u32::from(*b26.offset(i4 as isize));
                if u2 == (param_1 & 0xffff) {
                    let b1 = *b27.offset(i4 as isize);
                    u2 = u32::from(b1);
                    if b1 == 0x12 || (b1 == 0x13 && param_3 == 1) {
                        u2 = FUN_00435a10(DAT_009ed228, DAT_009ed22c, DAT_009ed22d);
                        let m = u32::from(*b33.offset(i4 as isize)).wrapping_mul(5);
                        if m == 0 || u2 <= m {
                            local_4 = (*b28.offset(i4 as isize) as i8) as i32;
                        }
                        u2 = 0;
                        let mut i3: i32 = 0;
                        local_4 = local_4.wrapping_mul(0xfa);
                        loop {
                            if *b29.offset(i3.wrapping_add(i4) as isize) != 0 {
                                let p = b68.offset(i3 as isize);
                                *p = (*p).wrapping_add(local_4 as u8);
                            }
                            i3 = i3.wrapping_add(1);
                            if i3 >= 10 {
                                break;
                            }
                        }
                    }
                }
            }
            i4 = i4.wrapping_add(0x4e);
            i5 = i5.wrapping_sub(1);
            if i5 == 0 {
                break;
            }
        }
        u2
    }
}
