// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527f48: [i32; 2];
    static mut DAT_00527f50: u32;
    static mut DAT_00527f54: u32;
    static mut DAT_00527f58: u32;
    static mut DAT_00527f5c: u32;
    static mut DAT_00527f60: u32;
    static mut DAT_00527f64: u32;
    static mut DAT_00527f68: i32;
    static mut DAT_00527f6c: i32;
    static mut DAT_00527f70: i32;
    static mut DAT_00527f74: u32;
    static mut DAT_00622ca8: [u32; 2];
    static mut DAT_00622cb4: i32;
    static mut DAT_00622cb8: i32;
    static mut DAT_00622ca0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6440(param_1: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
    fn joyGetDevCapsA(a: u32, b: *mut core::ffi::c_void, c: u32) -> u32;
    fn joyGetPosEx(a: u32, b: *mut core::ffi::c_void) -> u32;
}

unsafe fn udiv(a: u32, b: u32) -> u32 {
    let q: u32;
    // SAFETY: the machine's own unsigned divide, faulting on zero as the original does.
    unsafe {
        core::arch::asm!("div {d}", d = in(reg) b, inout("eax") a => q, inout("edx") 0u32 => _);
    }
    q
}

unsafe fn setup(l: &[u32; 101]) {
    // SAFETY: the globals are the game's own memory.
    unsafe {
        DAT_00527f74 = (l[24] & 0x10) >> 4;
        DAT_00527f50 = l[10].wrapping_sub(l[9]);
        DAT_00527f54 = l[12].wrapping_sub(l[11]);
        DAT_00527f58 = l[14].wrapping_sub(l[13]);
        DAT_00527f5c = ((DAT_00527f50 as i32) >> 1) as u32;
        DAT_00527f5c = DAT_00527f5c.wrapping_sub(l[10]);
        DAT_00527f60 = (((DAT_00527f54 as i32) >> 1) as u32).wrapping_sub(l[12]);
        DAT_00527f64 = (((DAT_00527f58 as i32) >> 1) as u32).wrapping_sub(l[14]);
        DAT_00527f68 = (DAT_00527f50 as i32) / 0x14;
        DAT_00527f6c = (DAT_00527f54 as i32) / 0x14;
        DAT_00527f70 = (DAT_00527f58 as i32) / 0x14;
        DAT_00622cb4 = l[9].wrapping_add(((DAT_00527f50 as i32) >> 1) as u32) as i32;
        DAT_00622cb8 = l[11].wrapping_add(((DAT_00527f54 as i32) >> 1) as u32) as i32;
        DAT_00622ca0 = l[13].wrapping_add(((DAT_00527f58 as i32) >> 1) as u32) as i32;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004aaba0(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: *mut u32,
    param_5: *mut u32,
) {
    let mut local_1c8: [u32; 13] = [0; 13];
    let mut local_194: [u32; 101] = [0; 101];
    let u_var1: usize = (param_1 != 0) as usize;
    // SAFETY: globals and callees are the game's own; the out pointers are the
    // caller's, written where the original writes.
    unsafe {
        let flag = (&raw mut DAT_00527f48).cast::<i32>().add(u_var1);
        let joy = (&raw mut DAT_00622ca8).cast::<u32>().add(u_var1);
        if flag.read() == 0 {
            flag.write(1);
            local_194 = [0; 101];
            let u_var2 = FUN_004d6440(0);
            let m = joyGetDevCapsA(u_var2, local_194.as_mut_ptr().cast(), 0x194);
            if m == 0 {
                let u_var4 = FUN_004d6440(0);
                joy.write(u_var4);
                setup(&local_194);
            } else {
                let u_var2 = FUN_004d6440(1);
                let m = joyGetDevCapsA(u_var2, local_194.as_mut_ptr().cast(), 0x194);
                if m == 0 {
                    let u_var4 = FUN_004d6440(1);
                    joy.write(u_var4);
                    setup(&local_194);
                } else {
                    DAT_00527f74 = 0;
                    DAT_00527f50 = 1;
                    DAT_00527f54 = 1;
                    DAT_00527f58 = 1;
                    DAT_00527f5c = 0;
                    DAT_00527f60 = 0;
                    DAT_00527f64 = 0;
                    DAT_00527f68 = 0;
                    DAT_00527f6c = 0;
                    DAT_00527f70 = 0;
                }
            }
        }
        param_5.write(0);
        local_1c8 = [0; 13];
        local_1c8[0] = 0x34;
        local_1c8[1] = 0x4c7;
        let m = joyGetPosEx(joy.read(), local_1c8.as_mut_ptr().cast());
        if m == 0 {
            let mut i = local_1c8[2].wrapping_sub(DAT_00622cb4 as u32) as i32;
            if i < 0 {
                i = i.wrapping_neg();
            }
            if DAT_00527f68 < i {
                if DAT_00527f50 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                param_2.write(udiv(
                    local_1c8[2].wrapping_add(DAT_00527f5c).wrapping_mul(0xff),
                    DAT_00527f50,
                ));
            } else {
                param_2.write(0);
            }
            let mut i = local_1c8[3].wrapping_sub(DAT_00622cb8 as u32) as i32;
            if i < 0 {
                i = i.wrapping_neg();
            }
            if DAT_00527f6c < i {
                if DAT_00527f54 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                param_3.write(udiv(
                    local_1c8[3].wrapping_add(DAT_00527f60).wrapping_mul(0xff),
                    DAT_00527f54,
                ));
            } else {
                param_3.write(0);
            }
            let mut i = local_1c8[4].wrapping_sub(DAT_00622ca0 as u32) as i32;
            if i < 0 {
                i = i.wrapping_neg();
            }
            if DAT_00527f70 < i && 0 < (DAT_00527f58 as i32) {
                param_4.write(
                    local_1c8[4].wrapping_add(DAT_00527f64).wrapping_mul(0xff) / DAT_00527f58,
                );
            } else {
                param_4.write(0);
            }
            param_5.write(local_1c8[8] & 0xffff);
            if DAT_00527f74 != 0 && local_1c8[10] != 0xffff {
                param_5.write(
                    0x10000u32.wrapping_shl((local_1c8[10] / 9000) & 0x1f)
                        | (local_1c8[8] & 0xffff),
                );
            }
        } else {
            param_2.write(32000);
            param_3.write(32000);
            param_4.write(32000);
        }
    }
}
