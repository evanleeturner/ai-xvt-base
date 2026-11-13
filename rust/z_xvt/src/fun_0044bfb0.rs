// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d49a0: u16;
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00703ca0: [i8; 0];
    static mut s_escortldr1pln_00524904: [i8; 0];
    static mut DAT_00732e54: [i8; 0];
    static mut DAT_006d9d7a: u8;
    static mut DAT_006d9d50: u32;
    static mut DAT_006d499c: u32;
    static mut DAT_006d9d56: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049ec30(a: u16, b: u16, c: u16) -> u16;
    fn FUN_0042b150(a: i32) -> i32;
    fn FUN_00429a20(a: u32) -> u8;
    fn FUN_00429c40(a: u32) -> u8;
    fn FUN_0044c560(a: u16, b: i16) -> u8;
    fn FUN_0042a1a0(a: u32, b: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044bfb0(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) -> u16 {
    let mut u_stack_6: u16 = 0xffff;
    let mut local_4: u16 = 0xffff;
    let mut u_stack_2: u16 = 0xffff;
    let mut local_8: u16 = 0;
    let mut unaff_ebp: u32 = 0;
    // SAFETY: globals and callees are the game's own memory; pointers read
    // are the game's records, accessed at the original's offsets.
    unsafe {
        if 0 < DAT_006d49a0 as i32 {
            loop {
                let mut u_var1 = FUN_0049ec30(local_8, param_1, param_2);
                let u_var2 = FUN_0049ec30(local_8, param_4, param_5);
                if param_3 == 1 {
                    u_var1 |= u_var2;
                } else {
                    u_var1 &= u_var2;
                }
                if u_var1 != 0 {
                    unaff_ebp = (unaff_ebp & 0xFFFF0000) | DAT_00732a3c as u32;
                    if (DAT_00732a3c as i32) < DAT_006d4980 {
                        loop {
                            let u_var5 = unaff_ebp & 0xffff;
                            let i_var3 = (u_var5 as i32)
                                .wrapping_mul(0x23)
                                .wrapping_add(DAT_006d4988);
                            let u_var1 = unaff_ebp as u16;
                            'skip: {
                                if *((i_var3 as u32 as usize + 3) as *const u8) != 0 {
                                    let rec = ((i_var3 as u32 as usize + 0x1f) as *const i32)
                                        .read_unaligned();
                                    let arg = ((rec as u32 as usize + 0xa7) as *const i32)
                                        .read_unaligned();
                                    let i_var3 = FUN_0042b150(arg);
                                    let mut b_var8: bool = false;
                                    let mut i_var4: i32 = 0xe;
                                    let mut pc6 = (&raw mut DAT_00703ca0 as *const i8).byte_add(
                                        (*((i_var3 as u32 as usize + 0xb) as *const u8) as usize)
                                            * 0x55,
                                    );
                                    let mut pc7 = &raw mut s_escortldr1pln_00524904 as *const i8;
                                    loop {
                                        if i_var4 == 0 {
                                            break;
                                        }
                                        i_var4 -= 1;
                                        b_var8 = *pc6 == *pc7;
                                        pc6 = pc6.add(1);
                                        pc7 = pc7.add(1);
                                        if !b_var8 {
                                            break;
                                        }
                                    }
                                    if b_var8
                                        && (*((i_var3 as u32 as usize + 0x2d) as *const u8) as u16)
                                            == local_8
                                        && ((&raw mut DAT_00732e54 as *const u8)
                                            .byte_add(0x342)
                                            .cast::<u16>()
                                            .read_unaligned())
                                            != u_var1
                                    {
                                        if FUN_00429a20(u_var5) != 0 {
                                            if (DAT_006d9d7a & 4) != 0
                                                && FUN_00429c40(unaff_ebp) == 0
                                            {
                                                break 'skip;
                                            }
                                            if (DAT_006d9d7a & 1) != 0
                                                && FUN_0044c560(u_var1, 0xff) == 0
                                            {
                                                break 'skip;
                                            }
                                            FUN_0042a1a0(DAT_006d9d50, u_var5);
                                            if DAT_006d499c
                                                < (((u_stack_2 as u32) << 16) | local_4 as u32)
                                            {
                                                local_4 = DAT_006d499c as u16;
                                                u_stack_2 = (DAT_006d499c >> 0x10) as u16;
                                                u_stack_6 = u_var1;
                                            }
                                        }
                                    }
                                }
                            }
                            unaff_ebp = (unaff_ebp & 0xFFFF0000) | (u_var1 as u32 + 1);
                            if !((u_var1.wrapping_add(1) as i32) < DAT_006d4980) {
                                break;
                            }
                        }
                    }
                }
                local_8 = local_8.wrapping_add(1);
                if !((local_8 as i32) < (DAT_006d49a0 as i32)) {
                    break;
                }
            }
        }
        if u_stack_6 != 0xffff {
            let base = DAT_006d9d56 as *mut u8;
            base.byte_add(0x18).cast::<u16>().write_unaligned(u_stack_6);
            let v = ((u_stack_6 as u32 as usize * 0x23 + DAT_006d4988 as u32 as usize)
                as *const u16)
                .read_unaligned();
            base.byte_add(0x1a).cast::<u16>().write_unaligned(v);
            base.byte_add(0x1e).write(1);
        }
    }
    u_stack_6
}
