// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa5aac: i16;
    static mut DAT_00b6a5a6: u8;
    static mut DAT_00aa5cb0: [i32; 0];
    static mut DAT_00aa5ac0: [i32; 0];
    static mut DAT_00aa5e10: [i32; 0];
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a2a0: u32;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00b6a270: [u8; 0];
    static mut DAT_00a91d24: [i8; 0];
    static mut DAT_00b69d20: [i8; 0];
    static mut DAT_0052d0b0: [i8; 0];
    static mut DAT_0052a6b0: [i8; 0];
    static mut DAT_0052d0a8: [i8; 0];
    static mut DAT_00aa6150: [u8; 0];
    static mut DAT_00aa615e: [u8; 0];
    static mut DAT_00aa6162: [u8; 0];
    static mut DAT_00aa62aa: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn FUN_004d11e0() -> i32;
    fn sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004db7d0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
    fn FUN_004d4cf0(param_1: *mut u32);
    fn FUN_004d4d20(param_1: *mut core::ffi::c_void);
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004fa810(param_1: i32) -> u32 {
    let mut local_30: [i32; 4] = [0; 4];
    let mut local_20: [i32; 4] = [0; 4];
    let mut local_10: [u32; 4] = [0; 4];
    // SAFETY: all globals and callees are the game's own memory and code; the
    // pointers are derived from the game's tables and read where the original does.
    unsafe {
        let a_cb0 = &raw mut DAT_00aa5cb0 as *mut i32;
        let a_ac0 = &raw mut DAT_00aa5ac0 as *mut i32;
        let a_e10 = &raw mut DAT_00aa5e10 as *mut i32;
        let buf = &raw mut DAT_00b69d20 as *mut i8;
        let base_6150 = &raw mut DAT_00aa6150 as *mut u8;
        let base_615e = &raw mut DAT_00aa615e as *mut u8;
        let base_6162 = &raw mut DAT_00aa6162 as *mut u8;
        let end_limit = (&raw mut DAT_00aa62aa as *mut u8).add(3);

        FUN_004d6460(local_30.as_mut_ptr(), 0x58, 0x14f, 0x1bb, 0x15d);
        let mut u_var9: u32 = 0xffff;
        let mut pc = FUN_004ddc10(0x273);
        FUN_004db640(0xc, pc, local_30.as_mut_ptr(), u_var9);
        let i_var2 = FUN_004d11e0();
        if i_var2 == *a_cb0.offset((DAT_00b6a5a6 as isize) * 8) || DAT_00b69e30 == 2 {
            FUN_004d6460(local_30.as_mut_ptr(), 0x58, 0x1a6, 0x1bb, 0x1b4);
            u_var9 = DAT_00b6a2a0;
            pc = FUN_004ddc10(0x2e5);
            FUN_004db640(0xc, pc, local_30.as_mut_ptr(), u_var9);
        }
        let mut i_var7: i32 = 0;
        let mut i_var2: i32 = 0;
        FUN_004d6460(local_30.as_mut_ptr(), 0x58, 0x160, 0x108, 0x170);
        if DAT_00aa5aac < 1 {
            return 1;
        }
        let mut local_34: *mut i8 = &raw mut DAT_00a91d24 as *mut i8;
        loop {
            'next: {
                if !(*local_34 != 0 && (local_34.offset(-0xc).read() as u8) == DAT_00b6a5a6) {
                    break 'next;
                }
                let mut i_var5: i32 = 0;
                'after: {
                    if 0 < *a_e10.offset(DAT_00b6a5a6 as isize) {
                        loop {
                            if *a_ac0.offset((DAT_00b6a5a6 as isize) * 8 + i_var5 as isize)
                                == i_var2
                            {
                                let mut i_var6: i32 = 0;
                                let mut p4: *mut u8 = base_615e;
                                loop {
                                    let tmp = p4.cast::<i32>().read_unaligned();
                                    if tmp
                                        == *a_cb0
                                            .offset((DAT_00b6a5a6 as isize) * 8 + i_var5 as isize)
                                    {
                                        sprintf(
                                            buf,
                                            &raw const DAT_0052d0b0 as *const i8,
                                            i_var7.wrapping_add(1),
                                        );
                                        FUN_004d6480(local_20.as_mut_ptr(), local_30.as_mut_ptr());
                                        local_20[2] = (local_20[0] as u32).wrapping_add(0xf) as i32;
                                        FUN_004db7d0(0xc, buf, local_20.as_mut_ptr(), 0, 1, 0xffff);
                                        local_20[0] = (local_30[0] as u32).wrapping_add(0xf) as i32;
                                        local_20[2] = local_30[2];
                                        let p3 =
                                            base_6150.offset((i_var6 as isize).wrapping_mul(0x2a));
                                        let u_var8: u32 = 1;
                                        let utmp = base_6162
                                            .offset((i_var6 as isize).wrapping_mul(0x2a))
                                            .cast::<u32>()
                                            .read_unaligned();
                                        pc = FUN_004ddc10(utmp.wrapping_add(0x9a) as i32);
                                        sprintf(
                                            buf,
                                            &raw const DAT_0052a6b0 as *const i8,
                                            6i32,
                                            pc,
                                            u_var8,
                                            p3 as *mut i8,
                                        );
                                        FUN_004d4cf0(local_10.as_mut_ptr());
                                        FUN_004d4d20(local_30.as_mut_ptr().cast());
                                        let i_var3 = FUN_004d11e0();
                                        let tmp2 = base_615e
                                            .offset((i_var6 as isize).wrapping_mul(0x2a))
                                            .cast::<i32>()
                                            .read_unaligned();
                                        let mut use_tbl = i_var3 == tmp2;
                                        if !use_tbl {
                                            u_var9 = DAT_00b6a2c0;
                                            use_tbl = DAT_00b69e30 == 2;
                                        }
                                        if use_tbl {
                                            let idx = ((param_1 % 0x18) as u32 & 0xffff_fffe)
                                                .wrapping_mul(2);
                                            u_var9 = (&raw mut DAT_00b6a270 as *mut u8)
                                                .add(idx as usize)
                                                .cast::<u32>()
                                                .read_unaligned();
                                        }
                                        FUN_004db7d0(0xc, buf, local_20.as_mut_ptr(), 0, 1, u_var9);
                                        FUN_004d4d20(local_10.as_mut_ptr().cast());
                                        break;
                                    }
                                    p4 = p4.add(0x2a);
                                    i_var6 = i_var6.wrapping_add(1);
                                    if end_limit < p4 {
                                        break;
                                    }
                                }
                                break 'after;
                            }
                            i_var5 = i_var5.wrapping_add(1);
                            if !(i_var5 < *a_e10.offset(DAT_00b6a5a6 as isize)) {
                                break;
                            }
                        }
                    }
                }
                if *a_e10.offset(DAT_00b6a5a6 as isize) == i_var5 {
                    pc = FUN_004ddc10(0x321);
                    sprintf(
                        buf,
                        &raw const DAT_0052d0a8 as *const i8,
                        i_var7.wrapping_add(1),
                        pc,
                    );
                    FUN_004db7d0(0xc, buf, local_30.as_mut_ptr(), 0, 1, 0xffff);
                }
                i_var7 = i_var7.wrapping_add(1);
                if i_var7 == 4 {
                    FUN_004d6460(local_30.as_mut_ptr(), 0x10b, 0x160, 0x1bb, 0x170);
                } else {
                    FUN_004d64a0(local_30.as_mut_ptr(), 0, 0x11);
                }
            }
            if *a_e10.offset(DAT_00b6a5a6 as isize) <= i_var7 {
                return 1;
            }
            local_34 = local_34.byte_add(0x562);
            i_var2 = i_var2.wrapping_add(1);
            if (DAT_00aa5aac as i32) <= i_var2 {
                return 1;
            }
        }
    }
}
