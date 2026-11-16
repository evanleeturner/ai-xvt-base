// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: *mut u8;
    static mut DAT_006db798: *mut u8;
    static mut DAT_0071b704: i32;
    static mut DAT_00732904: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_00732a28: u32;
    static mut DAT_00732a2c: u32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_00524870: [u8; 1];
    static mut DAT_0070ad45: [u8; 1];
    static mut DAT_0070ad47: [u8; 1];
    static mut DAT_0070ad48: [u8; 1];
    static mut DAT_00525bf7: [u8; 1];
    static mut DAT_00525aae: [u16; 1];
    static mut DAT_00525a7e: [u16; 1];
    static mut DAT_00525ade: [u16; 1];
    static mut DAT_00525b0e: [u16; 1];
    static mut DAT_0071b776: [u8; 1];
    static mut DAT_0071b777: [u8; 1];
    static mut DAT_0071bcdd: [u8; 1];
    static mut DAT_0071b730: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00422710(param_1: i32) -> u16;
    fn FUN_00478320(param_1: u32, param_2: u32) -> u32;
    fn FUN_00421c70(param_1: u32, param_2: i32);
    fn FUN_00478470() -> u8;
    fn FUN_004a1d70(param_1: i32, param_2: i32);
    fn FUN_0040b730(
        param_1: i32,
        param_2: u32,
        param_3: i32,
        param_4: i32,
        param_5: u32,
        param_6: u32,
    );
}

unsafe fn rec(off: u32) -> *mut u8 {
    // SAFETY: caller guarantees the game's record table is valid.
    unsafe { DAT_006d4988.wrapping_add(off as usize) }
}

