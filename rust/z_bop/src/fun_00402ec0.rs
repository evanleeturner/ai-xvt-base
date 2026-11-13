// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a73b0: i32;
    static mut DAT_009a1ff8: i32;
    static mut DAT_00524100: i32;
    static mut DAT_009a73d0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00403540(param_1: u32, param_2: u32);
    fn FUN_00425b70(param_1: u32, param_2: u32) -> u32;
    fn FUN_00403360(param_1: u32, param_2: u32) -> i32;
}

unsafe fn rd32(addr: u32) -> u32 {
    // SAFETY: the address is a game-memory address read where the original reads it.
    unsafe { (addr as usize as *const u32).read_unaligned() }
}

unsafe fn rd8(addr: u32) -> u8 {
    // SAFETY: the address is a game-memory address read where the original reads it.
    unsafe { (addr as usize as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00402ec0(param_1: u32, param_2: u32, param_3: i32) -> u32 {
    let _ = param_1;
    // SAFETY: the globals and callees are the game's own; all pointers are
    // game records read exactly as the original does.
    unsafe {
        let iv3: i32 = 'blk: {
            if param_2 == 0xffff {
                break 'blk 0;
            }
            let mut iv3: u32 = (DAT_009a1fe8 as u32).wrapping_add(param_2.wrapping_mul(0x23));
            let u = rd32(iv3.wrapping_add(0x1f));
            if u == 0 {
                let b2: u32 = (rd8(iv3.wrapping_add(3)) == 0) as u32;
                break 'blk 1i32.wrapping_sub(b2 as i32);
            }
            if rd8(iv3.wrapping_add(3)) == 0 {
                break 'blk 0;
            }
            let u = rd32(iv3.wrapping_add(0x1f));
            iv3 = rd32(u.wrapping_add(0xa9));
            if iv3 == 0 {
                break 'blk 1;
            }
            if rd8(iv3.wrapping_add(0x5e)) == 0x15 && rd8(iv3.wrapping_add(0x5f)) != 0 {
                break 'blk 0;
            }
            let c = rd8(iv3.wrapping_add(7));
            if c == 1 || c == 3 || c == 4 {
                break 'blk 0;
            }
            if rd8(iv3.wrapping_add(0x23d)) == 3 && rd8(iv3.wrapping_add(0x241)) != 0 {
                FUN_00403540(param_2, DAT_009a73b0 as u32);
                let addr = (DAT_009a73b0 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(2)
                    .wrapping_add(DAT_009a1fe8 as u32);
                let c = rd8(addr);
                if c == 0 || c == 1 || c == 2 {
                    if 0x8000 < DAT_009a1ff8 {
                        break 'blk 0;
                    }
                } else if 0x10000 < DAT_009a1ff8 {
                    break 'blk 0;
                }
            }
            let iv4: u32 = (DAT_009a1fe8 as u32).wrapping_add(param_2.wrapping_mul(0x23));
            if rd32(iv4.wrapping_add(0x1b)) as i32 != -1 {
                break 'blk 1;
            }
            let u = rd32(iv4.wrapping_add(0x1f));
            let ptr_a9 = rd32(u.wrapping_add(0xa9));
            let a = rd32(ptr_a9.wrapping_add(0xcb));
            let b = rd32(iv3.wrapping_add(0xc3));
            1i32.wrapping_sub((a < b) as i32)
        };
        if iv3 != 0 {
            let idx = (DAT_009a73d0 as u32).wrapping_mul(2) as usize;
            let w = (&raw mut DAT_00524100)
                .cast::<u8>()
                .byte_add(idx)
                .cast::<u16>()
                .read_unaligned();
            let mut u5 = FUN_00425b70(0x500, w as u32);
            u5 = (u5 & 0xffff).wrapping_add(0xa00);
            if param_3 != 0 {
                let u6 = FUN_00425b70(u5, 0x5555);
                u5 = u5.wrapping_add(u6 & 0xffff);
            }
            let r = FUN_00403360(param_2, u5.wrapping_shl(8));
            if r == 1 {
                return 1;
            }
        }
        0
    }
}
