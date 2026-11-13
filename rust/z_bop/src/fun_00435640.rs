// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ed228: u8;
    static mut DAT_009ed22c: u8;
    static mut DAT_009ed22d: u8;
    static mut DAT_009d8e25: [i8; 1];
    static mut DAT_009d8e29: [i8; 1];
    static mut DAT_009d6990: [i32; 1];
    static mut DAT_009e96dd: [i32; 1];
    static mut DAT_009ecc34: i32;
    static mut DAT_00a07be0: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00435a10(a: u8, b: u8, c: u8) -> u32;
    fn FUN_00450650(a: i32, b: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00435640(
    param_1: u16,
    param_2: u32,
    param_3: i32,
    param_4: u16,
    param_5: i32,
    param_6: i32,
) -> i32 {
    let mut param_4 = param_4;
    // SAFETY: all globals and callees belong to the game; pointers are
    // computed and accessed exactly as the original does.
    unsafe {
        let u_var2 = FUN_00435a10(DAT_009ed228, DAT_009ed22c, DAT_009ed22d);
        let u_var6 = param_2 & 0xffff;
        let mut i_var4: i32 = 0;
        let mut local_10: i32 = 0;
        let row = u_var6.wrapping_mul(0x566) as i32;
        let mut pc: *const i8 = (&raw mut DAT_009d8e25 as *const i8).byte_add(row as usize);
        loop {
            let idx = i_var4.wrapping_add(param_6).wrapping_add(row);
            let flag = (&raw mut DAT_009d8e29 as *const i8).offset(idx as isize);
            let b0 = *pc;
            let b1 = *pc.add(1) as u8;
            let b2 = *pc.add(2);
            let m = (*pc.add(0xe) as u8 as u32).wrapping_mul(5);
            if b0 == 2
                && ((b1 as u16 == param_1 && b2 == 0x12 && *flag != 0)
                    || (b0 == 2
                        && b1 as u16 == param_1
                        && b2 == 0x13
                        && *flag != 0
                        && param_5 == 1))
                && (m == 0 || u_var2 <= m)
            {
                let mut i_var5: i32 = (*pc.add(3) as i32).wrapping_mul(0xfa);
                if 10 < param_4 {
                    param_4 = 9;
                }
                if 1 < param_4 {
                    let i_var3: i32 = if i_var5 < 1 {
                        (param_4 as i32).wrapping_sub(1)
                    } else {
                        1i32.wrapping_sub(param_4 as u32 as i32)
                    };
                    i_var5 = i_var5.wrapping_add(i_var3.wrapping_mul(i_var5 / 10));
                }
                if param_3 == -1 {
                    let p = (&raw mut DAT_009d6990 as *mut i32).offset(param_6 as isize);
                    p.write_unaligned(p.read_unaligned().wrapping_add(i_var5));
                } else {
                    let p = (&raw mut DAT_009e96dd as *mut u8)
                        .offset(param_3.wrapping_mul(0x5bd) as isize)
                        .cast::<i32>();
                    p.write_unaligned(p.read_unaligned().wrapping_add(i_var5));
                }
                local_10 = local_10.wrapping_add(i_var5);
            }
            pc = pc.add(0x4e);
            i_var4 = i_var4.wrapping_add(0x4e);
            if !(i_var4 < 0x270) {
                break;
            }
        }
        if local_10 != 0 && param_3 == DAT_009ecc34 {
            DAT_00a07be0 = local_10 as i16;
            if local_10 < 1 {
                DAT_00a07be0 = DAT_00a07be0.wrapping_neg();
                i_var4 = 200;
            } else {
                i_var4 = 199;
            }
            FUN_00450650(i_var4, param_3);
        }
        local_10
    }
}
