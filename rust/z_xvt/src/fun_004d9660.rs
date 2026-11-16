// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00532af0: i32;
    static mut DAT_00532aec: i32;
    static mut DAT_0064f67c: i32;
    static mut DAT_0064f898: i32;
    static mut DAT_00532ad8: i32;
    static mut DAT_00534aac: i32;
    static mut DAT_005348a0: *const i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004df760(a: i32, b: i32, c: i32) -> i32;
    fn FUN_004ddc70(a: *mut i32, b: *mut i32);
    fn FUN_004dd7d0(a: *mut i32, b: i32, c: i32) -> i32;
    fn FUN_004db570() -> u8;
    fn FUN_004db590() -> u8;
    fn FUN_004dbe80() -> u32;
    fn FUN_004dbec0();
    fn __isctype(c: u32, mask: u32) -> u32;
    fn FUN_004db2f0(s: *mut i8, a: i32) -> i32;
    fn FUN_004d5180(a: *mut i32);
    fn FUN_004dcfd0(a: *mut u32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004d51b0(a: *mut i32);
    fn FUN_004da7c0(a: i32, s: *mut i8, b: i32, c: i32, d: i32);
    fn FUN_004d5270() -> i32;
    fn FUN_004dd000(a: *mut i32, b: *mut i32);
    fn FUN_004dd3e0(a: *mut i32, b: i32, c: i32, d: i16, e: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d9660(
    param_1: *mut i32,
    param_2: *mut i8,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: *mut i8,
) -> u32 {
    // SAFETY: the callees and globals are the game's own; the pointers are
    // the caller's buffers, read and written where the original does.
    unsafe {
        let mut i_stack_2c: i32 = 0;
        let mut i_stack_28: [i32; 4] = [0; 4];
        let mut u_stack_18: u32;
        let mut i_stack_14: i32 = 0;
        let mut au_stack_10: [u32; 4] = [0; 4];

        if DAT_00532af0 == 0 {
            DAT_00532aec = FUN_004df760(0xff, 0xff, 0xff);
            DAT_00532af0 = 1;
        }
        let mut len: u32 = 0;
        while *param_2.add(len as usize) != 0 {
            len = len.wrapping_add(1);
        }
        DAT_0064f67c = len as i32;
        DAT_0064f898 = len as i32;
        u_stack_18 = 0;
        FUN_004ddc70(&raw mut i_stack_2c, &raw mut i_stack_14);
        let i_var3 = FUN_004dd7d0(param_1, i_stack_2c, i_stack_14);
        if i_var3 != 0 && (FUN_004db570() != 0 || FUN_004db590() != 0) {
            DAT_00532ad8 = param_4;
        }
        'end: {
            if param_4 == DAT_00532ad8 {
                let u_var5 = FUN_004dbe80();
                let mut b_var6 = u_var5 as u8;
                if !param_6.is_null() {
                    let mut i: i32 = 0;
                    let mut c = *param_6;
                    while c != 0 {
                        if *param_6.offset(i as isize) == b_var6 as i8 {
                            b_var6 = 0;
                            FUN_004dbec0();
                            break;
                        }
                        i = i.wrapping_add(1);
                        c = *param_6.offset(i as isize);
                    }
                }
                if b_var6 != 0 {
                    let mut u5 = b_var6 as u32;
                    if u5 != 0x1b {
                        FUN_004dbec0();
                    }
                    if DAT_00534aac < 2 {
                        u5 = (DAT_005348a0
                            .byte_add(u5.wrapping_mul(2) as usize)
                            .cast::<u16>()
                            .read_unaligned() as u32)
                            & 0x157;
                    } else {
                        u5 = __isctype(u5, 0x157);
                    }
                    if u5 == 0 && b_var6 != 0x20 {
                        if b_var6 == 0xd || b_var6 == 9 {
                            u_stack_18 = 1;
                            break 'end;
                        }
                        if b_var6 != 8 || DAT_0064f898 == 0 {
                            break 'end;
                        }
                        DAT_0064f898 = DAT_0064f898.wrapping_sub(1);
                        DAT_0064f67c = DAT_0064f67c.wrapping_sub(1);
                    } else {
                        if param_3.wrapping_sub(1) <= DAT_0064f898 {
                            break 'end;
                        }
                        *param_2.offset(DAT_0064f898 as isize) = b_var6 as i8;
                        DAT_0064f67c = DAT_0064f67c.wrapping_add(1);
                        DAT_0064f898 = DAT_0064f898.wrapping_add(1);
                    }
                    *param_2.offset(DAT_0064f898 as isize) = 0;
                }
            }
        }
        let p0 = *param_1;
        let p1 = *param_1.add(1);
        let p2 = *param_1.add(2);
        let p3 = *param_1.add(3);
        let i_var3 = FUN_004db2f0(param_2, param_5);
        i_stack_2c = 0;
        if p2.wrapping_sub(p0).wrapping_add(1) < i_var3 {
            i_stack_2c = p2
                .wrapping_sub(p0)
                .wrapping_sub(i_var3)
                .wrapping_sub(param_5)
                .wrapping_add(1);
        }
        FUN_004d5180(i_stack_28.as_mut_ptr());
        FUN_004dcfd0(
            au_stack_10.as_mut_ptr(),
            p0.wrapping_add(2),
            p1.wrapping_add(2),
            p2,
            p3,
        );
        FUN_004d51b0(au_stack_10.as_mut_ptr().cast::<i32>());
        FUN_004da7c0(
            param_5,
            param_2,
            p0.wrapping_add(i_stack_2c).wrapping_add(2),
            p1.wrapping_add(2),
            DAT_00532aec,
        );
        FUN_004d51b0(i_stack_28.as_mut_ptr());
        if param_4 == DAT_00532ad8 {
            let c = *param_2.offset(DAT_0064f67c as isize);
            *param_2.offset(DAT_0064f67c as isize) = 0;
            let i_var3 = FUN_004db2f0(param_2, param_5);
            *param_2.offset(DAT_0064f67c as isize) = c;
            let i_var4 = FUN_004d5270();
            if i_var4 % 10 < 5 {
                FUN_004dd000(i_stack_28.as_mut_ptr(), param_1);
                i_stack_28[0] = i_stack_28[0]
                    .wrapping_add(i_stack_2c)
                    .wrapping_add(i_var3)
                    .wrapping_add(1);
                i_stack_28[1] = i_stack_28[1].wrapping_add(1);
                i_stack_28[2] = i_stack_28[0].wrapping_add(1);
                i_stack_28[3] = param_5.wrapping_add(i_stack_28[1]);
                FUN_004dd3e0(i_stack_28.as_mut_ptr(), 0, 0, DAT_00532aec as i16, 1);
            }
        }
        u_stack_18
    }
}
