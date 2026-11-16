// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a00738: [u16; 3];
    static mut DAT_009ec5fe: u16;
    static mut DAT_005234c8: [u16; 1];
    static mut DAT_005234c6: [u16; 1];
    static mut DAT_009d6943: u8;
    static mut DAT_00520ee8: i32;
    static mut _DAT_00520eec: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08104: i32;
    static mut DAT_00a00870: [i32; 1];
    static mut DAT_00a00874: [i32; 1];
    static mut DAT_009a8078: u16;
    static mut DAT_009d694a: i16;
    static mut DAT_009d694e: i16;
    static mut DAT_009d6952: i16;
    static mut DAT_009d6944: i32;
    static mut DAT_009d8b74: u16;
    static mut DAT_009a8c2c: i32;
    static mut DAT_009ecc34: i32;
    static mut _DAT_00a07be0: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004adc40(a: u32) -> i32;
    fn FUN_004ae2a0(a: u32, b: i32) -> i32;
    fn FUN_0042c410(a: u32) -> i32;
    fn FUN_00450650(a: i32, b: i32);
    fn FUN_0042cc40();
    fn FUN_0042e4d0(a: i32, b: i32, c: i32);
    fn FUN_004ac760() -> i32;
    fn FUN_0042bd80(a: u16);
}

