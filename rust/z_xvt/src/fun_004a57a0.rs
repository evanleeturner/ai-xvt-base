// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052aec8: [i32; 2];
    static mut DAT_0052aed0: u32;
    static mut DAT_0052aed4: u32;
    static mut DAT_0052aed8: u32;
    static mut DAT_0052aedc: u32;
    static mut DAT_0052aee0: u32;
    static mut DAT_0052aee4: u32;
    static mut DAT_0052aee8: i32;
    static mut DAT_0052aeec: i32;
    static mut DAT_0052aef0: i32;
    static mut DAT_0052aef4: u32;
    static mut DAT_00645b90: i32;
    static mut DAT_00645b98: i32;
    static mut DAT_00645b9c: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
    fn joyGetDevCapsA(a: u32, b: *mut u32, c: u32) -> u32;
    fn joyGetPosEx(a: u32, b: *mut u32) -> u32;
}

#[inline(always)]
unsafe fn udiv(n: u32, d: u32) -> u32 {
    let q: u32;
    // SAFETY: machine unsigned divide; faults on zero divisor like the original.
    unsafe {
        core::arch::asm!("div {d}", d = in(reg) d, inout("eax") n => q, inout("edx") 0u32 => _);
    }
    q
}

#[inline(always)]
unsafe fn setup_caps(c: &[u32; 101]) {
    // SAFETY: the globals are the game's own memory.
    unsafe {
        DAT_0052aef4 = (c[24] & 0x10) >> 4;
        DAT_0052aed0 = c[10].wrapping_sub(c[9]);
        DAT_0052aed4 = c[12].wrapping_sub(c[11]);
        DAT_0052aed8 = c[14].wrapping_sub(c[13]);
        DAT_0052aedc = (((DAT_0052aed0 as i32) >> 1) as u32).wrapping_sub(c[10]);
        DAT_0052aee0 = (((DAT_0052aed4 as i32) >> 1) as u32).wrapping_sub(c[12]);
        DAT_0052aee4 = (((DAT_0052aed8 as i32) >> 1) as u32).wrapping_sub(c[14]);
        DAT_0052aee8 = (DAT_0052aed0 as i32) / 0x14;
        DAT_0052aeec = (DAT_0052aed4 as i32) / 0x14;
        DAT_0052aef0 = (DAT_0052aed8 as i32) / 0x14;
        DAT_00645b98 = (((DAT_0052aed0 as i32) >> 1) as u32).wrapping_add(c[9]) as i32;
        DAT_00645b90 = (((DAT_0052aed8 as i32) >> 1) as u32).wrapping_add(c[13]) as i32;
        DAT_00645b9c = (((DAT_0052aed4 as i32) >> 1) as u32).wrapping_add(c[11]) as i32;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a57a0(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: *mut u32,
    param_5: *mut u32,
) {
    let mut local_1c8: [u32; 13] = [0; 13];
    let mut local_194: [u32; 101] = [0; 101];
    let u_joy_id: u32 = (param_1 != 0) as u32;

    // SAFETY: globals and the Windows joystick API are the game's own; the
    // out pointers are the caller's and written where the original writes.
    unsafe {
        let flag = (&raw mut DAT_0052aec8).cast::<i32>().add(u_joy_id as usize);
        if flag.read() == 0 {
            flag.write(1);
            local_194 = [0; 101];
            if joyGetDevCapsA(0, local_194.as_mut_ptr(), 0x194) == 0 {
                setup_caps(&local_194);
            } else if joyGetDevCapsA(1, local_194.as_mut_ptr(), 0x194) == 0 {
                setup_caps(&local_194);
            } else {
                DAT_0052aef4 = 0;
                DAT_0052aed0 = 1;
                DAT_0052aed4 = 1;
                DAT_0052aed8 = 1;
                DAT_0052aedc = 0;
                DAT_0052aee0 = 0;
                DAT_0052aee4 = 0;
                DAT_0052aee8 = 0;
                DAT_0052aeec = 0;
                DAT_0052aef0 = 0;
            }
        }
        param_5.write_unaligned(0);
        local_1c8 = [0; 13];
        local_1c8[0] = 0x34;
        local_1c8[1] = 0x4c7;
        if joyGetPosEx(u_joy_id, local_1c8.as_mut_ptr()) == 0 {
            let mut i = local_1c8[2].wrapping_sub(DAT_00645b98 as u32) as i32;
            if i < 0 {
                i = i.wrapping_neg();
            }
            if DAT_0052aee8 < i {
                if DAT_0052aed0 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                param_2.write_unaligned(udiv(
                    DAT_0052aedc.wrapping_add(local_1c8[2]).wrapping_mul(0xff),
                    DAT_0052aed0,
                ));
            } else {
                param_2.write_unaligned(0);
            }
            let mut i = local_1c8[3].wrapping_sub(DAT_00645b9c as u32) as i32;
            if i < 0 {
                i = i.wrapping_neg();
            }
            if DAT_0052aeec < i {
                if DAT_0052aed4 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                }
                param_3.write_unaligned(udiv(
                    DAT_0052aee0.wrapping_add(local_1c8[3]).wrapping_mul(0xff),
                    DAT_0052aed4,
                ));
            } else {
                param_3.write_unaligned(0);
            }
            let mut i = local_1c8[4].wrapping_sub(DAT_00645b90 as u32) as i32;
            if i < 0 {
                i = i.wrapping_neg();
            }
            if DAT_0052aef0 < i && 0 < (DAT_0052aed8 as i32) {
                param_4.write_unaligned(udiv(
                    DAT_0052aee4.wrapping_add(local_1c8[4]).wrapping_mul(0xff),
                    DAT_0052aed8,
                ));
            } else {
                param_4.write_unaligned(0);
            }
            param_5.write_unaligned(local_1c8[8] & 0xffff);
            if DAT_0052aef4 != 0 && local_1c8[10] != 0xffff {
                param_5.write_unaligned(
                    0x10000u32.wrapping_shl((local_1c8[10] / 9000) & 0x1f)
                        | (local_1c8[8] & 0xffff),
                );
            }
        } else {
            param_2.write_unaligned(32000);
            param_3.write_unaligned(32000);
            param_4.write_unaligned(32000);
        }
    }
}
