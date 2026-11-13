// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e54: i32;
    static mut DAT_006d9d56: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_006d9d50: i32;
    static mut DAT_0052a5a0: i32;
    static mut DAT_006d9d70: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b250(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00491c50(param_1: u32) {
    // SAFETY: all globals and pointers are the game's own memory, accessed
    // at the same offsets and in the same order as the original.
    unsafe {
        let obj = DAT_00732e54 as u32;
        let flags_base = DAT_006d9d56 as u32;
        let tbl = DAT_006d4988 as u32;
        let mut u_var5: u16 = ((obj.wrapping_add(0x6a)) as *const u16).read_unaligned();
        let s_var2: i16;
        if u_var5 == 0xffff {
            if (((flags_base.wrapping_add(0x35)) as *const u8).read() & 1) != 0 {
                let a = tbl
                    .wrapping_add(param_1.wrapping_mul(0x23))
                    .wrapping_add(0x10);
                s_var2 = ((a as *const i16).read_unaligned()).wrapping_sub(0x4000);
            } else {
                let a = tbl
                    .wrapping_add(param_1.wrapping_mul(0x23))
                    .wrapping_add(0x10);
                s_var2 = ((a as *const i16).read_unaligned()).wrapping_add(0x4000);
            }
        } else if (((flags_base.wrapping_add(0x35)) as *const u8).read() & 1) != 0 {
            let a = tbl
                .wrapping_add((u_var5 as u32).wrapping_mul(0x23))
                .wrapping_add(0x10);
            s_var2 = ((a as *const i16).read_unaligned()).wrapping_sub(0x4000);
        } else {
            let a = tbl
                .wrapping_add(param_1.wrapping_mul(0x23))
                .wrapping_add(0x10);
            s_var2 = ((a as *const i16).read_unaligned()).wrapping_add(0x4000);
        }
        ((DAT_006d9d56 as u32).wrapping_add(0x32) as *mut i16).write_unaligned(s_var2);
        let u_var3 = FUN_0042b250(DAT_00732e54);
        let entry = (DAT_006d4988 as u32).wrapping_add((DAT_006d9d50 as u32).wrapping_mul(0x23));
        let psvar1 = entry.wrapping_add(0x10) as *mut i16;
        let cur = psvar1.read_unaligned();
        let tgt = ((DAT_006d9d56 as u32).wrapping_add(0x32) as *const i16).read_unaligned();
        u_var5 = (cur as i32).wrapping_sub(tgt as i32) as u16;
        if 0x7fff < u_var5 {
            u_var5 = u_var5.wrapping_neg();
        }
        if u_var5 < 0x301 {
            let t = ((DAT_006d9d56 as u32).wrapping_add(0x32) as *const i16).read_unaligned();
            psvar1.write_unaligned(t);
            let p1 = ((((DAT_006d4988 as u32)
                .wrapping_add((DAT_006d9d50 as u32).wrapping_mul(0x23)))
            .wrapping_add(0x1f)) as *const u32)
                .read_unaligned();
            (p1.wrapping_add(0x90) as *mut u8).write(1);
            let i_var4 = ((((DAT_006d4988 as u32)
                .wrapping_add((DAT_006d9d50 as u32).wrapping_mul(0x23)))
            .wrapping_add(0x1f)) as *const u32)
                .read_unaligned();
            let v = (i_var4.wrapping_add(0x90) as *const u8).read();
            (i_var4.wrapping_add(0x89) as *mut u8).write(v);
            ((DAT_00732e54 as u32).wrapping_add(0xab) as *mut u8).write(3);
        } else {
            ((DAT_00732e54 as u32).wrapping_add(0xab) as *mut u8).write(2);
            let v = (((u_var3 >> 1) & 0x7fff).wrapping_add(0x8000)) as u16;
            ((DAT_00732e54 as u32).wrapping_add(0xac) as *mut u16).write_unaligned(v);
        }
        let src = (DAT_0052a5a0 as u32).wrapping_add((DAT_006d9d70 as u32).wrapping_mul(2));
        let w = (src as *const i16).read_unaligned();
        let r = (w as i32).wrapping_mul(0xec) as i16;
        ((DAT_006d9d56 as u32).wrapping_add(0x3a) as *mut i16).write_unaligned(r);
    }
}
