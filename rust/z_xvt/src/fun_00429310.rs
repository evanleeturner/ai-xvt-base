// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e54: i32;
    static mut DAT_00732a40: [*mut i8; 0];
    static mut DAT_006d9d50: u16;
    static mut DAT_006d4988: u32;
    static mut DAT_006d9d56: [u8; 4];
    static mut DAT_0070b1f2: [u8; 0];
    static mut DAT_0070b1f4: [u8; 0];
    static mut DAT_0070b1da: [u8; 0];
    static mut DAT_00732a2c: u32;
    static mut DAT_00732a28: u32;
    static mut DAT_00732a24: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(param_1: i32) -> i32;
    fn FUN_00421c70(param_1: u32, param_2: u32);
    fn FUN_00491ad0();
}

unsafe fn sel_byte() -> u32 {
    // SAFETY: reads the game's own globals and the byte they point at.
    unsafe {
        let a = (DAT_006d9d50 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(0x16)
            .wrapping_add(DAT_006d4988);
        u32::from((a as usize as *const u8).read())
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00429310(param_1: u32) {
    // SAFETY: all globals, callees and pointers belong to the game; accesses
    // are made at the same offsets and in the same order as the original.
    unsafe {
        let i_var3 = FUN_0042b150(DAT_00732e54) as u32 as usize as *mut u8;
        let idx = u32::from(i_var3.byte_add(10).read()) as usize;
        let pc_var2 = (&raw mut DAT_00732a40).cast::<*mut i8>().add(idx).read();
        let c_var1 = pc_var2.read();
        'lab513: {
            if c_var1 == -1 {
                break 'lab513;
            }
            let v: u16;
            if c_var1 == -3 {
                let off = sel_byte().wrapping_mul(0x566) as usize;
                let p = (&raw mut DAT_0070b1f2).cast::<u8>().byte_add(off);
                if p.cast::<u16>().read_unaligned() == 0 {
                    v = 0x8000;
                } else {
                    v = 0x800c;
                }
            } else if c_var1 == -2 {
                let flag = ((DAT_00732e54 as u32 as usize + 0x1e5) as *const i8).read();
                let mut r: u16 = 0;
                let mut have = false;
                if flag != 0 {
                    let off = sel_byte().wrapping_mul(0x566) as usize;
                    let p = (&raw mut DAT_0070b1f2).cast::<u8>().byte_add(off);
                    if p.cast::<u16>().read_unaligned() != 0 {
                        r = 0x800c;
                        have = true;
                    }
                }
                if !have {
                    let off = sel_byte().wrapping_mul(0x566) as usize;
                    let p = (&raw mut DAT_0070b1f4).cast::<u8>().byte_add(off);
                    if p.cast::<u16>().read_unaligned() == 0 {
                        r = 0x8000;
                    } else {
                        r = 0x800d;
                    }
                }
                v = r;
            } else if c_var1 == -7 {
                v = 0x800c;
            } else {
                let b = u32::from(i_var3.byte_add(0xc).read());
                let off = sel_byte()
                    .wrapping_mul(0x566)
                    .wrapping_add(b.wrapping_mul(2)) as usize;
                let p = (&raw mut DAT_0070b1da).cast::<u8>().byte_add(off);
                if p.cast::<u16>().read_unaligned() == 0 {
                    v = 0x8000;
                } else {
                    v = (b.wrapping_add(0x8000)) as u16;
                }
            }
            i_var3.byte_add(0x18).cast::<u16>().write_unaligned(v);
            i_var3.byte_add(0x1a).cast::<u16>().write_unaligned(0);
            i_var3.byte_add(0x1e).write(0);
            if i_var3.byte_add(0x18).cast::<i16>().read_unaligned() != -1 {
                let p_tmp =
                    (&raw mut DAT_006d9d56).cast::<u32>().read_unaligned() as usize as *mut u8;
                FUN_00421c70(
                    u32::from(p_tmp.byte_add(0x18).cast::<u16>().read_unaligned()),
                    sel_byte(),
                );
                let p_tmp =
                    (&raw mut DAT_006d9d56).cast::<u32>().read_unaligned() as usize as *mut u8;
                p_tmp
                    .byte_add(0x1f)
                    .cast::<u32>()
                    .write_unaligned(DAT_00732a2c);
                p_tmp
                    .byte_add(0x23)
                    .cast::<u32>()
                    .write_unaligned(DAT_00732a28);
                p_tmp
                    .byte_add(0x27)
                    .cast::<u32>()
                    .write_unaligned(DAT_00732a24);
            }
        }
        i_var3.byte_add(0x3a).cast::<u16>().write_unaligned(0);
        let c2 = pc_var2.add(1).read();
        if c2 != -1 {
            i_var3.byte_add(0x34).cast::<i8>().write(c2);
            FUN_00491ad0();
        }
        let base = DAT_00732e54 as u32 as usize as *mut u8;
        base.byte_add(0x6a).cast::<u16>().write_unaligned(0xffff);
        let base = DAT_00732e54 as u32 as usize as *mut u8;
        base.byte_add(0x6c).cast::<u16>().write_unaligned(0);
        let base = DAT_00732e54 as u32 as usize as *mut u8;
        base.byte_add(0x6e).cast::<u16>().write_unaligned(0xffff);
        let i_tmp = i_var3.byte_add(0xe).cast::<i32>().read_unaligned();
        let u_tmp = ((i_tmp as u32).wrapping_mul(param_1 & 7)) >> 3;
        i_var3.byte_add(0x12).cast::<u32>().write_unaligned(u_tmp);
    }
}