#[inline(always)]
unsafe fn idiv(n: i32, d: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide, faulting as the original does.
    unsafe {
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) d,
            inout("eax") n => q,
            out("edx") _,
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042c0a0() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers computed from game data are read and written where the
    // original does.
    unsafe {
        let mut local_s: [i16; 3] = [0; 3];
        let mut u_var12: u16 = 0;
        loop {
            let u_var5 = u_var12 as usize;
            let s_var9 = (DAT_00a00738[u_var5] as i32).wrapping_sub(DAT_009ec5fe as i32) as i16;
            DAT_00a00738[u_var5] = s_var9 as u16;
            if s_var9 < 0 {
                let mut u_var3 = (&raw mut DAT_005234c8)
                    .cast::<u16>()
                    .wrapping_add(DAT_009d6943 as usize)
                    .read();
                if u_var12 == 2 {
                    u_var3 = (&raw mut DAT_005234c6)
                        .cast::<u16>()
                        .wrapping_add(DAT_009d6943 as usize)
                        .read();
                }
                let q = idiv(s_var9 as i32, u_var3 as i32);
                let s_var11 = 1i16.wrapping_sub(q as i16);
                local_s[u_var5] = s_var11;
                DAT_00a00738[u_var5] = (u_var3 as u32)
                    .wrapping_mul(s_var11 as i32 as u32)
                    .wrapping_add(s_var9 as i32 as u32)
                    as u16;
            } else {
                local_s[u_var5] = 0;
            }
            u_var12 = u_var12.wrapping_add(1);
            if !(u_var12 < 3) {
                break;
            }
        }

        DAT_00520ee8 = (DAT_00520ee8 as u32).wrapping_sub(DAT_009ec5fe as u32) as i32;
        if DAT_00520ee8 < 0 {
            DAT_00520ee8 = (DAT_00520ee8 as u32).wrapping_add(0x1d) as i32;
            _DAT_00520eec = (_DAT_00520eec as u32).wrapping_add(1) as i32;
        }

        let mut local_c: u32 = 1;
        loop {
            let i_var8 = local_c.wrapping_mul(0x23) as i32;
            let base = DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var8);

            let mut ptr = (base as u32 as *const i32).read_unaligned();
            DAT_00a08104 = ((ptr as u32).wrapping_add(0xa9) as *const i32).read_unaligned();

            if _DAT_00520eec == 4 {
                _DAT_00520eec = 0;
            }
            ptr = (base as u32 as *const i32).read_unaligned();
            ((ptr as u32).wrapping_add(0x8a) as *mut u8).write(_DAT_00520eec as u8);
            if _DAT_00520eec == 3 {
                ptr = (base as u32 as *const i32).read_unaligned();
                ((ptr as u32).wrapping_add(0x8a) as *mut u8).write(1);
            }

            let b_var2 =
                (DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var8) as u32 as *const u8).read();
            let i_var6: i32 = if b_var2 < 0x49 {
                (&raw mut DAT_00a00870)
                    .cast::<i32>()
                    .wrapping_add(b_var2 as usize * 0x65)
                    .read()
            } else {
                FUN_004adc40(b_var2 as u32)
            };
            let mut i_var10: i32 = 0;
            let mut i_var13: i32 = 0;
            if 0 < i_var6 {
                loop {
                    let b = (DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var8) as u32 as *const u8)
                        .read();
                    let u_var5 = b as u32;
                    let i_var7: i32;
                    if b < 0x49 {
                        if i_var10 < 0 {
                            i_var7 = 0;
                        } else {
                            let mut idx = i_var10;
                            let cnt = (&raw mut DAT_00a00870)
                                .cast::<i32>()
                                .wrapping_add(u_var5 as usize * 0x65)
                                .read();
                            if cnt <= i_var10 {
                                idx = cnt.wrapping_sub(1);
                            }
                            i_var7 = (&raw mut DAT_00a00874)
                                .cast::<i32>()
                                .wrapping_add(
                                    (idx as u32).wrapping_add(u_var5.wrapping_mul(0x65)) as usize
                                )
                                .read();
                        }
                    } else {
                        i_var7 = FUN_004ae2a0(u_var5, i_var10);
                    }
                    let mut do_add = false;
                    let mut c_var4: i8 = 0;
                    if i_var7 == 0x11 {
                        c_var4 = local_s[0] as i8;
                        if 2 < DAT_009d6943 {
                            do_add = true;
                        }
                    } else if i_var7 == 0x12 {
                        c_var4 = local_s[2] as i8;
                        if 5 < DAT_009d6943 {
                            do_add = true;
                        }
                    } else if i_var7 == 0x13 {
                        c_var4 = local_s[1] as i8;
                        if 3 < DAT_009d6943 {
                            do_add = true;
                        }
                    }
                    if do_add {
                        let pc = i_var13.wrapping_add(0x30a).wrapping_add(DAT_00a08104) as u32
                            as *mut i8;
                        pc.write(pc.read().wrapping_add(c_var4));
                    }
                    i_var10 = i_var10.wrapping_add(1);
                    i_var13 = i_var13.wrapping_add(1);
                    if !(i_var10 < i_var6) {
                        break;
                    }
                }
            }

            u_var12 = local_c.wrapping_add(1) as u16;
            local_c = u_var12 as u32;
            if 0xc < u_var12 {
                u_var12 = 1;
                if DAT_009a8078 != 0xc {
                    u_var12 = DAT_009a8078.wrapping_add(1);
                }
                let r = FUN_0042c410(u_var12 as u32);
                if r != 0 {
                    DAT_009d694a = DAT_009d694a.wrapping_add(1);
                    DAT_009d694e = DAT_009d694e.wrapping_sub(1);
                    DAT_009a8078 = u_var12;
                    if u_var12 == 1 {
                        FUN_00450650(0xc6, DAT_009ecc34);
                        DAT_009d6952 = 0;
                        FUN_0042cc40();
                        while (DAT_009d8b74 & 0xff) as u8 != 0 || (DAT_009d8b74 >> 8) != 0 {
                            if (DAT_009d8b74 >> 8) == 0 {
                                DAT_009d8b74 = ((b';' as u16) << 8)
                                    | ((DAT_009d8b74 & 0xff) as u8).wrapping_sub(1) as u16;
                            } else {
                                DAT_009d8b74 = (DAT_009d8b74 & 0xff)
                                    | ((((DAT_009d8b74 >> 8) as u8).wrapping_sub(1) as u16) << 8);
                            }
                            DAT_009d6952 = DAT_009d6952.wrapping_add(10);
                            DAT_009d6944 = DAT_009d6944.wrapping_add(10);
                            if DAT_009d6944 % 100 == 0 {
                                FUN_0042e4d0(0x2a, -1, DAT_009ecc34);
                            }
                            FUN_0042cc40();
                            loop {
                                let v = FUN_004ac760();
                                DAT_009a8c2c = DAT_009a8c2c.wrapping_add(v);
                                if !(DAT_009a8c2c < 4) {
                                    break;
                                }
                            }
                            DAT_009a8c2c = 0;
                        }
                        _DAT_00a07be0 = DAT_009d6952;
                        FUN_00450650(199, DAT_009ecc34);
                        DAT_009d6943 = DAT_009d6943.wrapping_add(1);
                        FUN_0042bd80(DAT_009d6943 as u16);
                    }
                }
                return;
            }
        }
    }
}
