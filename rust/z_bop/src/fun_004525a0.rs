// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009d8c30: [i8; 0];
    static mut DAT_00a60698: [*mut i8; 0];
    static mut DAT_00a605e4: [*mut i8; 0];
    static mut DAT_0051c560: [i8; 0];
    static mut DAT_0051c56a: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00452830(param_1: *mut i8, param_2: *mut i8);
    fn FUN_00452860(param_1: i8, param_2: *mut i8);
    fn FUN_0043c740(param_1: u32, param_2: *mut u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004525a0(param_1: u32, param_2: i16, param_3: *mut i8) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the pointers read from the game's records are used as the original does.
    unsafe {
        let mut b_var4: bool = false;
        *param_3 = 0;
        let i_var3: u32 = (param_1 & 0xffff).wrapping_mul(0x23);
        let i_var7: *mut u8 = DAT_009a1fe8.byte_add(i_var3 as usize);
        let names = (&raw mut DAT_009d8c30).cast::<i8>();
        let tab98 = (&raw mut DAT_00a60698).cast::<*mut i8>();
        let mut pc_var5: *mut i8 = i_var7.byte_add(0x1f).cast::<*mut i8>().read_unaligned();
        if pc_var5.is_null() {
            if param_2 != 1 && param_2 != 0 {
                FUN_00452830(
                    names.byte_add((*i_var7.byte_add(0x16) as usize).wrapping_mul(0x566)),
                    param_3,
                );
                return;
            }
            FUN_00452830(tab98.add(*i_var7.byte_add(3) as usize).read(), param_3);
            let p = names.byte_add(
                (*DAT_009a1fe8.byte_add(0x16usize.wrapping_add(i_var3 as usize)) as usize)
                    .wrapping_mul(0x566),
            );
            if *p == 0 {
                return;
            }
            FUN_00452860(0x20, param_3);
            FUN_00452830(p, param_3);
            return;
        }
        let mut b_var1: u8 = *i_var7.byte_add(3);
        if *pc_var5 != 0 {
            if 0x8e < b_var1 && b_var1 < 0x9c {
                let tab_e4 = (&raw mut DAT_00a605e4).cast::<*mut i8>();
                FUN_00452830(tab_e4.add(b_var1 as usize).read(), param_3);
                return;
            }
            if b_var1 < 0x46 {
                return;
            }
            if 0x54 < b_var1 {
                return;
            }
            FUN_00452830(tab98.add(b_var1 as usize).read(), param_3);
            return;
        }
        let p_var2: *mut u32 = pc_var5.byte_add(0xa9).cast::<*mut u32>().read_unaligned();
        b_var1 = *i_var7.byte_add(0x16);
        if param_2 == 1 {
            let idx = (*p_var2.add(1).cast::<u8>() as usize).wrapping_mul(0x101);
            pc_var5 = (&raw mut DAT_0051c56a)
                .cast::<u8>()
                .byte_add(idx)
                .cast::<*mut i8>()
                .read_unaligned();
            b_var4 = true;
            FUN_00452830(pc_var5, param_3);
        } else if param_2 == 0 {
            let idx = (*p_var2.add(1).cast::<u8>() as usize).wrapping_mul(0x101);
            pc_var5 = (&raw mut DAT_0051c560).cast::<i8>().byte_add(idx);
            b_var4 = true;
            FUN_00452830(pc_var5, param_3);
        }
        pc_var5 = names.byte_add((b_var1 as usize).wrapping_mul(0x566));
        if *pc_var5 != 0 {
            if b_var4 {
                FUN_00452860(0x20, param_3);
            }
            b_var4 = true;
            FUN_00452830(pc_var5, param_3);
        }
        let u_var6: u32 = FUN_0043c740(b_var1 as u32, p_var2);
        if (u_var6 as u16) == 0 {
            return;
        }
        if b_var4 {
            FUN_00452860(0x20, param_3);
        }
        if (u_var6 as u16) < 10 {
            FUN_00452860((u_var6 as i8).wrapping_add(0x30), param_3);
            return;
        }
        FUN_00452860((((u_var6 & 0xffff) / 10) as i8).wrapping_add(0x30), param_3);
        FUN_00452860((((u_var6 & 0xffff) % 10) as i8).wrapping_add(0x30), param_3);
    }
}
