// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00532ae4: i32;
    static mut DAT_00532ae8: i32;
    static mut DAT_0064f688: i32;
    static mut DAT_0081b844: i32;
    static mut DAT_0081bc74: i32;
    static mut DAT_00532b0c: [u8; 8];
    static mut DAT_00532b14: [u8; 8];
    static mut DAT_00532afc: [u8; 8];
    static mut DAT_00532b04: [u8; 8];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d8e20(a: i32);
    fn FUN_004ddc70(a: *mut i32, b: *mut i32);
    fn FUN_004dbe80() -> u32;
    fn FUN_004dbec0();
    fn FUN_004d8ee0();
    fn FUN_004db630(a: i32) -> u8;
    fn FUN_004db570() -> u8;
    fn FUN_004db590() -> u8;
    fn FUN_004dd000(a: *mut i32, b: *mut i32);
    fn FUN_004dcfd0(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004dd130(a: *mut i32, b: i32, c: i32, d: u32);
    fn FUN_004dd7d0(a: *mut i32, b: i32, c: i32) -> i32;
    fn FUN_004dbe00(a: i32) -> u8;
    fn FUN_004db530() -> u8;
    fn FUN_004db550() -> u8;
    fn FUN_004d0e60(a: *mut i8, b: i32, c: i32);
    fn FUN_004dd3e0(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004d8e10(a: i32) -> u8;
    fn FUN_004dd070(a: *mut i32, b: i32, c: i32);
    fn FUN_004db640() -> u8;
    fn FUN_004db510(a: i32);
    fn FUN_004db5f0(a: i32) -> u8;
    fn FUN_004db610(a: i32) -> u8;
    fn FUN_004db520();
    fn FUN_004db650();
}

unsafe fn sdiv(a: i32, b: i32) -> i32 {
    // SAFETY: the original raises the same exception before dividing; the
    // division itself is the machine's idiv.
    unsafe {
        if b == 0 || (a == i32::MIN && b == -1) {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        let q: i32;
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) b,
            inout("eax") a => q,
            out("edx") _,
        );
        q
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d9010(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: u32,
    param_7: i32,
) -> i32 {
    // SAFETY: param_1 points at the caller's rect; callees and globals are
    // the game's own.
    unsafe {
        let p0 = param_1.read();
        let p1 = param_1.add(1).read();
        let p2 = param_1.add(2).read();
        let p3 = param_1.add(3).read();
        let mut local_38: [i32; 2] = [0; 2];
        let mut local_34: i32;
        let mut local_30: [i32; 4] = [0; 4];
        let mut local_20: [i32; 2] = [0; 2];
        let local_1c: i32;
        let local_18: i32;
        let local_14: i32;
        let mut local_10: [i32; 4] = [0; 4];
        let mut i_var4: i32;
        let mut i_var5: i32;
        let mut i_var6: i32;

        FUN_004d8e20(param_7);
        FUN_004ddc70(local_20.as_mut_ptr(), local_38.as_mut_ptr());
        i_var5 = p2.wrapping_sub(p0);
        local_14 = p3.wrapping_sub(p1).wrapping_add(i_var5.wrapping_mul(-2));
        local_18 = param_3.wrapping_sub(param_4);
        local_34 = sdiv(local_14, local_18);
        if local_34 < 1 {
            local_34 = 1;
        }
        i_var6 = param_2;
        if i_var5 < p3.wrapping_sub(p1) {
            let u_var3 = FUN_004dbe80();
            if (u_var3 as u8 as i8) == 9 {
                FUN_004dbec0();
                FUN_004d8ee0();
            }
            local_1c = param_7.wrapping_add(1000);
            let b_var1 = FUN_004db630(local_1c);
            if b_var1 == 0 {
                if FUN_004db570() != 0 || FUN_004db590() != 0 {
                    DAT_00532ae4 = 0;
                    DAT_00532ae8 = 0;
                }
                FUN_004dd000(local_30.as_mut_ptr(), param_1);
                let t = param_2.wrapping_mul(local_14);
                i_var4 = local_30[1]
                    .wrapping_add(sdiv(t, local_18))
                    .wrapping_add(i_var5);
                FUN_004dcfd0(
                    local_10.as_mut_ptr(),
                    local_30[0],
                    i_var4,
                    local_30[2],
                    local_34.wrapping_add(i_var4),
                );
                local_30[1] = local_30[1].wrapping_add(i_var5);
                local_30[3] = local_30[3].wrapping_sub(i_var5);
                FUN_004dd130(local_30.as_mut_ptr(), 0, 0, param_6);
                i_var4 = FUN_004dd7d0(local_30.as_mut_ptr(), local_20[0], local_38[0]);
                if i_var4 != 0 && (FUN_004db570() != 0 || FUN_004db590() != 0) {
                    if local_38[0] < local_10[1] {
                        i_var6 = param_2.wrapping_sub(param_5);
                        if i_var6 < param_4 {
                            i_var6 = param_4;
                        }
                    } else if local_10[3] < local_38[0] {
                        i_var6 = param_5.wrapping_add(param_2);
                        if param_3 <= i_var6 {
                            i_var6 = param_3.wrapping_sub(1);
                        }
                    }
                }
                if param_7 == DAT_0064f688 {
                    if FUN_004dbe00(0x21) == 0 {
                        if FUN_004dbe00(0x22) != 0 {
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
                FUN_004dcfd0(local_30.as_mut_ptr(), p0, p1, p2, i_var5.wrapping_add(p1));
                i_var4 = FUN_004dd7d0(local_30.as_mut_ptr(), local_20[0], local_38[0]);
                if i_var4 == 0 || (FUN_004db530() == 0 && FUN_004db550() == 0) {
                    FUN_004d0e60((&raw mut DAT_00532b0c).cast::<i8>(), p0, p1);
                } else {
                    FUN_004d0e60((&raw mut DAT_00532b14).cast::<i8>(), p0, p1);
                    if DAT_00532ae4 == 0 {
                        if param_4 < param_2 {
                            i_var6 = param_2.wrapping_sub(1);
                        }
                        if DAT_00532ae8 == 0 {
                            DAT_00532ae8 = 0xc;
                        } else {
                            DAT_00532ae8 >>= 2;
                            if DAT_00532ae8 == 0 {
                                DAT_00532ae8 = 1;
                            }
                        }
                        DAT_00532ae4 = DAT_00532ae8;
                    } else {
                        DAT_00532ae4 = DAT_00532ae4.wrapping_sub(1);
                    }
                }
                if param_7 == DAT_0064f688 && FUN_004dbe00(0x26) != 0 && param_4 < param_2 {
                    i_var6 = param_2.wrapping_sub(1);
                }
                FUN_004dcfd0(local_30.as_mut_ptr(), p0, p3.wrapping_sub(i_var5), p2, p3);
                i_var4 = FUN_004dd7d0(local_30.as_mut_ptr(), local_20[0], local_38[0]);
                if i_var4 == 0 || (FUN_004db530() == 0 && FUN_004db550() == 0) {
                    FUN_004d0e60(
                        (&raw mut DAT_00532afc).cast::<i8>(),
                        p0,
                        p3.wrapping_sub(i_var5),
                    );
                } else {
                    FUN_004d0e60(
                        (&raw mut DAT_00532b04).cast::<i8>(),
                        p0,
                        p3.wrapping_sub(i_var5),
                    );
                    if DAT_00532ae4 == 0 {
                        if param_2 < param_3.wrapping_sub(1) {
                            i_var6 = param_2.wrapping_add(1);
                        }
                        if DAT_00532ae8 == 0 {
                            DAT_00532ae8 = 0xc;
                        } else {
                            DAT_00532ae8 >>= 2;
                            if DAT_00532ae8 == 0 {
                                DAT_00532ae8 = 1;
                            }
                        }
                        DAT_00532ae4 = DAT_00532ae8;
                    } else {
                        DAT_00532ae4 = DAT_00532ae4.wrapping_sub(1);
                    }
                }
                if param_7 == DAT_0064f688
                    && FUN_004dbe00(0x28) != 0
                    && param_2 < param_3.wrapping_sub(1)
                {
                    i_var6 = param_2.wrapping_add(1);
                }
                FUN_004dd3e0(local_10.as_mut_ptr(), 0, 0, 0xffff, 0);
                if FUN_004d8e10(param_7) != 0 {
                    FUN_004dd070(local_10.as_mut_ptr(), 2, 2);
                    FUN_004dd3e0(local_10.as_mut_ptr(), 0, 0, (DAT_0081b844 as i16) as i32, 1);
                    FUN_004dd070(local_10.as_mut_ptr(), -2, -2);
                }
                let b = FUN_004db640();
                if b != 0 {
                    i_var5 = FUN_004dd7d0(local_10.as_mut_ptr(), local_20[0], local_38[0]);
                    if i_var5 != 0 && (FUN_004db530() != 0 || FUN_004db550() != 0) {
                        FUN_004db510(local_1c);
                        return i_var6;
                    }
                }
            } else {
                DAT_00532ae4 = 0;
                DAT_00532ae8 = 0;
                if FUN_004db5f0(local_1c) != 0 || FUN_004db610(local_1c) != 0 {
                    FUN_004db520();
                    FUN_004db650();
                }
                FUN_004dd000(local_30.as_mut_ptr(), param_1);
                FUN_004dd130(local_30.as_mut_ptr(), 0, 0, param_6);
                FUN_004dcfd0(local_30.as_mut_ptr(), p0, p1, p2, i_var5.wrapping_add(p1));
                FUN_004d0e60((&raw mut DAT_00532b0c).cast::<i8>(), p0, p1);
                FUN_004dcfd0(local_30.as_mut_ptr(), p0, p3.wrapping_sub(i_var5), p2, p3);
                FUN_004d0e60(
                    (&raw mut DAT_00532afc).cast::<i8>(),
                    p0,
                    p3.wrapping_sub(i_var5),
                );
                let t = local_38[0]
                    .wrapping_sub(p1)
                    .wrapping_sub(i_var5)
                    .wrapping_mul(local_18);
                i_var6 = sdiv(t, local_14);
                if i_var6 < param_4 {
                    i_var6 = param_4;
                }
                if param_3 <= i_var6 {
                    i_var6 = param_3.wrapping_sub(1);
                }
                i_var4 = i_var5.wrapping_add(p1);
                if i_var4 <= local_38[0] {
                    i_var5 = p3.wrapping_sub(local_34).wrapping_sub(i_var5);
                    i_var4 = local_38[0];
                    if i_var5 <= local_38[0] {
                        i_var4 = i_var5.wrapping_sub(1);
                    }
                }
                FUN_004dcfd0(
                    local_30.as_mut_ptr(),
                    p0,
                    i_var4,
                    p2,
                    local_34.wrapping_add(i_var4),
                );
                FUN_004dd3e0(local_30.as_mut_ptr(), 0, 0, 0xffff, 0);
                if FUN_004d8e10(param_7) != 0 {
                    FUN_004dd070(local_30.as_mut_ptr(), 2, 2);
                    FUN_004dd3e0(local_30.as_mut_ptr(), 0, 0, (DAT_0081bc74 as i16) as i32, 1);
                    FUN_004dd070(local_30.as_mut_ptr(), -2, -2);
                }
            }
        }
        i_var6
    }
}
