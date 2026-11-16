// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00815ac6: i32;
    static mut DAT_00815aca: i32;
    static mut DAT_00815ad2: i32;
    static mut DAT_00815ada: i32;
    static mut DAT_00815ace: i32;
    static mut DAT_00815ad6: i32;
    static mut DAT_00815ade: i32;
    static mut DAT_00815ae2: i32;
    static mut DAT_00815ae6: i32;
    static mut DAT_00815ce6: i32;
    static mut DAT_00815cea: i32;
    static mut DAT_00815cfa: i32;
    static mut DAT_00815cee: i32;
    static mut DAT_00815cf2: i32;
    static mut DAT_00815cf6: i32;
    static mut DAT_0081b5be: i32;
    static DAT_005324f8: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

unsafe extern "C" {
    #[link_name = "sprintf"]
    fn c_sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cee90(param_1: i32) -> u32 {
    let mut u_var3: u32 = 0;
    let mut local_100 = [0i8; 256];
    // SAFETY: the globals, the format string and the callees are the game's
    // own; the table is 0x28 records of 0x8e ints, indexed below 0x28.
    unsafe {
        if DAT_0081b5be != 0 {
            let mut pi_var1: *const i32 = &raw mut DAT_00815ac6;
            loop {
                if pi_var1.read_unaligned() == param_1 {
                    return u_var3;
                }
                pi_var1 = pi_var1.wrapping_add(0x8e);
                u_var3 = u_var3.wrapping_add(1);
                if !(u_var3 < DAT_0081b5be as u32) {
                    break;
                }
            }
        }

        if DAT_0081b5be as u32 == u_var3 && u_var3 < 0x28 {
            let off = (u_var3 as usize).wrapping_mul(0x8e);
            (&raw mut DAT_00815ac6)
                .wrapping_add(off)
                .write_unaligned(param_1);
            (&raw mut DAT_00815aca)
                .wrapping_add(off)
                .write_unaligned(0x7f);
            (&raw mut DAT_00815ad2)
                .wrapping_add(off)
                .write_unaligned(0x7f);
            (&raw mut DAT_00815ada)
                .wrapping_add(off)
                .write_unaligned(0x7f);
            (&raw mut DAT_00815ace)
                .wrapping_add(off)
                .write_unaligned(0x7f);
            (&raw mut DAT_00815ad6)
                .wrapping_add(off)
                .write_unaligned(0x7f);
            (&raw mut DAT_00815ade)
                .wrapping_add(off)
                .write_unaligned(0x7f);
            (&raw mut DAT_00815ae2).wrapping_add(off).write_unaligned(0);
            (&raw mut DAT_00815ae6)
                .wrapping_add(off)
                .write_unaligned(0x39);
            (&raw mut DAT_00815ce6).wrapping_add(off).write_unaligned(1);
            let d = GetTickCount();
            (&raw mut DAT_00815cea)
                .wrapping_add(off)
                .write_unaligned(d as i32);
            let d = GetTickCount();
            (&raw mut DAT_00815cfa)
                .wrapping_add(off)
                .write_unaligned(d as i32);
            (&raw mut DAT_00815cee).wrapping_add(off).write_unaligned(0);
            (&raw mut DAT_00815cf2).wrapping_add(off).write_unaligned(0);
            (&raw mut DAT_00815cf6).wrapping_add(off).write_unaligned(0);
            DAT_0081b5be = DAT_0081b5be.wrapping_add(1);
            c_sprintf(
                local_100.as_mut_ptr(),
                (&raw const DAT_005324f8).cast::<i8>(),
                param_1,
            );
        }
    }
    u_var3
}
