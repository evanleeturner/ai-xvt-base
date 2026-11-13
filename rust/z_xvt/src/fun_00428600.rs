// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_00536f0d: u32;
    static mut _DAT_00536f09: u32;
    static mut DAT_00536f05: u32;
    static mut DAT_00536f21: u32;
    static mut DAT_00536efc: u32;
    static mut DAT_00536efd: u32;
    static mut DAT_00536efe: u32;
    static mut _DAT_00536f2d: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00428600(param_1: i32) {
    // SAFETY: param_1 is the address of the game's node structure and the
    // globals are the game's own memory; all reads and writes follow the
    // original's order and offsets.
    unsafe {
        let flags = ((param_1 as u32).wrapping_add(2) as *const u8).read();
        if flags != 0 {
            let mut u_var3: u32 = 0;
            loop {
                if ((1u32.wrapping_shl(u_var3 & 0x1f)) & (flags as u32)) != 0 {
                    let child_ptr = ((param_1 as u32)
                        .wrapping_add(0x32)
                        .wrapping_add(u_var3.wrapping_mul(4))
                        as *const i32)
                        .read_unaligned();
                    FUN_00428600(child_ptr);
                }
                u_var3 = u_var3.wrapping_add(1);
                if u_var3 >= 8 {
                    break;
                }
            }
        }

        let mut d_var2: f64 =
            f64::from_bits(((_DAT_00536f0d as u64) << 32) | (_DAT_00536f09 as u64));

        let val_10 = ((param_1 as u32).wrapping_add(10) as *const i32).read_unaligned();
        if val_10 != 0 {
            let val_6 = ((param_1 as u32).wrapping_add(6) as *const i32).read_unaligned();
            let pbvar4 = (val_6 as u32).wrapping_mul(9).wrapping_add(DAT_00536f05) as *const u8;
            let a0 = DAT_00536f21.wrapping_add(
                (pbvar4.read() as u32)
                    .wrapping_sub(DAT_00536efc)
                    .wrapping_mul(4),
            );
            let l0 = (a0 as *const u32).read_unaligned() as u64;
            d_var2 = l0 as f64;
            let a1 = DAT_00536f21.wrapping_add(
                (pbvar4.add(1).read() as u32)
                    .wrapping_sub(DAT_00536efd)
                    .wrapping_mul(4),
            );
            let l1 = (a1 as *const u32).read_unaligned() as u64;
            let d_var1a = l1 as f64;
            let a2 = DAT_00536f21.wrapping_add(
                (pbvar4.add(2).read() as u32)
                    .wrapping_sub(DAT_00536efe)
                    .wrapping_mul(4),
            );
            let l2 = (a2 as *const u32).read_unaligned() as u64;
            let d_var1 = (l2 as f64) + d_var1a + d_var2;

            let g = f64::from_bits(((_DAT_00536f0d as u64) << 32) | (_DAT_00536f09 as u64));
            d_var2 = g;

            if d_var1 < g {
                let v = ((param_1 as u32).wrapping_add(6) as *const u16).read_unaligned();
                _DAT_00536f2d = v as u32;
                d_var2 = d_var1;
            }
        }

        let bits = d_var2.to_bits();
        _DAT_00536f0d = (bits >> 32) as u32;
        _DAT_00536f09 = bits as u32;
    }
}
