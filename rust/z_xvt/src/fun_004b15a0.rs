// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00656c90: i32;
    static mut DAT_00655ab4: i32;
    static mut DAT_00655ab0: i32;
    static mut DAT_00655ab2: i32;
    static mut DAT_0071f2e2: i32;
    static mut DAT_006d4a4e: i32;
    static mut DAT_00703c80: unsafe extern "C" fn();
    static mut DAT_0071b730: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00655ab6: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048ed00(a: i16, b: i16, c: i16, d: i16);
    fn FUN_0048ed70(a: u32);
    fn FUN_0048ece0(a: u16, b: u16);
    fn FUN_0040ccc0(a: u16, b: u32, c: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b15a0(mut param_1: u16, param_2: u16, param_3: u32) {
    // SAFETY: all globals and callees are the game's own memory; the
    // pointer chains are dereferenced exactly as the original does.
    unsafe {
        let mut u_var3: u32 = param_1 as u32;
        let p_6c90 = (&raw mut DAT_00656c90)
            .cast::<u8>()
            .byte_add(u_var3.wrapping_mul(2) as usize);
        if p_6c90.cast::<u16>().read_unaligned() != param_2 {
            p_6c90.cast::<u16>().write_unaligned(param_2);
            let off = u_var3.wrapping_mul(0xc) as usize;
            let p_ab0 = (&raw mut DAT_00655ab0).cast::<u8>().byte_add(off);
            let p_ab2 = (&raw mut DAT_00655ab2).cast::<u8>().byte_add(off);
            let p_ab4 = (&raw mut DAT_00655ab4).cast::<u8>().byte_add(off);
            let p_ab6 = (&raw mut DAT_00655ab6).cast::<u8>().byte_add(off);
            let u_var2: u16 = p_ab4.cast::<u16>().read_unaligned();
            let a = p_ab0.cast::<i16>().read_unaligned();
            let b = p_ab2.cast::<i16>().read_unaligned();
            let c = DAT_0071f2e2
                .wrapping_mul(u_var2 as i32)
                .wrapping_add(a as i32)
                .wrapping_add(2) as i16;
            let d = ((DAT_006d4a4e as u16) as i32).wrapping_add(b as i32) as i16;
            FUN_0048ed00(a, b, c, d);
            (DAT_00703c80)();
            if 0x78 < param_1 {
                let q = (param_1.wrapping_sub(1) as i32) / 0x78;
                param_1 = (param_1 as i32).wrapping_add(q.wrapping_mul(-0x78)) as u16;
            }
            let set1 = param_1 == 0x3d
                || param_1 == 0x3a
                || param_1 == 0x4e
                || param_1 == 0x3e
                || param_1 == 0x4f;
            if param_2 < 0x15 && set1 {
                u_var3 = 0x4a;
            } else if param_2 < 0x33 && set1 {
                u_var3 = 0x4e;
            } else if (param_1 == 0x19 || param_1 == 0x18) && {
                let a1 =
                    ((&raw mut DAT_0071b730) as i32).wrapping_add(DAT_0071ecf4.wrapping_mul(0x5bd));
                let v1 = (a1 as usize as *const i32).read_unaligned();
                let a2 = v1
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_006d4988);
                let v2 = (a2 as usize as *const i32).read_unaligned();
                let a3 = v2.wrapping_add(0xa7);
                let v3 = (a3 as usize as *const i32).read_unaligned();
                ((v3.wrapping_add(0xbf)) as usize as *const i16).read_unaligned() == 0
            } {
                u_var3 = 0x52;
            } else {
                u_var3 = p_ab6.cast::<u16>().read_unaligned() as u32;
            }
            FUN_0048ed70(u_var3);
            FUN_0048ece0(
                p_ab0.cast::<u16>().read_unaligned(),
                p_ab2.cast::<u16>().read_unaligned(),
            );
            FUN_0040ccc0(param_2, u_var2 as u32, param_3 & 0xffff);
        }
    }
}
