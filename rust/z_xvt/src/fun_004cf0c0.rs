// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0078072e: i32;
    static mut DAT_00780726: u32;
    static mut DAT_0078072a: i32;
    static mut DAT_00780c7a: i32;
    static mut DAT_0078077a: i32;
    static mut DAT_0081b5be: u32;
    static mut DAT_00815cfa: u32;
    static mut DAT_00815ad6: u32;
    static mut DAT_00815ace: u32;
    static mut DAT_008050a6: u32;
    static mut DAT_008050ae: u32;
    static mut DAT_007810a6: u32;
    static mut DAT_007810aa: u32;
    static mut DAT_007810ae: u32;
    static mut DAT_007810b2: u8;
    static mut DAT_007810b3: u8;
    static mut DAT_007810b4: u8;
    static mut DAT_007810b5: u8;
    static mut DAT_007810b6: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cee90(param_1: u32) -> u32;
    fn FUN_004cb830(param_1: i32, param_2: *mut u32, param_3: i32);
}

#[inline(always)]
fn wp(base: *mut u32, idx: u32, words: u32) -> *mut u32 {
    (base as usize).wrapping_add((idx.wrapping_mul(words) as usize).wrapping_mul(4)) as *mut u32
}

#[inline(always)]
fn bp(base: *mut u8, idx: u32, stride: u32) -> *mut u8 {
    (base as usize).wrapping_add(idx.wrapping_mul(stride) as usize) as *mut u8
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004cf0c0() -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // indexed addresses are computed as the original does.
    unsafe {
        let mut local_8: [u32; 2] = [0; 2];
        if DAT_0078072e == 0 {
            let dvar2 = GetTickCount();
            local_8[0] = DAT_00780726;
            let uvar3 = FUN_004cee90(local_8[0]);
            local_8[0] = DAT_0081b5be;
            if local_8[0] <= uvar3 {
                return 0;
            }
            let dvar4 = wp(&raw mut DAT_00815cfa, uvar3, 0x8e).read_unaligned();
            if 45000 < dvar2.wrapping_sub(dvar4) {
                let t = GetTickCount();
                wp(&raw mut DAT_00815cfa, uvar3, 0x8e).write_unaligned(t);
                local_8[0] = DAT_008050ae;
                if local_8[0] < 0x400 {
                    local_8[1] = DAT_008050a6;
                    local_8[0] = 0x46;
                    wp(&raw mut DAT_007810b6, local_8[1], 0x84).write_unaligned(local_8[0]);
                    local_8[0] = DAT_00780726;
                    wp(&raw mut DAT_007810a6, local_8[1], 0x84).write_unaligned(local_8[0]);
                    local_8[0] = 4;
                    wp(&raw mut DAT_007810aa, local_8[1], 0x84).write_unaligned(local_8[0]);
                    local_8[0] = 0;
                    wp(&raw mut DAT_007810ae, local_8[1], 0x84).write_unaligned(local_8[0]);
                    bp(&raw mut DAT_007810b2, local_8[1], 0x210).write(0);
                    bp(&raw mut DAT_007810b5, local_8[1], 0x210).write(0);
                    bp(&raw mut DAT_007810b3, local_8[1], 0x210).write(0);
                    let mut uvar5 = wp(&raw mut DAT_00815ad6, uvar3, 0x8e).read_unaligned();
                    uvar5 = uvar5.wrapping_add(1);
                    if 0x7f < uvar5 {
                        uvar5 = 0;
                    }
                    wp(&raw mut DAT_00815ad6, uvar3, 0x8e).write_unaligned(uvar5);
                    bp(&raw mut DAT_007810b4, local_8[1], 0x210).write(uvar5 as u8);
                    local_8[1] = local_8[1].wrapping_add(1);
                    DAT_008050a6 = local_8[1];
                    local_8[0] = DAT_008050ae;
                    local_8[0] = local_8[0].wrapping_add(1);
                    DAT_008050ae = local_8[0];
                    if 0x3ff < local_8[1] {
                        local_8[1] = 0;
                        DAT_008050a6 = local_8[1];
                    }
                }
            }
        } else {
            let mut pvar6: *mut i32 = &raw mut DAT_0078077a;
            let dvar2 = GetTickCount();
            loop {
                let mut ivar1 = pvar6.read_unaligned();
                if ivar1 != 0 {
                    local_8[1] = DAT_00780c7a as u32;
                    if local_8[1] != ivar1 as u32 {
                        let mut uvar5 = DAT_0078072a as u32;
                        if uvar5 != ivar1 as u32 {
                            local_8[1] = pvar6.add(1).read_unaligned() as u32;
                            if local_8[1] != 0 {
                                let uvar3 = FUN_004cee90(ivar1 as u32);
                                local_8[1] = DAT_0081b5be;
                                if uvar3 < local_8[1] {
                                    uvar5 = wp(&raw mut DAT_00815cfa, uvar3, 0x8e).read_unaligned();
                                    if 45000 < dvar2.wrapping_sub(uvar5) {
                                        let dvar4 = GetTickCount();
                                        wp(&raw mut DAT_00815cfa, uvar3, 0x8e)
                                            .write_unaligned(dvar4);
                                        local_8[1] = DAT_008050ae;
                                        if local_8[1] < 0x400 {
                                            local_8[0] = 0x5b;
                                            ivar1 = pvar6.read_unaligned();
                                            FUN_004cb830(ivar1, local_8.as_mut_ptr(), 4);
                                            local_8[1] = DAT_008050a6;
                                            local_8[0] = 0x47;
                                            wp(&raw mut DAT_007810b6, local_8[1], 0x84)
                                                .write_unaligned(local_8[0]);
                                            ivar1 = pvar6.read_unaligned();
                                            wp(&raw mut DAT_007810a6, local_8[1], 0x84)
                                                .write_unaligned(ivar1 as u32);
                                            local_8[0] = 4;
                                            wp(&raw mut DAT_007810aa, local_8[1], 0x84)
                                                .write_unaligned(local_8[0]);
                                            local_8[0] = 0;
                                            wp(&raw mut DAT_007810ae, local_8[1], 0x84)
                                                .write_unaligned(local_8[0]);
                                            bp(&raw mut DAT_007810b2, local_8[1], 0x210).write(0);
                                            bp(&raw mut DAT_007810b5, local_8[1], 0x210).write(0);
                                            bp(&raw mut DAT_007810b3, local_8[1], 0x210).write(1);
                                            uvar5 = wp(&raw mut DAT_00815ace, uvar3, 0x8e)
                                                .read_unaligned();
                                            uvar5 = uvar5.wrapping_add(1);
                                            if 0x7f < uvar5 {
                                                uvar5 = 0;
                                            }
                                            wp(&raw mut DAT_00815ace, uvar3, 0x8e)
                                                .write_unaligned(uvar5);
                                            bp(&raw mut DAT_007810b4, local_8[1], 0x210)
                                                .write(uvar5 as u8);
                                            local_8[1] = local_8[1].wrapping_add(1);
                                            DAT_008050a6 = local_8[1];
                                            local_8[0] = DAT_008050ae;
                                            local_8[0] = local_8[0].wrapping_add(1);
                                            DAT_008050ae = local_8[0];
                                            if 0x3ff < local_8[1] {
                                                local_8[1] = 0;
                                                DAT_008050a6 = local_8[1];
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                pvar6 = pvar6.byte_add(40);
                if !((pvar6 as usize) < ((&raw mut DAT_00780c7a) as usize)) {
                    break;
                }
            }
        }
        1
    }
}
