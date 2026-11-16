// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523410: i32;
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009e9680: [u8; 0];
    static mut DAT_009e9684: [u8; 0];
    static mut DAT_009e9685: [u8; 0];
    static mut DAT_009e967a: [u8; 0];
    static mut DAT_009e9a8a: [u8; 0];
    static mut DAT_009e9a84: [u8; 0];
    static mut DAT_009e9a80: [u8; 0];
    static mut DAT_009e9a67: [u8; 0];
    static mut DAT_009e9a73: [u8; 0];
    static mut DAT_009e9a75: [u8; 0];
    static mut DAT_009d8ccc: [i8; 0];
    static mut DAT_009d8ccd: [i8; 0];
    static mut DAT_009d8cce: [i8; 0];
    static mut DAT_009d8ccf: [i8; 0];
    static mut DAT_009d6940: [u8; 2];
    static mut DAT_00a07be0: u16;
    static mut DAT_00a07be2: u16;
    static mut DAT_009ecc34: *mut u8;
    static mut DAT_009a1ed0: [i8; 0];
    static mut DAT_009ec4d0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004525a0(param_1: u32, param_2: i16, param_3: *mut i8);
    fn FUN_00451bf0(param_1: i32, param_2: *mut core::ffi::c_void);
    fn FUN_00450650(param_1: i32, param_2: i32);
    fn FUN_00451560();
    fn FUN_00427720(param_1: i32, param_2: i32);
    fn FUN_0042e4d0(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_0046f720(param_1: *mut u8) -> *mut u8;
}

unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u8).read() }
}
unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u32).read_unaligned() }
}
unsafe fn wr8(a: u32, v: u8) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as usize as *mut u8).write(v) }
}
unsafe fn wr16(a: u32, v: u16) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00485900(param_1: i32, param_2: *mut u8) {
    // SAFETY: all globals and callees are the game's own; the pointers and
    // addresses are used with the offsets the original uses.
    unsafe {
        let p1 = param_1 as u32;
        let idx = (param_2 as usize as u32).wrapping_mul(0x5bd);
        if DAT_00523410 != 0 {
            return;
        }
        if (rd8(p1.wrapping_add(0x1e3)) & 0x80) == 0 {
            let mut u_var8: u32 = DAT_00a07cec as u32;
            let mut u_var10: u32 = 0xffff;
            let mut u_var5: u32 = 0xffff;
            if (u_var8 as i32) < DAT_009a1fe0 {
                let tmp_i =
                    rd32((&raw mut DAT_009e9670 as *mut u8 as usize as u32).wrapping_add(idx));
                let i_var9 = (rd8(tmp_i
                    .wrapping_mul(0x23)
                    .wrapping_add(0x16)
                    .wrapping_add(DAT_009a1fe8 as u32)) as u32)
                    .wrapping_mul(0x566) as usize;
                loop {
                    if *(&raw mut DAT_009d8ccd as *mut i8).add(i_var9) != 0 {
                        let i_var7 = u_var8.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8 as u32);
                        if rd8(i_var7.wrapping_add(3)) != 0
                            && *(&raw mut DAT_009d8ccc as *mut i8).add(i_var9)
                                == rd8(i_var7.wrapping_add(0x16)) as i8
                        {
                            u_var5 = u_var8;
                        }
                    }
                    if *(&raw mut DAT_009d8ccf as *mut i8).add(i_var9) != 0 {
                        let i_var7 = u_var8.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8 as u32);
                        if rd8(i_var7.wrapping_add(3)) != 0
                            && *(&raw mut DAT_009d8cce as *mut i8).add(i_var9)
                                == rd8(i_var7.wrapping_add(0x16)) as i8
                        {
                            u_var10 = u_var8;
                        }
                    }
                    u_var8 = ((u_var8 as i16).wrapping_add(1)) as u16 as u32;
                    if !((u_var8 as i32) < DAT_009a1fe0) {
                        break;
                    }
                }
            }
            if (u_var5 as i16) != -1 {
                if (u_var10 as i16) != -1 {
                    FUN_004525a0(u_var5, 0, &raw mut DAT_009a1ed0 as *mut i8);
                    FUN_00451bf0(0, &raw mut DAT_009a1ed0 as *mut core::ffi::c_void);
                    FUN_004525a0(u_var10, 0, &raw mut DAT_009ec4d0 as *mut i8);
                    FUN_00451bf0(1, &raw mut DAT_009ec4d0 as *mut core::ffi::c_void);
                    FUN_00450650(0x6f, param_2 as usize as i32);
                    return;
                }
                if (u_var5 as i16) != -1 {
                    FUN_004525a0(u_var5, 0, &raw mut DAT_009a1ed0 as *mut i8);
                    FUN_00451bf0(0, &raw mut DAT_009a1ed0 as *mut core::ffi::c_void);
                    FUN_00450650(0x6e, param_2 as usize as i32);
                    return;
                }
            }
            if (u_var10 as i16) != -1 {
                FUN_004525a0(u_var10, 0, &raw mut DAT_009a1ed0 as *mut i8);
                FUN_00451bf0(0, &raw mut DAT_009a1ed0 as *mut core::ffi::c_void);
                FUN_00450650(0x6e, param_2 as usize as i32);
            }
        } else {
            if (*(&raw mut DAT_009d6940 as *mut u8).add(1)) != 0 {
                *(&raw mut DAT_009d6940 as *mut u8) = 1;
                *(&raw mut DAT_009e9680 as *mut u8).add(idx as usize) = 2;
                return;
            }
            if (rd8(p1.wrapping_add(0x1e5)) & 0x80) == 0 {
                DAT_00a07be0 = 99;
                DAT_00a07be2 = 0x57;
                FUN_00450650(0x56, param_2 as usize as i32);
                return;
            }
            let mut b_var3: u8 = 0;
            let mut u_var6: u16 = DAT_00a07cec;
            while (u_var6 as i16 as i32) < DAT_009a1fe0 {
                let i_var9 = (u_var6 as i16 as i32 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_009a1fe8 as u32);
                let c_var1 = rd8(i_var9.wrapping_add(3)) as i8;
                if c_var1 == 0x33 || c_var1 == 0x5c {
                    let i_var9 = rd32(i_var9.wrapping_add(0x1f));
                    let tmp_s =
                        rd16((&raw mut DAT_009e967a as *mut u8 as usize as u32).wrapping_add(idx));
                    if tmp_s != rd8(i_var9.wrapping_add(0x88)) as u16 {
                        let tmp_i = rd32(i_var9.wrapping_add(0xa9));
                        let tmp_s = rd16(tmp_i.wrapping_add(0x1e5));
                        if tmp_s != 0 {
                            b_var3 = 1;
                        }
                    }
                }
                u_var6 = u_var6.wrapping_add(1);
            }
            if b_var3 != 0 {
                FUN_00450650(0x6d, param_2 as usize as i32);
                return;
            }
            if param_2 == DAT_009ecc34 {
                FUN_00451560();
            }
            FUN_00450650(0x6a, param_2 as usize as i32);
            let i9 = idx as usize;
            *(&raw mut DAT_009e9684 as *mut u8).add(i9) = 1;
            (&raw mut DAT_009e9685 as *mut u8)
                .add(i9)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_009e9a8a as *mut u8)
                .add(i9)
                .cast::<u16>()
                .write_unaligned(0);
            (&raw mut DAT_009e9a84 as *mut u8)
                .add(i9)
                .cast::<u16>()
                .write_unaligned(0);
            (&raw mut DAT_009e9a80 as *mut u8)
                .add(i9)
                .cast::<u16>()
                .write_unaligned(0);
            let tmp_i = (&raw mut DAT_009e9670 as *mut u8)
                .add(i9)
                .cast::<u32>()
                .read_unaligned();
            (&raw mut DAT_009e9a67 as *mut u8)
                .add(i9)
                .cast::<u16>()
                .write_unaligned(tmp_i as u16);
            FUN_00427720(0, param_2 as usize as i32);
            (&raw mut DAT_009e9a73 as *mut u8)
                .add(i9)
                .cast::<u16>()
                .write_unaligned(0);
            (&raw mut DAT_009e9a75 as *mut u8)
                .add(i9)
                .cast::<u16>()
                .write_unaligned(0);
            wr16(p1.wrapping_add(0xbd), 0);
            let tmp_j = (&raw mut DAT_009e9670 as *mut u8)
                .add(i9)
                .cast::<u32>()
                .read_unaligned();
            let c_var1 = rd8(tmp_j
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_009a1fe8 as u32)) as i8;
            if c_var1 == 1 || c_var1 == 4 {
                let b_var2 = rd8(p1.wrapping_add(0x29));
                if (b_var2 & 2) == 0 {
                    wr8(p1.wrapping_add(0x29), b_var2 | 2);
                    wr8(p1.wrapping_add(0x29), b_var2 | 3);
                    FUN_0042e4d0(0x55, -1, param_2 as usize as i32);
                }
            }
            if param_2 != DAT_009ecc34 {
                let puvar4 = FUN_0046f720(param_2);
                FUN_00451bf0(0, puvar4 as *mut core::ffi::c_void);
                FUN_00450650(0x71, DAT_009ecc34 as usize as i32);
                return;
            }
        }
    }
}
