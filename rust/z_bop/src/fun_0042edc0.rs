// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523410: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009d80c9: i8;
    static mut DAT_00bb7372: i8;
    static mut DAT_00bb737b: i8;
    static mut DAT_009e9670: [u8; 1];
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d8110: i32;
    static mut DAT_00a0b3a8: [i8; 1];
    static mut DAT_00a0b378: [i8; 1];
    static mut DAT_00a0adf0: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9300(a: i32, b: i32) -> u32;
    fn FUN_004a9380(a: i32);
    fn FUN_0042e810(a: i32, b: u32) -> u32;
    fn FUN_0042d120(a: *mut i8, b: i32, c: i32, d: i32, e: u32, f: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042edc0(param_1: i32, param_2: i32) {
    // SAFETY: the globals and callees are the game's own; the pointers
    // dereferenced are the game's structures, read where the original does.
    unsafe {
        if !(DAT_00523410 == 0
            && DAT_009ecc34 == param_2
            && DAT_009d80c9 != 0
            && DAT_00bb7372 != 0
            && DAT_00bb737b != 0)
        {
            return;
        }
        let base = (&raw mut DAT_009e9670).cast::<u8>();
        let i_tmp0: i32 = base
            .byte_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<i32>()
            .read_unaligned();
        let addr1 = (i_tmp0 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8 as u32);
        let i_tmp1: i32 = (addr1 as usize as *const i32).read_unaligned();
        let i_var4: i32 =
            ((i_tmp1 as u32).wrapping_add(0xa9) as usize as *const i32).read_unaligned();
        let p = i_var4 as u32 as usize;

        if param_1 == 0 || (*((p.wrapping_add(0x1e6)) as *const u8) & 1) == 0 {
            let mut i = 0x34i32;
            loop {
                let u = FUN_004a9300(i, 0x100);
                if u != 0 {
                    FUN_004a9380(i);
                }
                i = i.wrapping_add(1);
                if i >= 0x3e {
                    break;
                }
            }
            return;
        }

        let c_var1 = *((p.wrapping_add(0x23d)) as *const i8);
        let u_var2: u32;
        if c_var1 == 1 {
            if FUN_004a9300(0x34, 0x100) != 0 {
                return;
            }
            u_var2 = 0x35;
        } else if c_var1 != 2 {
            if c_var1 != 3 {
                if FUN_004a9300(0x3c, 0x100) != 0 {
                    return;
                }
                if FUN_004a9300(0x3d, 0x100) != 0 {
                    return;
                }
                let u = FUN_0042e810(-1, 0x3d);
                FUN_0042d120((&raw mut DAT_00a0b3a8).cast::<i8>(), 1, 1, 0x7d, u, 0x40);
                return;
            }
            if FUN_004a9300(0x3a, 0x100) != 0 {
                return;
            }
            if FUN_004a9300(0x3b, 0x100) != 0 {
                return;
            }
            let u = FUN_0042e810(-1, 0x3b);
            FUN_0042d120((&raw mut DAT_00a0b378).cast::<i8>(), 1, 1, 0x7d, u, 0x40);
            return;
        } else {
            if FUN_004a9300(0x37, 0x100) != 0 {
                return;
            }
            u_var2 = 0x38;
        }

        if DAT_009d8110 == -1 {
            let u3 = FUN_004a9300(u_var2.wrapping_add(1) as i32, 0x100);
            if u3 != 0 {
                FUN_004a9380(u_var2.wrapping_add(1) as i32);
            }
            let u3 = FUN_004a9300(u_var2 as i32, 0x100);
            if u3 == 0 {
                let u3 = FUN_0042e810(-1, u_var2);
                let ptr = (&raw mut DAT_00a0adf0)
                    .cast::<u32>()
                    .wrapping_add(u_var2.wrapping_mul(6) as usize);
                FUN_0042d120(ptr.cast::<i8>(), 1, 1, 0x7d, u3, 0x40);
            }
        } else {
            let u3 = FUN_004a9300(u_var2 as i32, 0x100);
            if u3 != 0 {
                FUN_004a9380(u_var2 as i32);
            }
            let u3 = FUN_004a9300(u_var2.wrapping_add(1) as i32, 0x100);
            if u3 == 0 {
                let u3 = FUN_0042e810(-1, u_var2.wrapping_add(1));
                let addr = u_var2.wrapping_mul(0x18).wrapping_add(0xa0ae08);
                FUN_0042d120(addr as usize as *mut i8, 1, 1, 0x7d, u3, 0x40);
            }
        }
    }
}
