// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071f2db: u8;
    static mut DAT_0071f2dc: u8;
    static mut DAT_0071f2dd: u8;
    static mut DAT_0070aee5: [i8; 1];
    static mut DAT_0070aee9: [i8; 1];
    static mut DAT_00709310: [i32; 1];
    static mut DAT_0071b79d: [i8; 1];
    static mut DAT_0071ecf4: i32;
    static mut DAT_00732950: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a1d70(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a1590(
    param_1: u16,
    param_2: u32,
    param_3: i32,
    param_4: u16,
    param_5: i32,
    param_6: i32,
) -> i32 {
    let mut param_4 = param_4;
    // SAFETY: all globals and the callee belong to the game; pointers are
    // computed with the original's byte offsets and accessed as the C does.
    unsafe {
        let u_var4: u32 = DAT_0071f2db as u32;
        let u_var2: u32 = DAT_0071f2dc as u32;
        let mut local_10: i32 = 0;
        let u_var5: u32 = param_2 & 0xffff;
        let u_var3: u32 = DAT_0071f2dd as u32;
        let mut i_var7: i32 = 0;
        let base5 = (&raw mut DAT_0070aee5) as u32;
        let base9 = (&raw mut DAT_0070aee9) as u32;
        let mut pc: u32 = base5.wrapping_add(u_var5.wrapping_mul(0x566));
        loop {
            let p = pc as *const i8;
            let b0 = p.read();
            let b1 = p.byte_add(1).read() as u8;
            let b2 = p.byte_add(2).read();
            let idx = (i_var7 as u32)
                .wrapping_add(param_6 as u32)
                .wrapping_add(u_var5.wrapping_mul(0x566));
            let flag = (base9.wrapping_add(idx) as *const i8).read();
            let b0e = (p.byte_add(0xe).read() as u8) as u32;
            if b0 == 0x02
                && ((b1 as u32 == param_1 as u32 && b2 == 0x12 && flag != 0)
                    || (b0 == 0x02
                        && b1 as u32 == param_1 as u32
                        && b2 == 0x13
                        && flag != 0
                        && param_5 == 1))
                && (b0e.wrapping_mul(5) == 0
                    || u_var3.wrapping_add(
                        u_var4
                            .wrapping_mul(0x3c)
                            .wrapping_add(u_var2)
                            .wrapping_mul(0x3c),
                    ) <= b0e.wrapping_mul(5))
            {
                let mut i_var8: i32 = (p.byte_add(3).read() as i32).wrapping_mul(0xfa);
                if 10 < param_4 {
                    param_4 = 9;
                }
                if 1 < param_4 {
                    let i_var6: i32 = if i_var8 < 1 {
                        (param_4 as i32).wrapping_sub(1)
                    } else {
                        1i32.wrapping_sub(param_4 as i32)
                    };
                    i_var8 = i_var8.wrapping_add(i_var6.wrapping_mul(i_var8 / 10));
                }
                if param_3 == -1 {
                    let q = ((&raw mut DAT_00709310) as *mut i32).offset(param_6 as isize);
                    q.write(q.read().wrapping_add(i_var8));
                } else {
                    let q = ((&raw mut DAT_0071b79d) as *mut i8)
                        .byte_offset(param_3.wrapping_mul(0x5bd) as isize)
                        .cast::<i32>();
                    let v = q.read_unaligned().wrapping_add(i_var8);
                    q.write_unaligned(v);
                }
                local_10 = local_10.wrapping_add(i_var8);
            }
            pc = pc.wrapping_add(0x4e);
            i_var7 = i_var7.wrapping_add(0x4e);
            if !(i_var7 < 0x270) {
                break;
            }
        }
        if local_10 != 0 && param_3 == DAT_0071ecf4 {
            DAT_00732950 = local_10 as i16;
            let code: i32;
            if local_10 < 1 {
                DAT_00732950 = DAT_00732950.wrapping_neg();
                code = 199;
            } else {
                code = 0xc6;
            }
            FUN_004a1d70(code, param_3);
        }
        local_10
    }
}
