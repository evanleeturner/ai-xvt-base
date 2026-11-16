// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0071b743: [u8; 0];
    static mut DAT_0071b74a: [u8; 0];
    static mut DAT_0071b756: [u8; 0];
    static mut DAT_0071b759: [u8; 0];
    static mut DAT_0071bb27: [u8; 0];
    static mut DAT_0071bb37: [u8; 0];
    static mut DAT_0071bb4a: [u8; 0];
    static mut _DAT_00732950: u16;
    static mut _DAT_00732952: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040a1a0(a: u32, b: u32, c: i32);
    fn FUN_004490f0(a: u32) -> i32;
    fn FUN_00449790(a: u32, b: u32) -> i32;
    fn FUN_004a3830(a: u32, b: i32, c: i32, d: i32);
    fn FUN_004a1d70(a: i32, b: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00442bf0(param_1: u16, param_2: i32) {
    // SAFETY: all globals and addresses are the game's own memory, accessed
    // at the offsets the original uses.
    unsafe {
        let u_var6: u32 = param_1 as u32;
        if param_1 == 0xffff {
            return;
        }
        let mut u_var7: u32 = param_1 as u32;
        let base = DAT_006d4988 as u32;
        let i_var3 = u_var7.wrapping_mul(0x23).wrapping_add(base) as usize as *const u8;
        if i_var3.wrapping_byte_add(3).read() == 0 || i_var3.wrapping_byte_add(2).read() == 13 {
            return;
        }
        let i_var3: i32 = (param_2 as u32).wrapping_mul(0x5bd) as i32;
        let o = i_var3 as usize;
        let a730 = (&raw mut DAT_0071b730).cast::<u8>().wrapping_byte_add(o);
        let a743 = (&raw mut DAT_0071b743).cast::<u8>().wrapping_byte_add(o);
        let a74a = (&raw mut DAT_0071b74a).cast::<u8>().wrapping_byte_add(o);
        let a756 = (&raw mut DAT_0071b756).cast::<u8>().wrapping_byte_add(o);
        let a759 = (&raw mut DAT_0071b759).cast::<u8>().wrapping_byte_add(o);
        let abb27 = (&raw mut DAT_0071bb27).cast::<u8>().wrapping_byte_add(o);
        let abb37 = (&raw mut DAT_0071bb37).cast::<u8>().wrapping_byte_add(o);
        let abb4a = (&raw mut DAT_0071bb4a).cast::<u8>().wrapping_byte_add(o);

        let u_var1: u32 = a730.cast::<u32>().read_unaligned();
        if u_var1 == u_var7 {
            return;
        }
        let mut b_var2 = true;
        if u_var1 != 0xffff_ffff {
            let t = (u_var1
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(base) as usize as *const u32)
                .read_unaligned();
            let p = (t.wrapping_add(0xa7) as usize as *const u32).read_unaligned();
            let b = (p.wrapping_add(0x1df) as usize as *const u8).read();
            if b & 4 == 0 {
                b_var2 = false;
            }
        }
        if !b_var2 {
            _DAT_00732950 = 0x5f;
            _DAT_00732952 = 0x57;
            FUN_004a1d70(0x56, param_2);
            return;
        }
        let s_tmp: u16 = a74a.cast::<u16>().read_unaligned();
        if s_tmp == param_1 {
            return;
        }
        FUN_0040a1a0(0x31, 0xffff_ffff, param_2);
        a74a.cast::<u16>().write_unaligned(param_1);
        a759.cast::<u16>().write_unaligned(0);
        if (u_var7 as i32) < DAT_006d4980 {
            u_var7 = 0;
            let s_cur = a74a.cast::<u16>().read_unaligned() as u32;
            let i_var4 = FUN_004490f0(
                (s_cur
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_006d4988 as u32) as usize as *const u8)
                    .read() as u32,
            );
            if 0 < i_var4 {
                loop {
                    let s_cur = a74a.cast::<u16>().read_unaligned() as u32;
                    let i_var5 = FUN_00449790(
                        (s_cur
                            .wrapping_mul(0x23)
                            .wrapping_add(3)
                            .wrapping_add(DAT_006d4988 as u32) as usize
                            as *const u8)
                            .read() as u32,
                        u_var7,
                    );
                    if i_var5 == 1 || i_var5 == 3 {
                        a759.cast::<u16>().write_unaligned(u_var7 as u16);
                        break;
                    }
                    u_var7 = (u_var7 as u16).wrapping_add(1) as u32;
                    if (u_var7 as i32) >= i_var4 {
                        break;
                    }
                }
            }
        }
        if abb4a.cast::<u16>().read_unaligned() != 0 {
            let s = a74a.cast::<u16>().read_unaligned();
            abb27.cast::<u16>().write_unaligned(s);
        }
        a756.write(0);
        let mut u_tmp: u32 = a730.cast::<u32>().read_unaligned();
        if u_tmp as i32 != -1 {
            u_tmp = (u_tmp
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_006d4988 as u32) as usize as *const u32)
                .read_unaligned();
            let u_p = (u_tmp.wrapping_add(0xa7) as usize as *const u32).read_unaligned();
            (u_p.wrapping_add(0x231) as usize as *mut u16).write_unaligned(0);
        }
        if a743.read() != 0 {
            FUN_004a3830(u_var6, param_2, 1, 0);
            return;
        }
        let mut u_tmp: u32 = a730.cast::<u32>().read_unaligned();
        u_tmp = (u_tmp
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_006d4988 as u32) as usize as *const u32)
            .read_unaligned();
        let u_p = (u_tmp.wrapping_add(0xa7) as usize as *const u32).read_unaligned();
        if (u_p.wrapping_add(0x1db) as usize as *const u8).read() & 1 != 0 {
            let v = abb37.read();
            let other = (&raw mut DAT_0071bb37)
                .cast::<u8>()
                .wrapping_byte_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
                .read();
            if v == 0x13 || v == 0 || other == 0x14 {
                FUN_004a3830(u_var6, param_2, 1, 0);
                return;
            }
        }
    }
}
