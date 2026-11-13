// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b6: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08294: i32;
    static mut DAT_00a08104: i32;
    static mut DAT_0051a44f: [u8; 1];
    static mut DAT_0051c5aa: [u8; 1];
    static mut DAT_0051c5ac: [u8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004686b0() -> u32 {
    // SAFETY: all globals and the memory they address belong to the game;
    // reads happen where the original reads.
    unsafe {
        let mut u5: u32 = ((DAT_009a73b6.wrapping_add(0x18)) as *const u16).read_unaligned() as u32;
        let c11: u8;
        if (u5 as i32) < DAT_009a1fe0 {
            let b = (u5
                .wrapping_mul(0x23)
                .wrapping_add(2)
                .wrapping_add(DAT_009a1fe8 as u32) as *const u8)
                .read();
            u5 = ((((DAT_009a1fe8 >> 8).wrapping_shl(8)) as u32) | b as u32) & 0xffff00ff;
            if u5 == 3 || u5 == 5 || u5 == 4 || (u5 == 1 && DAT_00a08294 == 0xe) {
                c11 = 2;
            } else {
                c11 = 1;
            }
        } else {
            c11 = 1;
        }
        u5 &= 0xffff0000;

        let base = DAT_00a08104 as u32;
        let t_a = (&raw const DAT_0051a44f).cast::<u8>();
        let t_b = (&raw const DAT_0051c5aa).cast::<u8>();
        let t_c = (&raw const DAT_0051c5ac).cast::<u8>();

        if (base.wrapping_add(0x232) as *const u8).read() != 0 {
            loop {
                let b1 = ((u5 & 0xffff).wrapping_add(0x233).wrapping_add(base) as *const u8).read();
                let go: bool;
                if t_a.add(b1 as usize).read() == c11 {
                    go = true;
                } else {
                    go = DAT_00a08294 == 0xe && (b1 == 0x98 || b1 == 0x99) && c11 == 2;
                }
                if go {
                    let i9 = ((base.wrapping_add(4) as *const u8).read() as i32)
                        .wrapping_mul(0x101)
                        .wrapping_add((u5 & 0xffff) as i32);
                    let mut u10: u32 = t_b.offset(i9 as isize).read() as u32;
                    if u10 <= t_c.offset(i9 as isize).read() as u32 {
                        let u6 = u5;
                        loop {
                            let c2 = (base.wrapping_add(0x372).wrapping_add(u10.wrapping_mul(4))
                                as *const i8)
                                .read();
                            let u7 = (u6 >> 16) as u16;
                            if c2 != 0 {
                                return ((u7 as u32) << 16) | 1;
                            }
                            let u8 = (u10 as u16).wrapping_add(1);
                            u10 = u8 as u32;
                            if !((u8 as u32) <= t_c.offset(i9 as isize).read() as u32) {
                                u5 = ((u7 as u32) << 16) | (u5 as u16 as u32);
                                break;
                            }
                        }
                    }
                }
                let u8 = (u5 as u16).wrapping_add(1);
                u5 = ((u5 >> 16) << 16) | u8 as u32;
                if !((u8 as u32) < (base.wrapping_add(0x232) as *const u8).read() as u32) {
                    break;
                }
            }
        }
        u5 & 0xffff0000
    }
}
