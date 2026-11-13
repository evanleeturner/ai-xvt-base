// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_0051c5aa: [i8; 1];
    static mut DAT_009e9697: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00406290(param_1: i32, u_var7: u32, byte_val: u32, param_2: u32) -> i32;
    fn FUN_00484d80(arg: u32) -> u32;
    fn FUN_00450650(arg1: u32, arg2: i32);
}

#[inline(always)]
unsafe fn rb(addr: u32) -> u8 {
    // SAFETY: the address is inside the game's own memory, as in the original.
    unsafe { (addr as usize as *const u8).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00406030(param_1: i32, param_2: u32) {
    // SAFETY: all globals, callees and addresses belong to the game; they are
    // read and written exactly where the original does.
    unsafe {
        let p = (DAT_009a1fe8 as u32)
            .wrapping_add(0x1f)
            .wrapping_add((param_1 as u32).wrapping_mul(0x23));
        DAT_00a08104 = ((p as usize as *const u32)
            .read_unaligned()
            .wrapping_add(0xa9) as usize as *const i32)
            .read_unaligned();
        let b_var2 = rb(param_2
            .wrapping_add(0x235)
            .wrapping_add(DAT_00a08104 as u32));
        let i_var6 = (param_2 as i32).wrapping_add(DAT_00a08104);
        let mut s_var8: i16 = 0;
        let mut b_var5: i32 = 0;
        let idx = param_2
            .wrapping_add((rb((DAT_00a08104 as u32).wrapping_add(4)) as u32).wrapping_mul(0x101));
        let b_var3: u8 = ((&raw mut DAT_0051c5aa) as *mut u8)
            .add(idx as usize)
            .read();
        let mut u_var7: u32 = b_var3 as u32;
        let c_var4: i8;

        'lab: {
            if (b_var2 & 0x7f) == 3 {
                let r = FUN_00406290(
                    param_1,
                    u_var7,
                    rb((i_var6 as u32).wrapping_add(0x233)) as u32,
                    param_2,
                );
                if r == -1 {
                    if rb((DAT_00a08104 as u32)
                        .wrapping_add(0x372)
                        .wrapping_add(u_var7.wrapping_mul(4)))
                        != 0
                    {
                        b_var5 = 1;
                    }
                } else {
                    s_var8 = 1;
                }
                u_var7 = (b_var3 as u16).wrapping_add(1) as u32;
                let r = FUN_00406290(
                    param_1,
                    u_var7,
                    rb((DAT_00a08104 as u32)
                        .wrapping_add(0x233)
                        .wrapping_add(param_2)) as u32,
                    param_2,
                );
                if r != -1 {
                    s_var8 = s_var8.wrapping_add(1);
                    break 'lab;
                }
                c_var4 = rb((DAT_00a08104 as u32)
                    .wrapping_add(0x372)
                    .wrapping_add(u_var7.wrapping_mul(4))) as i8;
            } else if (b_var2 & 0x80) == 0 {
                let r = FUN_00406290(
                    param_1,
                    u_var7,
                    rb((i_var6 as u32).wrapping_add(0x233)) as u32,
                    param_2,
                );
                if r != -1 {
                    s_var8 = 1;
                    break 'lab;
                }
                c_var4 = rb((DAT_00a08104 as u32)
                    .wrapping_add(0x372)
                    .wrapping_add(u_var7.wrapping_mul(4))) as i8;
            } else {
                u_var7 = (b_var3 as u16).wrapping_add(1) as u32;
                let r = FUN_00406290(
                    param_1,
                    u_var7,
                    rb((i_var6 as u32).wrapping_add(0x233)) as u32,
                    param_2,
                );
                if r != -1 {
                    s_var8 = 1;
                    break 'lab;
                }
                c_var4 = rb((DAT_00a08104 as u32)
                    .wrapping_add(0x372)
                    .wrapping_add(u_var7.wrapping_mul(4))) as i8;
            }
            if c_var4 != 0 {
                b_var5 = 1;
            }
        }

        let a = (DAT_00a08104 as u32)
            .wrapping_add(0x237)
            .wrapping_add(param_2.wrapping_mul(2)) as usize as *mut i16;
        let local_short = a.read_unaligned().wrapping_add(0x1d8);
        a.write_unaligned(local_short);

        let q = (DAT_009a1fe8 as u32)
            .wrapping_add(0x1b)
            .wrapping_add((param_1 as u32).wrapping_mul(0x23));
        if (q as usize as *const i32).read_unaligned() == DAT_009ecc34 && b_var5 == 0 {
            let ci = DAT_009ecc34.wrapping_mul(0x5bd) as isize;
            let ch = ((&raw mut DAT_009e9697) as *mut i8).offset(ci).read();
            let addr = (ch as i32).wrapping_add(0x233).wrapping_add(DAT_00a08104) as u32;
            let byte = rb(addr) as u32;
            u_var7 = FUN_00484d80(((DAT_00a08104 as u32 & 0xFFFFFF00) | byte) & 0xFFFF00FF);
            if s_var8 == 0 {
                FUN_00450650((u_var7 & 0xffff).wrapping_add(0x26), DAT_009ecc34);
                return;
            }
            if s_var8 == 1 {
                FUN_00450650((u_var7 & 0xffff).wrapping_add(0x30), DAT_009ecc34);
                return;
            }
            if s_var8 == 2 {
                FUN_00450650((u_var7 & 0xffff).wrapping_add(0x3a), DAT_009ecc34);
            }
        }
    }
}
