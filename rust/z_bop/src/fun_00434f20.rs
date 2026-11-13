// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009fd458: [u8; 0];
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009d69e0: [i16; 0];
    static mut DAT_009d69cc: [i16; 0];
    static mut DAT_009d69b8: [i16; 0];
    static mut DAT_009d6954: u8;
    static mut DAT_009d6990: [i16; 0];
    static mut unaff_EBX: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00435a40(param_1: u32) -> u32;
    fn FUN_00435640(
        param_1: i32,
        param_2: u32,
        param_3: i32,
        param_4: u16,
        param_5: i32,
        param_6: i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00434f20(param_1: u32, param_2: i32, param_3: i32, param_4: i32) {
    // SAFETY: all globals and callees are the game's own memory; indices
    // are used exactly as the original computes them.
    unsafe {
        let u_var4: u32 = param_1 & 0xffff;
        let mut u_var5: u16 = 1;
        let b_var1: u8 = DAT_009a1fe8
            .add(u_var4.wrapping_mul(0x23).wrapping_add(0x16) as usize)
            .read();

        let inner: u8 = (&raw mut DAT_009d8c88)
            .cast::<u8>()
            .add((b_var1 as u32).wrapping_mul(0x566) as usize)
            .read();
        let idx = param_4.wrapping_mul(0x1e5).wrapping_add(inner as i32) as isize;
        let flag: u8 = (&raw mut DAT_009fd458).cast::<u8>().offset(idx).read();

        let p4 = param_4 as isize;
        if flag == 0 {
            let mut i_var2: i32 = FUN_00435a40(u_var4) as i32;
            if param_3 == 1 {
                let p = (&raw mut DAT_009d69e0).cast::<i16>().offset(p4);
                p.write_unaligned(p.read_unaligned().wrapping_add(1));
                u_var5 = 10;
                i_var2 /= 10;
            } else if param_3 == 2 {
                let p = (&raw mut DAT_009d69cc).cast::<i16>().offset(p4);
                p.write_unaligned(p.read_unaligned().wrapping_add(1));
                u_var5 = 6;
                i_var2 /= 2;
            } else if param_3 == 3 {
                let p = (&raw mut DAT_009d69b8).cast::<i16>().offset(p4);
                p.write_unaligned(p.read_unaligned().wrapping_add(1));
                u_var5 = 1;
            }
            if DAT_009d6954 == 2 {
                i_var2 = i_var2.wrapping_mul(2);
            } else if DAT_009d6954 == 0 {
                i_var2 /= 2;
            }
            let arg2 = (((unaff_EBX >> 8) << 8) | b_var1 as u32) & 0xffff00ff;
            let i_var3 = FUN_00435640(2, arg2, -1, u_var5, param_2, param_4);
            if -1 < i_var3 {
                let p = (&raw mut DAT_009d6990).cast::<i16>().offset(p4);
                let v = (p.read_unaligned() as i32).wrapping_add(i_var2) as i16;
                p.write_unaligned(v);
                return;
            }
        } else if param_3 == 2 || param_3 == 3 {
            let i_var2 = FUN_00435a40(u_var4) as i32;
            let p = (&raw mut DAT_009d6990).cast::<i16>().offset(p4);
            let v = (p.read_unaligned() as i32).wrapping_sub(i_var2) as i16;
            p.write_unaligned(v);
        }
    }
}
