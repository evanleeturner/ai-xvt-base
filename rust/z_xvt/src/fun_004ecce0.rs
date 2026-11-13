// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757cb4: i32;
    static mut DAT_00757ca4: i32;
    static mut DAT_00743dc2: i32;
    static mut DAT_00757cb0: i32;
    static mut DAT_007438d2: i32;
    static mut DAT_00757cac: i32;
    static mut DAT_005333fc: i32;
    static mut DAT_0053340c: i32;
    static mut DAT_007579d0: i32;
    static mut DAT_00757870: i32;
    static mut DAT_00757cee: i32;
    static mut DAT_00757680: i32;
    static mut DAT_007579f8: i32;
    static mut DAT_00757cf6: i32;
    static mut DAT_00757cfa: i32;
    static mut DAT_00743dc3: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ecce0(param_1: i32) -> u32 {
    // SAFETY: all globals and the tables addressed from them are the game's
    // own memory, read at the offsets the original uses.
    unsafe {
        let mut u_var2: u32;
        if param_1 == -1 {
            if DAT_00757cb4 == 0 {
                if DAT_00757ca4 != 0 {
                    let idx = DAT_00757ca4.wrapping_add(DAT_00757cb0.wrapping_mul(0x562));
                    return u32::from(
                        (&raw mut DAT_00743dc2)
                            .cast::<u8>()
                            .offset(idx as isize)
                            .read(),
                    );
                }
                return u32::from(
                    (&raw mut DAT_007438d2)
                        .cast::<u8>()
                        .offset(DAT_00757cb0.wrapping_mul(0x562) as isize)
                        .read(),
                );
            }
            if DAT_00757cac == 0 {
                return u32::from(
                    (&raw mut DAT_007438d2)
                        .cast::<u8>()
                        .offset(DAT_00757cb0.wrapping_mul(0x562) as isize)
                        .read(),
                );
            }
            let b_var1 = (&raw mut DAT_00743dc2)
                .cast::<u8>()
                .offset(DAT_00757cb0.wrapping_mul(0x562) as isize)
                .read();
            u_var2 = DAT_00757cac as u32;
            if b_var1 != 0 {
                if b_var1 < 3 {
                    return (&raw mut DAT_005333fc).offset(DAT_00757cac as isize).read() as u32;
                }
                if b_var1 == 3 {
                    return (&raw mut DAT_0053340c).offset(DAT_00757cac as isize).read() as u32;
                }
                return DAT_00757cac as u32;
            }
        } else {
            let mut local_4: i32 = 0;
            let mut i_var4: i32 = 0;
            let mut pi_var6: *mut i32 = &raw mut DAT_007579d0;
            let mut local_8: *mut i32 = &raw mut DAT_00757870;
            let key = (&raw mut DAT_00757cee)
                .cast::<u8>()
                .offset(param_1.wrapping_mul(0x2a) as isize)
                .cast::<i32>()
                .read_unaligned();
            loop {
                let mut i_var5: i32 = 0;
                if 0 < pi_var6.read() {
                    let mut pi_var3 = local_8;
                    loop {
                        if pi_var3.read() == key {
                            local_4 = (&raw mut DAT_00757680)
                                .offset(i_var5.wrapping_add(i_var4) as isize)
                                .read();
                            break;
                        }
                        pi_var3 = pi_var3.add(1);
                        i_var5 = i_var5.wrapping_add(1);
                        if !(i_var5 < pi_var6.read()) {
                            break;
                        }
                    }
                }
                i_var4 = i_var4.wrapping_add(8);
                local_8 = local_8.add(8);
                pi_var6 = pi_var6.add(1);
                if !(pi_var6 < (&raw mut DAT_007579f8)) {
                    break;
                }
            }
            u_var2 = (&raw mut DAT_00757cf6)
                .cast::<u8>()
                .offset(param_1.wrapping_mul(0x2a) as isize)
                .cast::<u32>()
                .read_unaligned();
            if u_var2 == 0 {
                u_var2 = (&raw mut DAT_00757cfa)
                    .cast::<u8>()
                    .offset(param_1.wrapping_mul(0x2a) as isize)
                    .cast::<u32>()
                    .read_unaligned();
                if u_var2 != 0xffff_ffff && u_var2 < 10 {
                    let idx = u_var2.wrapping_add(local_4.wrapping_mul(0x562) as u32);
                    return u32::from(
                        (&raw mut DAT_00743dc3)
                            .cast::<u8>()
                            .offset(idx as i32 as isize)
                            .read(),
                    );
                }
                u_var2 = u32::from(
                    (&raw mut DAT_007438d2)
                        .cast::<u8>()
                        .offset(local_4.wrapping_mul(0x562) as isize)
                        .read(),
                );
            }
        }
        u_var2
    }
}
