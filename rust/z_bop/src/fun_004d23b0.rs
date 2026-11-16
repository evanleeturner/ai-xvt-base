// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69a89: i32;
    static mut DAT_00b63f91: i32;
    static mut DAT_00b63f95: i32;
    static mut DAT_00b63f9d: i32;
    static mut DAT_00b63fa5: i32;
    static mut DAT_00b63f99: i32;
    static mut DAT_00b63fa1: i32;
    static mut DAT_00b63fa9: i32;
    static mut DAT_00b63fad: i32;
    static mut DAT_00b63fb1: i32;
    static mut DAT_00b641b1: i32;
    static mut DAT_00b641b5: i32;
    static mut DAT_00b641c5: i32;
    static mut DAT_00b641b9: i32;
    static mut DAT_00b641bd: i32;
    static mut DAT_00b641c1: i32;
    static mut DAT_0052b9f0: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

unsafe extern "C" {
    #[link_name = "sprintf"]
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d23b0(param_1: i32) -> u32 {
    let mut local_100 = [0i8; 256];
    let mut u_var3: u32 = 0;
    // SAFETY: all globals and callees are the game's own memory/functions;
    // slot index is bounded below 0x28 so the strided pointers stay in the table.
    unsafe {
        if DAT_00b69a89 != 0 {
            let mut pi: *mut i32 = &raw mut DAT_00b63f91;
            loop {
                if pi.read_unaligned() == param_1 {
                    return u_var3;
                }
                pi = pi.add(0x8e);
                u_var3 = u_var3.wrapping_add(1);
                if u_var3 >= DAT_00b69a89 as u32 {
                    break;
                }
            }
        }
        if DAT_00b69a89 as u32 == u_var3 && u_var3 < 0x28 {
            let off = u_var3.wrapping_mul(0x8e) as usize;
            (&raw mut DAT_00b63f91).add(off).write_unaligned(param_1);
            (&raw mut DAT_00b63f95).add(off).write_unaligned(0x7f);
            (&raw mut DAT_00b63f9d).add(off).write_unaligned(0x7f);
            (&raw mut DAT_00b63fa5).add(off).write_unaligned(0x7f);
            (&raw mut DAT_00b63f99).add(off).write_unaligned(0x7f);
            (&raw mut DAT_00b63fa1).add(off).write_unaligned(0x7f);
            (&raw mut DAT_00b63fa9).add(off).write_unaligned(0x7f);
            (&raw mut DAT_00b63fad).add(off).write_unaligned(0);
            (&raw mut DAT_00b63fb1).add(off).write_unaligned(0x39);
            (&raw mut DAT_00b641b1).add(off).write_unaligned(1);
            let d = GetTickCount();
            (&raw mut DAT_00b641b5).add(off).write_unaligned(d as i32);
            let d = GetTickCount();
            (&raw mut DAT_00b641c5).add(off).write_unaligned(d as i32);
            (&raw mut DAT_00b641b9).add(off).write_unaligned(0);
            (&raw mut DAT_00b641bd).add(off).write_unaligned(0);
            (&raw mut DAT_00b641c1).add(off).write_unaligned(0);
            DAT_00b69a89 = DAT_00b69a89.wrapping_add(1);
            _sprintf(
                local_100.as_mut_ptr(),
                (&raw const DAT_0052b9f0).cast::<i8>(),
                param_1,
            );
        }
    }
    u_var3
}