unsafe fn recp(iv8: u32) -> *mut u8 {
    // SAFETY: reads the record's sub-pointer at +0x1f.
    unsafe {
        rec(iv8.wrapping_add(0x1f))
            .cast::<*mut u8>()
            .read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00448450(param_1: u16, param_2: u16) -> u16 {
    // SAFETY: all globals and callees are the game's own memory; pointers
    // are dereferenced exactly where the original does.
    unsafe {
        let i_var2: u32 = (param_1 as u32)
            .wrapping_mul(8)
            .wrapping_sub(param_1 as u32)
            .wrapping_mul(5);
        let u_var6: u32 = *rec(0x16u32.wrapping_add(i_var2)) as u32;
        let mut i_var7: u32 = u_var6.wrapping_mul(0x566);
        let tab = (&raw mut DAT_0070ad45).cast::<u8>();
        let mut b_var4: u8 = *(&raw mut DAT_00524870)
            .cast::<u8>()
            .add(*tab.wrapping_add(i_var7 as usize) as usize);
        if b_var4 == 0 {
            return 0xffff;
        }
        let mut u_var5: u16 = FUN_00422710(7);
        if u_var5 == 0xffff {
            u_var5 = (DAT_0071b704 as i16).wrapping_add(0x80) as u16;
            while (u_var5 as i32) < DAT_00732904 {
                let i_var8 = (u_var5 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_006d4988 as u32);
                let p = ((i_var8.wrapping_add(0x1f)) as *const *mut u8).read_unaligned();
                let t3 = *((i_var8.wrapping_add(3)) as *const u8);
                let flag = *(&raw mut DAT_00525bf7).cast::<u8>().add(t3 as usize);
                let c1 = *(p.wrapping_add(0x87)) as i8;
                let c2 = *(&raw mut DAT_0070ad48)
                    .cast::<u8>()
                    .wrapping_add(i_var7 as usize) as i8;
                if !(flag != 0 || c1 != c2) {
                    break;
                }
                u_var5 = u_var5.wrapping_add(1);
            }
        }
        let mut u_var6: u32 = u_var5 as u32;
        if DAT_00732904 == u_var6 as i32 {
            return 0xffff;
        }
        let i_var8: u32 = u_var6.wrapping_mul(0x23);

        let mut p = recp(i_var8);
        *p = 1;
        *rec(2u32.wrapping_add(i_var8)) = 7;
        *rec(3u32.wrapping_add(i_var8)) = b_var4;
        p = recp(i_var8);
        p.wrapping_add(0x81).cast::<u16>().write_unaligned(1);
        p = recp(i_var8);
        p.wrapping_add(0x83).cast::<u16>().write_unaligned(param_1);
        p = recp(i_var8);
        *p.wrapping_add(0x85) = *rec(3u32.wrapping_add(i_var2));
        p = recp(i_var8);
        *p.wrapping_add(0x86) = *(&raw mut DAT_0070ad47)
            .cast::<u8>()
            .wrapping_add(i_var7 as usize);
        let u_var9: usize = b_var4 as usize;
        rec(0x12u32.wrapping_add(i_var8))
            .cast::<u16>()
            .write_unaligned(0);
        rec(0x14u32.wrapping_add(i_var8))
            .cast::<u16>()
            .write_unaligned(0);
        rec(0x10u32.wrapping_add(i_var8))
            .cast::<u16>()
            .write_unaligned(0);
        p = recp(i_var8);
        let mut us: u16 = (&raw mut DAT_00525aae).cast::<u16>().add(u_var9).read();
        p.wrapping_add(0x79).cast::<u16>().write_unaligned(us);
        p = recp(i_var8);
        us = p.wrapping_add(0x79).cast::<u16>().read_unaligned();
        DAT_006db798
            .wrapping_add(6)
            .wrapping_add((u_var6.wrapping_sub(DAT_0071b704 as u32).wrapping_mul(8)) as usize)
            .cast::<u16>()
            .write_unaligned(us);
        i_var7 = DAT_006d4988 as u32;
        p = recp(i_var8);
        us = (&raw mut DAT_00525a7e).cast::<u16>().add(u_var9).read();
        p.wrapping_add(0x7d).cast::<u16>().write_unaligned(us);
        let s_var1: i16 = (&raw mut DAT_00525ade).cast::<u16>().add(u_var9).read() as i16;
        let w = (&raw mut DAT_00525b0e).cast::<u16>().add(u_var9).read() as u32;
        u_var6 = FUN_00478320((((i_var7 >> 0x10) & 0xffff) << 0x10) | w, 0xec);
        p = recp(i_var8);
        let ss: i16 = ((s_var1 as i32 as u32)
            .wrapping_mul(0xec)
            .wrapping_add(u_var6 as i16 as i32 as u32)) as i16;
        p.wrapping_add(0x7f).cast::<i16>().write_unaligned(ss);

        FUN_00421c70(param_1 as u32, 0);

        let u_var3 = DAT_00732a28;
        i_var7 = (DAT_00732a24 as u32).wrapping_add(0x180);
        p = recp(i_var8);
        let mut ui: u32 = DAT_00732a2c;
        p.wrapping_add(6).cast::<u32>().write_unaligned(ui);
        p = recp(i_var8);
        ui = p.wrapping_add(6).cast::<u32>().read_unaligned();
        rec(i_var8.wrapping_add(4))
            .cast::<u32>()
            .write_unaligned(ui);
        p = recp(i_var8);
        p.wrapping_add(10).cast::<u32>().write_unaligned(u_var3);
        p = recp(i_var8);
        ui = p.wrapping_add(10).cast::<u32>().read_unaligned();
        rec(i_var8.wrapping_add(8))
            .cast::<u32>()
            .write_unaligned(ui);
        p = recp(i_var8);
        p.wrapping_add(0xe).cast::<u32>().write_unaligned(i_var7);
        p = recp(i_var8);
        ui = p.wrapping_add(0xe).cast::<u32>().read_unaligned();
        rec(i_var8.wrapping_add(0xc))
            .cast::<u32>()
            .write_unaligned(ui);

        let u_var10: u16 = u_var5.wrapping_sub(DAT_0071b704 as i16 as u16);
        i_var7 = (u_var10 as u32).wrapping_mul(8);
        b_var4 = FUN_00478470();
        *DAT_006db798.wrapping_add(i_var7 as usize + 1) = (b_var4 & 3).wrapping_add(3);
        DAT_006db798
            .wrapping_add(i_var7 as usize + 4)
            .cast::<u16>()
            .write_unaligned(param_2);
        *DAT_006db798.wrapping_add(i_var7 as usize) = 0xff;
        p = recp(i_var8);
        let q: *mut u8 = DAT_006db798.wrapping_add(i_var7 as usize);
        p.wrapping_add(0xa3).cast::<*mut u8>().write_unaligned(q);

        let mut si: i32 = rec((param_2 as u32).wrapping_mul(0x23).wrapping_add(0x1b))
            .cast::<i32>()
            .read_unaligned();
        if si != -1 {
            us = DAT_006db798
                .wrapping_add(i_var7 as usize + 4)
                .cast::<u16>()
                .read_unaligned();
            si = rec((us as u32).wrapping_mul(0x23).wrapping_add(0x1b))
                .cast::<i32>()
                .read_unaligned();
            if si != -1 {
                let i7 = si as u32;
                let i2 = i7.wrapping_mul(0x5bd) as usize;
                let f776 = (&raw mut DAT_0071b776).cast::<u8>().wrapping_add(i2);
                if *f776 == 0 {
                    *f776 = 1;
                    let i8 = DAT_0071ecf4 as u32;
                    us = u_var10.wrapping_add(DAT_0071b704 as i16 as u16);
                    (&raw mut DAT_0071b777)
                        .cast::<u8>()
                        .wrapping_add(i2)
                        .cast::<u16>()
                        .write_unaligned(us);
                    (&raw mut DAT_0071bcdd)
                        .cast::<u8>()
                        .wrapping_add(i2)
                        .cast::<u32>()
                        .write_unaligned(0x588);
                    if i7 == i8 {
                        FUN_004a1d70(0x73, i7 as i32);
                        ui = (&raw mut DAT_0071b730)
                            .cast::<u8>()
                            .wrapping_add(i2)
                            .cast::<u32>()
                            .read_unaligned();
                        FUN_0040b730(DAT_0071ecf4, 0xffff_ffff, 0xc, -1, ui, 0xffff);
                    }
                }
            }
        }
        u_var5
    }
}
