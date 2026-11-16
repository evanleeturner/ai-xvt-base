// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052c00c: i32;
    static mut DAT_0052c010: i32;
    static mut DAT_00665470: i32;
    static mut DAT_00b69e24: i32;
    static mut DAT_00b6a264: i32;
    static mut DAT_0052c024: i8;
    static mut DAT_0052c02c: i8;
    static mut DAT_0052c034: i8;
    static mut DAT_0052c03c: i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d9ba0(a: i32) -> i32;
    fn FUN_004de090(a: *mut i32, b: *mut i32) -> i32;
    fn FUN_004dcb10() -> u32;
    fn FUN_004dcb50();
    fn FUN_004d9c50();
    fn FUN_004dc2e0(a: i32) -> u8;
    fn FUN_004dc220() -> u8;
    fn FUN_004dc240() -> u8;
    fn FUN_004d6480(a: *mut i32, b: *mut i32);
    fn FUN_004d6460(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004d6550(a: *mut i32, b: i32, c: i32, d: u32);
    fn FUN_004d6b90(a: *mut i32, b: i32, c: i32) -> i32;
    fn FUN_004dca90(a: i32) -> u8;
    fn FUN_004dc1e0() -> u8;
    fn FUN_004dc200() -> u8;
    fn FUN_004b5570(a: *mut i8, b: i32, c: i32);
    fn FUN_004d67e0(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004d9b80(a: i32) -> u8;
    fn FUN_004d64c0(a: *mut i32, b: i32, c: i32);
    fn FUN_004dc300() -> u8;
    fn FUN_004dc1c0(a: i32);
    fn FUN_004dc2a0(a: i32) -> u8;
    fn FUN_004dc2c0(a: i32) -> u8;
    fn FUN_004dc1d0();
    fn FUN_004dc310();
}

#[inline(always)]
unsafe fn sdiv(a: i32, b: i32) -> i32 {
    if b == 0 || (a == i32::MIN && b == -1) {
        // SAFETY: the Win32 API raises the divide exception as the original does.
        unsafe { RaiseException(0xC0000094, 0, 0, core::ptr::null()) };
    }
    let q: i32;
    // SAFETY: the machine's own idiv, faulting as the original would.
    unsafe {
        core::arch::asm!(
            "cdq",
            "idiv {b:e}",
            b = in(reg) b,
            inout("eax") a => q,
            out("edx") _,
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d9d70(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: u32,
    param_7: i32,
) -> i32 {
    let mut local_38: i32 = 0;
    let mut local_20: i32 = 0;
    let mut local_30: [i32; 4] = [0; 4];
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: globals and callees are the game's own; param_1 points at a
    // four-int rectangle as in the original.
    unsafe {
        let p0 = param_1.read();
        let p1 = param_1.add(1).read();
        let p2 = param_1.add(2).read();
        let p3 = param_1.add(3).read();
        FUN_004d9ba0(param_7);
        FUN_004de090(&mut local_20, &mut local_38);
        let mut i_var5 = p2.wrapping_sub(p0);
        let local_14 = p3.wrapping_sub(p1).wrapping_add(i_var5.wrapping_mul(-2));
        let local_18 = param_3.wrapping_sub(param_4);
        let mut local_34 = sdiv(local_14, local_18);
        if local_34 < 1 {
            local_34 = 1;
        }
        let mut i_var6 = param_2;
        if i_var5 < p3.wrapping_sub(p1) {
            let u_var3 = FUN_004dcb10();
            if (u_var3 as u8) == 9 {
                FUN_004dcb50();
                FUN_004d9c50();
            }
            let local_1c = param_7.wrapping_add(1000);
            if FUN_004dc2e0(local_1c) == 0 {
                if FUN_004dc220() != 0 || FUN_004dc240() != 0 {
                    DAT_0052c00c = 0;
                    DAT_0052c010 = 0;
                }
                FUN_004d6480(local_30.as_mut_ptr(), param_1);
                let t = param_2.wrapping_mul(local_14);
                let mut i_var4 = local_30[1]
                    .wrapping_add(sdiv(t, local_18))
                    .wrapping_add(i_var5);
                FUN_004d6460(
                    local_10.as_mut_ptr(),
                    local_30[0],
                    i_var4,
                    local_30[2],
                    local_34.wrapping_add(i_var4),
                );
                local_30[1] = local_30[1].wrapping_add(i_var5);
                local_30[3] = local_30[3].wrapping_sub(i_var5);
                FUN_004d6550(local_30.as_mut_ptr(), 0, 0, param_6);
                i_var4 = FUN_004d6b90(local_30.as_mut_ptr(), local_20, local_38);
                if i_var4 != 0 && (FUN_004dc220() != 0 || FUN_004dc240() != 0) {
                    if local_38 < local_10[1] {
                        i_var6 = param_2.wrapping_sub(param_5);
                        if i_var6 < param_4 {
                            i_var6 = param_4;
                        }
                    } else if local_10[3] < local_38 {
                        i_var6 = param_5.wrapping_add(param_2);
                        if param_3 <= i_var6 {
                            i_var6 = param_3.wrapping_sub(1);
                        }
                    }
                }
                if DAT_00665470 == param_7 {
                    if FUN_004dca90(0x21) == 0 {
                        if FUN_004dca90(0x22) != 0 {
                            i_var6 = param_5.wrapping_add(param_2);
                            if param_3 <= i_var6 {
                                i_var6 = param_3.wrapping_sub(1);
                            }
                        }
                    } else {
                        i_var6 = param_2.wrapping_sub(param_5);
                        if i_var6 < param_4 {
                            i_var6 = param_4;
                        }
                    }
                }
                FUN_004d6460(local_30.as_mut_ptr(), p0, p1, p2, p1.wrapping_add(i_var5));
                i_var4 = FUN_004d6b90(local_30.as_mut_ptr(), local_20, local_38);
                if i_var4 == 0 || (FUN_004dc1e0() == 0 && FUN_004dc200() == 0) {
                    FUN_004b5570(&raw mut DAT_0052c034, p0, p1);
                } else {
                    FUN_004b5570(&raw mut DAT_0052c03c, p0, p1);
                    if DAT_0052c00c == 0 {
                        if param_4 < param_2 {
                            i_var6 = param_2.wrapping_sub(1);
                        }
                        if DAT_0052c010 == 0 {
                            DAT_0052c010 = 0xc;
                        } else {
                            DAT_0052c010 >>= 2;
                            if DAT_0052c010 == 0 {
                                DAT_0052c010 = 1;
                            }
                        }
                        DAT_0052c00c = DAT_0052c010;
                    } else {
                        DAT_0052c00c = DAT_0052c00c.wrapping_sub(1);
                    }
                }
                if DAT_00665470 == param_7 && FUN_004dca90(0x26) != 0 && param_4 < param_2 {
                    i_var6 = param_2.wrapping_sub(1);
                }
                FUN_004d6460(local_30.as_mut_ptr(), p0, p3.wrapping_sub(i_var5), p2, p3);
                i_var4 = FUN_004d6b90(local_30.as_mut_ptr(), local_20, local_38);
                if i_var4 == 0 || (FUN_004dc1e0() == 0 && FUN_004dc200() == 0) {
                    FUN_004b5570(&raw mut DAT_0052c024, p0, p3.wrapping_sub(i_var5));
                } else {
                    FUN_004b5570(&raw mut DAT_0052c02c, p0, p3.wrapping_sub(i_var5));
                    if DAT_0052c00c == 0 {
                        if param_2 < param_3.wrapping_sub(1) {
                            i_var6 = param_2.wrapping_add(1);
                        }
                        if DAT_0052c010 == 0 {
                            DAT_0052c010 = 0xc;
                        } else {
                            DAT_0052c010 >>= 2;
                            if DAT_0052c010 == 0 {
                                DAT_0052c010 = 1;
                            }
                        }
                        DAT_0052c00c = DAT_0052c010;
                    } else {
                        DAT_0052c00c = DAT_0052c00c.wrapping_sub(1);
                    }
                }
                if DAT_00665470 == param_7
                    && FUN_004dca90(0x28) != 0
                    && param_2 < param_3.wrapping_sub(1)
                {
                    i_var6 = param_2.wrapping_add(1);
                }
                FUN_004d67e0(local_10.as_mut_ptr(), 0, 0, 0xffff, 0);
                if FUN_004d9b80(param_7) != 0 {
                    FUN_004d64c0(local_10.as_mut_ptr(), 2, 2);
                    FUN_004d67e0(local_10.as_mut_ptr(), 0, 0, (DAT_00b69e24 as i16) as i32, 1);
                    FUN_004d64c0(local_10.as_mut_ptr(), -2, -2);
                }
                if FUN_004dc300() != 0
                    && FUN_004d6b90(local_10.as_mut_ptr(), local_20, local_38) != 0
                    && (FUN_004dc1e0() != 0 || FUN_004dc200() != 0)
                {
                    FUN_004dc1c0(local_1c);
                    return i_var6;
                }
            } else {
                DAT_0052c00c = 0;
                DAT_0052c010 = 0;
                if FUN_004dc2a0(local_1c) != 0 || FUN_004dc2c0(local_1c) != 0 {
                    FUN_004dc1d0();
                    FUN_004dc310();
                }
                FUN_004d6480(local_30.as_mut_ptr(), param_1);
                FUN_004d6550(local_30.as_mut_ptr(), 0, 0, param_6);
                FUN_004d6460(local_30.as_mut_ptr(), p0, p1, p2, p1.wrapping_add(i_var5));
                FUN_004b5570(&raw mut DAT_0052c034, p0, p1);
                FUN_004d6460(local_30.as_mut_ptr(), p0, p3.wrapping_sub(i_var5), p2, p3);
                FUN_004b5570(&raw mut DAT_0052c024, p0, p3.wrapping_sub(i_var5));
                let t = local_38
                    .wrapping_sub(i_var5)
                    .wrapping_sub(p1)
                    .wrapping_mul(local_18);
                i_var6 = sdiv(t, local_14);
                if i_var6 < param_4 {
                    i_var6 = param_4;
                }
                if param_3 <= i_var6 {
                    i_var6 = param_3.wrapping_sub(1);
                }
                let mut i_var4 = p1.wrapping_add(i_var5);
                if i_var4 <= local_38 {
                    i_var5 = p3.wrapping_sub(local_34).wrapping_sub(i_var5);
                    i_var4 = local_38;
                    if i_var5 <= local_38 {
                        i_var4 = i_var5.wrapping_sub(1);
                    }
                }
                FUN_004d6460(
                    local_30.as_mut_ptr(),
                    p0,
                    i_var4,
                    p2,
                    local_34.wrapping_add(i_var4),
                );
                FUN_004d67e0(local_30.as_mut_ptr(), 0, 0, 0xffff, 0);
                if FUN_004d9b80(param_7) != 0 {
                    FUN_004d64c0(local_30.as_mut_ptr(), 2, 2);
                    FUN_004d67e0(local_30.as_mut_ptr(), 0, 0, (DAT_00b6a264 as i16) as i32, 1);
                    FUN_004d64c0(local_30.as_mut_ptr(), -2, -2);
                }
            }
        }
        i_var6
    }
}
