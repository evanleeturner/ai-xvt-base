// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_0070ad8b: [u8; 0];
    static mut DAT_0070ad43: [u8; 0];
    static mut DAT_0070ad47: [u8; 0];
    static mut DAT_0070acf0: [u8; 0];
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b743: [u8; 0];
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0071b73a: [u8; 0];
    static mut DAT_0071bb23: [u8; 0];
    static mut DAT_0071bb1f: [u8; 0];
    static mut DAT_0071bb1b: [u8; 0];
    static mut DAT_0052b07a: [u8; 0];
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_006db580: u32;
    static mut _DAT_00732950: u16;
    static mut _DAT_00732952: u16;
    static mut _DAT_00732954: u16;
    static mut DAT_00732956: i16;
    static mut DAT_00732fe2: u16;
    static mut _DAT_00732e78: i32;
    static mut _DAT_00732e74: i32;
    static mut _DAT_00732e70: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00421c70(param_1: u32, param_2: u32);
    fn FUN_00418cd0(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_004a1d70(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a31b0(param_1: u32, param_2: u32) {
    // SAFETY: all globals, tables and the callees belong to the game; the
    // pointers read through are the game's own records, accessed at the
    // same offsets as the original.
    unsafe {
        let u_var6: u32 = param_1 & 0xffff;
        let i_var1: i32 = u_var6.wrapping_mul(0x566) as i32;
        let mut u_var4: u16 = DAT_00732a3c;
        let p8b = (&raw mut DAT_0070ad8b).cast::<u8>();
        let p43 = (&raw mut DAT_0070ad43).cast::<u8>();
        let p47 = (&raw mut DAT_0070ad47).cast::<u8>();
        let pf0 = (&raw mut DAT_0070acf0).cast::<u8>();
        let p743 = (&raw mut DAT_0071b743).cast::<u8>();
        let p730 = (&raw mut DAT_0071b730).cast::<u8>();
        let p73a = (&raw mut DAT_0071b73a).cast::<u8>();
        let pb23 = (&raw mut DAT_0071bb23).cast::<u8>();
        let pb1f = (&raw mut DAT_0071bb1f).cast::<u8>();
        let pb1b = (&raw mut DAT_0071bb1b).cast::<u8>();
        let p07a = (&raw mut DAT_0052b07a).cast::<u8>();

        if *p8b.add(u_var6.wrapping_mul(0x566) as usize) == 0 {
            FUN_00421c70(0x8000, u_var6);
        } else {
            while (u_var4 as i32) < DAT_006d4980 {
                let i_var7: u32 = (u_var4 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_006d4988 as u32);
                if *((i_var7.wrapping_add(3)) as usize as *const i8) != 0
                    && (*((i_var7.wrapping_add(0x16)) as usize as *const u8) as u32) == u_var6
                {
                    let i_tmp: i32 =
                        ((i_var7.wrapping_add(0x1f)) as usize as *const i32).read_unaligned();
                    let i_tmp2: i32 =
                        ((i_tmp.wrapping_add(0xa7)) as usize as *const i32).read_unaligned();
                    if *((i_tmp2.wrapping_add(5)) as usize as *const i8) == -1 {
                        FUN_00421c70(u_var4 as u32, u_var6);
                        break;
                    }
                }
                u_var4 = u_var4.wrapping_add(1);
            }
        }

        let off: usize = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
        let i_var2: i32;
        let i_var3: i32;
        let i_var7: i32;
        if *p743.add(off) == 0 {
            let i_tmp: u32 = p730.add(off).cast::<u32>().read_unaligned();
            let base: u32 = i_tmp.wrapping_mul(0x23).wrapping_add(DAT_006d4988 as u32);
            i_var2 = (base.wrapping_add(0xc) as usize as *const i32).read_unaligned();
            i_var3 = (base.wrapping_add(8) as usize as *const i32).read_unaligned();
            i_var7 = (base.wrapping_add(4) as usize as *const i32).read_unaligned();
        } else {
            i_var2 = pb23.add(off).cast::<i32>().read_unaligned();
            i_var3 = pb1f.add(off).cast::<i32>().read_unaligned();
            i_var7 = pb1b.add(off).cast::<i32>().read_unaligned();
        }
        FUN_00418cd0(
            DAT_00732a2c.wrapping_sub(i_var7),
            DAT_00732a28.wrapping_sub(i_var3),
            DAT_00732a24.wrapping_sub(i_var2),
        );
        DAT_006db580 = DAT_006db580.wrapping_mul(0xa1);
        let u_tmp: u32 = (DAT_006db580 >> 0x10).wrapping_add(0x32);
        let mut s_var5: i16 = (u_tmp / 100) as i16;
        if s_var5 == 0 {
            s_var5 = 1;
        }
        let idx = i_var1 as usize;
        _DAT_00732950 = *p43.add(idx) as u16;
        DAT_00732fe2 = *p47.add(idx) as u16;
        _DAT_00732954 = s_var5 as u16;
        let us_tmp: u16 = p73a
            .add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<u16>()
            .read_unaligned();
        let poff: usize = (param_2 & 0xffff).wrapping_mul(0xef) as usize;
        if us_tmp == DAT_00732fe2 {
            if _DAT_00732950 != 1 {
                _DAT_00732952 = 0x8001;
                _DAT_00732e78 = (pf0 as usize as i32).wrapping_add(i_var1);
                _DAT_00732954 = 0x8002;
                _DAT_00732e74 = p07a.add(poff).cast::<i32>().read_unaligned();
                DAT_00732956 = s_var5;
                FUN_004a1d70(0xeb, DAT_0071ecf4);
                return;
            }
            _DAT_00732950 = 0x8000;
            _DAT_00732e74 = (pf0 as usize as i32).wrapping_add(i_var1);
            _DAT_00732952 = 0x8001;
            _DAT_00732e70 = p07a.add(poff).cast::<i32>().read_unaligned();
            FUN_004a1d70(0xea, DAT_0071ecf4);
            return;
        }
        _DAT_00732952 = 0x8001;
        _DAT_00732e74 = p07a.add(poff).cast::<i32>().read_unaligned();
        if *p43.add(idx) != 1 {
            FUN_004a1d70(0x72, DAT_0071ecf4);
            return;
        }
        FUN_004a1d70(0x71, DAT_0071ecf4);
    }
}
