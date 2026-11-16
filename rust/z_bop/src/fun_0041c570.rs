// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fd4: i32;
    static mut DAT_009ec604: i32;
    static mut DAT_009a1fd8: i32;
    static mut DAT_00a081e8: i32;
    static mut DAT_009a1fd0: i32;
    static mut DAT_00a08298: i32;
    static mut DAT_00a07bdc: i32;
    static mut DAT_009d80c0: i32;
    static mut DAT_00a07c54: i32;
    static mut DAT_009d8c18: i32;
    static mut DAT_00a07c58: i32;
    static mut DAT_009d1150: i32;
    static mut DAT_009d77fc: u32;
    static mut DAT_009d6828: u32;
    static mut DAT_009cd260: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[inline(always)]
unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own idiv; faults on zero or overflow like the original.
    unsafe {
        core::arch::asm!(
            "cdq",
            "idiv {b:e}",
            b = in(reg) b,
            inout("eax") a => q,
            out("edx") _,
            options(nomem, nostack)
        );
    }
    q
}

#[inline(always)]
unsafe fn div_chk(a: i32, b: i32) -> i32 {
    // SAFETY: RaiseException is the OS call; the division follows as in the original.
    unsafe {
        if b == 0 || (b == -1 && a == i32::MIN) {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        idiv(a, b)
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041c570(param_1: i32) -> u32 {
    // SAFETY: globals and the OS call are the game's own.
    unsafe {
        let iv1 = DAT_009a1fd4.wrapping_sub(DAT_009ec604);
        let iv2 = DAT_009a1fd8.wrapping_sub(DAT_00a081e8);
        let iv3 = DAT_009a1fd0.wrapping_sub(DAT_00a08298);
        let iv4 = DAT_00a07bdc.wrapping_sub(DAT_009d80c0);
        let iv5 = DAT_00a07c54.wrapping_sub(DAT_009d8c18);
        let mut iv12 = DAT_009d80c0.wrapping_sub(DAT_009ec604);
        let iv6 = DAT_00a07c58.wrapping_sub(DAT_009d1150);
        let iv7: i32;
        let local_c: u32;
        let mut local_8: u32;
        if param_1 < iv12 {
            iv7 = param_1.wrapping_sub(iv12);
            local_c = iv4.wrapping_sub(iv1) as u32;
            let u9 = ((iv7 as u32) >> 16) as u16;
            if -1 < (local_c as i32) {
                return (u9 as u32) << 16;
            }
            if iv7 < (local_c as i32) {
                return (u9 as u32) << 16;
            }
            local_8 = 0u32.wrapping_sub((iv12 as u32).wrapping_add(param_1 as u32));
        } else if iv12.wrapping_neg() == param_1 || param_1.wrapping_neg() < iv12 {
            local_8 = 0u32.wrapping_sub((iv12 as u32).wrapping_add(param_1 as u32));
            iv7 = param_1.wrapping_sub(iv12);
            local_c = 0;
        } else {
            local_8 = 0u32.wrapping_sub((iv12 as u32).wrapping_add(param_1 as u32));
            local_c = iv4.wrapping_sub(iv1) as u32;
            if (local_c as i32) < 0 {
                return local_c & 0xffff0000;
            }
            if (local_c as i32) <= (local_8 as i32) {
                return local_c & 0xffff0000;
            }
            iv7 = param_1.wrapping_sub(iv12);
        }
        iv12 = DAT_009d8c18.wrapping_sub(DAT_00a081e8);
        let u9 = ((iv7 as u32) >> 16) as u16;
        let local_1c: i32;
        let mut local_14: i32;
        let iv13: i32;
        if param_1 < iv12 {
            local_1c = param_1.wrapping_sub(iv12);
            iv13 = iv5.wrapping_sub(iv2);
            if -1 < iv13 {
                return (u9 as u32) << 16;
            }
            if local_1c < iv13 {
                return (u9 as u32) << 16;
            }
            local_14 = 0u32.wrapping_sub((iv12 as u32).wrapping_add(param_1 as u32)) as i32;
        } else if iv12.wrapping_neg() == param_1 || param_1.wrapping_neg() < iv12 {
            iv13 = 0;
            local_1c = param_1.wrapping_sub(iv12);
            local_14 = 0u32.wrapping_sub((iv12 as u32).wrapping_add(param_1 as u32)) as i32;
        } else {
            local_14 = 0u32.wrapping_sub((iv12 as u32).wrapping_add(param_1 as u32)) as i32;
            iv13 = iv5.wrapping_sub(iv2);
            if iv13 < 0 {
                return (u9 as u32) << 16;
            }
            if iv13 <= local_14 {
                return (u9 as u32) << 16;
            }
            local_1c = param_1.wrapping_sub(iv12);
        }
        iv12 = DAT_009d1150.wrapping_sub(DAT_00a08298);
        let local_18: i32;
        let mut local_10: i32;
        let iv11: i32;
        if param_1 < iv12 {
            local_18 = param_1.wrapping_sub(iv12);
            iv11 = iv6.wrapping_sub(iv3);
            if -1 < iv11 {
                return (u9 as u32) << 16;
            }
            if local_18 < iv11 {
                return (u9 as u32) << 16;
            }
            local_10 = 0u32.wrapping_sub((iv12 as u32).wrapping_add(param_1 as u32)) as i32;
        } else if iv12.wrapping_neg() == param_1 || param_1.wrapping_neg() < iv12 {
            iv11 = 0;
            local_18 = param_1.wrapping_sub(iv12);
            local_10 = 0u32.wrapping_sub((iv12 as u32).wrapping_add(param_1 as u32)) as i32;
        } else {
            local_10 = 0u32.wrapping_sub((iv12 as u32).wrapping_add(param_1 as u32)) as i32;
            iv11 = iv6.wrapping_sub(iv3);
            if iv11 < 0 {
                return (u9 as u32) << 16;
            }
            if iv11 <= local_10 {
                return (u9 as u32) << 16;
            }
            local_18 = param_1.wrapping_sub(iv12);
        }
        local_14 = ((local_14 as u32) << 8) as i32;
        local_10 = ((local_10 as u32) << 8) as i32;
        let mut u10: u32;
        let mut u8v: u32;
        if local_c == 0 {
            let iv12b = DAT_00a07bdc.wrapping_sub(DAT_009a1fd4);
            let ivd = iv4.wrapping_sub(iv1);
            if param_1 < iv12b {
                u10 = 0;
                local_8 = div_chk(((iv7 as u32) << 8) as i32, ivd) as u32;
            } else if iv12b.wrapping_neg() == param_1 || param_1.wrapping_neg() < iv12b {
                local_8 = 0xff;
                u10 = 0;
            } else {
                u10 = 0;
                local_8 = div_chk((local_8 << 8) as i32, ivd) as u32;
            }
        } else {
            u8v = idiv(((iv7 as u32) << 8) as i32, local_c as i32) as u32;
            local_8 = idiv((local_8 << 8) as i32, local_c as i32) as u32;
            u10 = u8v;
            if -1 < (local_c as i32) {
                u10 = local_8;
                local_8 = u8v;
            }
        }
        'lab: {
            if iv13 == 0 {
                let iv4b = DAT_00a07c54.wrapping_sub(DAT_009a1fd8);
                let ivd = iv5.wrapping_sub(iv2);
                if param_1 < iv4b {
                    u8v = div_chk(((local_1c as u32) << 8) as i32, ivd) as u32;
                } else if iv4b.wrapping_neg() == param_1 || param_1.wrapping_neg() < iv4b {
                    u8v = 0xff;
                } else {
                    u8v = div_chk(local_14, ivd) as u32;
                }
                let u9b = (u8v >> 16) as u16;
                if (local_8 as i32) < 0 {
                    return (u9b as u32) << 16;
                }
                if (u10 as i32) < 0 {
                    u10 = 0;
                }
                if (u8v as i32) < (u10 as i32) {
                    return (u9b as u32) << 16;
                }
            } else {
                u8v = idiv(((local_1c as u32) << 8) as i32, iv13) as u32;
                let u9b = (u8v >> 16) as u16;
                if -1 < iv13 {
                    if (u8v as i32) < (u10 as i32) {
                        return (u9b as u32) << 16;
                    }
                    if (u8v as i32) < (local_8 as i32) {
                        local_8 = u8v;
                    }
                    u8v = idiv(local_14, iv13) as u32;
                    if (local_8 as i32) < (u8v as i32) {
                        return u8v & 0xffff0000;
                    }
                    if (u10 as i32) < (u8v as i32) {
                        u10 = u8v;
                    }
                    break 'lab;
                }
                if (local_8 as i32) < (u8v as i32) {
                    return (u9b as u32) << 16;
                }
                if (u10 as i32) < (u8v as i32) {
                    u10 = u8v;
                }
                u8v = idiv(local_14, iv13) as u32;
                if (u8v as i32) < (u10 as i32) {
                    return u8v & 0xffff0000;
                }
            }
            if (u8v as i32) < (local_8 as i32) {
                local_8 = u8v;
            }
        }
        if iv11 == 0 {
            let iv4b = DAT_00a07c58.wrapping_sub(DAT_009a1fd0);
            let ivd = iv6.wrapping_sub(iv3);
            if param_1 < iv4b {
                u8v = div_chk(((local_18 as u32) << 8) as i32, ivd) as u32;
            } else if iv4b.wrapping_neg() == param_1 || param_1.wrapping_neg() < iv4b {
                u8v = 0xff;
            } else {
                u8v = div_chk(local_10, ivd) as u32;
            }
            let u9b = (u8v >> 16) as u16;
            if (local_8 as i32) < 0 {
                return (u9b as u32) << 16;
            }
            if (u10 as i32) < 0 {
                u10 = 0;
            }
            if (u8v as i32) < (u10 as i32) {
                return (u9b as u32) << 16;
            }
        } else {
            u8v = idiv(((local_18 as u32) << 8) as i32, iv11) as u32;
            let u9b = (u8v >> 16) as u16;
            if iv11 < 0 {
                if (local_8 as i32) < (u8v as i32) {
                    return (u9b as u32) << 16;
                }
                if (u10 as i32) < (u8v as i32) {
                    u10 = u8v;
                }
                u8v = idiv(local_10, iv11) as u32;
                if (u8v as i32) < (u10 as i32) {
                    return u8v & 0xffff0000;
                }
            } else {
                if (u8v as i32) < (u10 as i32) {
                    return (u9b as u32) << 16;
                }
                if (u8v as i32) < (local_8 as i32) {
                    local_8 = u8v;
                }
                u8v = idiv(local_10, iv11) as u32;
                if (local_8 as i32) < (u8v as i32) {
                    return u8v & 0xffff0000;
                }
                if (u10 as i32) < (u8v as i32) {
                    u10 = u8v;
                }
            }
        }
        if (u10 as i32) < 0x100 {
            u10 = ((u10 << 7) as u16) as u32;
            let mut prod = (u10 as i32 as i64).wrapping_mul(iv1 as i64);
            DAT_009d77fc = ((prod as u32) >> 15) | (((prod as u64) >> 32) as u32) << 17;
            prod = (u10 as i32 as i64).wrapping_mul(iv2 as i64);
            DAT_009d6828 = ((prod as u32) >> 15) | (((prod as u64) >> 32) as u32) << 17;
            prod = (u10 as i32 as i64).wrapping_mul(iv3 as i64);
            DAT_009cd260 = ((prod as u32) >> 15) | (((prod as u64) >> 32) as u32) << 17;
            return (((DAT_009cd260 >> 16) & 0xffff) << 16) | 0xffff;
        }
        u8v & 0xffff0000
    }
}
