// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525550: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0070a189: i8;
    static mut DAT_0085e792: i8;
    static mut DAT_0085e79b: i8;
    static mut DAT_0071b730: [i8; 1];
    static mut DAT_006d4988: i32;
    static mut DAT_0070a1d0: i32;
    static mut DAT_0073bfe8: [i8; 1];
    static mut DAT_0073bfb8: [i8; 1];
    static mut DAT_0073ba30: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00439830(a: i32, b: i32) -> u32;
    fn FUN_004398b0(a: i32);
    fn FUN_0040a4b0(a: u32, b: u32) -> u32;
    fn FUN_00490a50(a: *mut i8, b: i32, c: i32, d: i32, e: u32, f: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040a980(param_1: i32, param_2: i32) {
    // SAFETY: globals and callees are the game's own memory and code; the
    // pointers chased are the game's structures, read where the original reads.
    unsafe {
        if !(DAT_00525550 == 0
            && DAT_0071ecf4 == param_2
            && DAT_0070a189 != 0
            && DAT_0085e792 != 0
            && DAT_0085e79b != 0)
        {
            return;
        }
        let base = (&raw mut DAT_0071b730).cast::<u8>();
        let t1 = base
            .byte_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<i32>()
            .read_unaligned();
        let addr2 = (t1 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_006d4988 as u32);
        let t2 = (addr2 as usize as *const i32).read_unaligned();
        let addr3 = (t2 as u32).wrapping_add(0xa7);
        let iv4 = (addr3 as usize as *const i32).read_unaligned();
        let p = iv4 as u32 as usize as *const u8;

        if param_1 == 0 || (p.byte_add(0x1e0).read() & 1) == 0 {
            let mut i: i32 = 0x34;
            loop {
                if FUN_00439830(i, 0x100) != 0 {
                    FUN_004398b0(i);
                }
                i = i.wrapping_add(1);
                if i >= 0x3e {
                    break;
                }
            }
            return;
        }

        let c = p.byte_add(0x233).read() as i8;
        let u2: u32;
        if c == 1 {
            if FUN_00439830(0x34, 0x100) != 0 {
                return;
            }
            u2 = 0x35;
        } else if c == 2 {
            if FUN_00439830(0x37, 0x100) != 0 {
                return;
            }
            u2 = 0x38;
        } else if c != 3 {
            if FUN_00439830(0x3c, 0x100) != 0 {
                return;
            }
            if FUN_00439830(0x3d, 0x100) != 0 {
                return;
            }
            let u = FUN_0040a4b0(0xffffffff, 0x3d);
            FUN_00490a50((&raw mut DAT_0073bfe8).cast::<i8>(), 1, 1, 0x7d, u, 0x40);
            return;
        } else {
            if FUN_00439830(0x3a, 0x100) != 0 {
                return;
            }
            if FUN_00439830(0x3b, 0x100) != 0 {
                return;
            }
            let u = FUN_0040a4b0(0xffffffff, 0x3b);
            FUN_00490a50((&raw mut DAT_0073bfb8).cast::<i8>(), 1, 1, 0x7d, u, 0x40);
            return;
        }

        if DAT_0070a1d0 == -1 {
            if FUN_00439830(u2.wrapping_add(1) as i32, 0x100) != 0 {
                FUN_004398b0(u2.wrapping_add(1) as i32);
            }
            if FUN_00439830(u2 as i32, 0x100) == 0 {
                let u3 = FUN_0040a4b0(0xffffffff, u2);
                let ptr = (&raw mut DAT_0073ba30)
                    .cast::<i8>()
                    .byte_add(u2.wrapping_mul(0x18) as usize);
                FUN_00490a50(ptr, 1, 1, 0x7d, u3, 0x40);
            }
        } else {
            if FUN_00439830(u2 as i32, 0x100) != 0 {
                FUN_004398b0(u2 as i32);
            }
            if FUN_00439830(u2.wrapping_add(1) as i32, 0x100) == 0 {
                let u3 = FUN_0040a4b0(0xffffffff, u2.wrapping_add(1));
                let a = u2.wrapping_mul(0x18).wrapping_add(0x73ba48);
                FUN_00490a50(a as usize as *mut i8, 1, 1, 0x7d, u3, 0x40);
            }
        }
    }
}
