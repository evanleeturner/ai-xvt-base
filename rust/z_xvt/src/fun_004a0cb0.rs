// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: *mut u8;
    static mut DAT_0070ad48: [u8; 0];
    static mut DAT_0070ad42: [u8; 0];
    static mut DAT_0072f508: [u8; 0];
    static mut DAT_005247e8: [u8; 0];
    static mut DAT_0052686a: [u8; 0];
    static mut DAT_006d4a02: i8;
    static mut DAT_007092d4: i8;
    static mut DAT_00709360: [i32; 0];
    static mut DAT_0070934c: [i32; 0];
    static mut DAT_00709338: [i32; 0];
    static mut DAT_00709310: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a1980(param_1: u32) -> i32;
    fn FUN_004a1590(
        param_1: i32,
        param_2: u32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a0cb0(param_1: u32, param_2: i32, param_3: i32, param_4: i32) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // indexes are computed exactly as the original does.
    unsafe {
        let u_var5: u32 = param_1 & 0xffff;
        let mut u_var6: u16 = 1;
        let b_var1: u8 = DAT_006d4988
            .byte_add(u_var5.wrapping_mul(0x23).wrapping_add(0x16) as usize)
            .read();
        let idx_a = (b_var1 as u32).wrapping_mul(0x566);
        let t = (&raw mut DAT_0070ad48)
            .cast::<u8>()
            .byte_add(idx_a as usize)
            .read() as u32;
        let idx_b = (param_4 as u32).wrapping_mul(0x1e5).wrapping_add(t);
        let flag = (&raw mut DAT_0072f508)
            .cast::<u8>()
            .byte_add(idx_b as usize)
            .read();
        if flag == 0 {
            let b2 = (&raw mut DAT_0070ad42)
                .cast::<u8>()
                .byte_add(idx_a as usize)
                .read();
            let u_var2: u32 = (&raw mut DAT_005247e8)
                .cast::<u8>()
                .byte_add(b2 as usize)
                .read() as u32;
            let mut i_var3: i32;
            if (&raw mut DAT_0052686a)
                .cast::<u8>()
                .byte_add(u_var2.wrapping_mul(0x16) as usize)
                .read()
                == 0
            {
                i_var3 = FUN_004a1980(u_var5);
            } else {
                if (u_var2 < 0x46 || 0x4a < u_var2) && (u_var2 < 0x50 || 0x54 < u_var2) {
                    i_var3 = 0x28;
                } else {
                    i_var3 = 500;
                }
                if DAT_006d4a02 == 2 {
                    i_var3 = i_var3.wrapping_mul(3);
                }
            }
            if param_3 == 1 {
                let p = (&raw mut DAT_00709360)
                    .cast::<i32>()
                    .offset(param_4 as isize);
                p.write_unaligned(p.read_unaligned().wrapping_add(1));
                u_var6 = 10;
                i_var3 /= 10;
            } else if param_3 == 2 {
                let p = (&raw mut DAT_0070934c)
                    .cast::<i32>()
                    .offset(param_4 as isize);
                p.write_unaligned(p.read_unaligned().wrapping_add(1));
                u_var6 = 6;
                i_var3 /= 2;
            } else if param_3 == 3 {
                let p = (&raw mut DAT_00709338)
                    .cast::<i32>()
                    .offset(param_4 as isize);
                p.write_unaligned(p.read_unaligned().wrapping_add(1));
                u_var6 = 1;
            }
            if DAT_007092d4 == 2 {
                i_var3 = i_var3.wrapping_mul(2);
            } else if DAT_007092d4 == 0 {
                i_var3 /= 2;
            }
            let i_var4 = FUN_004a1590(2, b_var1 as u32, -1, u_var6 as i32, param_2, param_4);
            if -1 < i_var4 {
                let p = (&raw mut DAT_00709310)
                    .cast::<i32>()
                    .offset(param_4 as isize);
                p.write_unaligned(p.read_unaligned().wrapping_add(i_var3));
                return;
            }
        } else if param_3 == 2 || param_3 == 3 {
            let b_a = DAT_006d4988
                .byte_add(u_var5.wrapping_mul(0x23).wrapping_add(0x16) as usize)
                .read();
            let b2 = (&raw mut DAT_0070ad42)
                .cast::<u8>()
                .byte_add((b_a as u32).wrapping_mul(0x566) as usize)
                .read();
            let u_var2: u32 = (&raw mut DAT_005247e8)
                .cast::<u8>()
                .byte_add(b2 as usize)
                .read() as u32;
            let mut i_var3: i32;
            if (&raw mut DAT_0052686a)
                .cast::<u8>()
                .byte_add(u_var2.wrapping_mul(0x16) as usize)
                .read()
                == 0
            {
                i_var3 = FUN_004a1980(u_var5);
            } else {
                if (u_var2 < 0x46 || 0x4a < u_var2) && (u_var2 < 0x50 || 0x54 < u_var2) {
                    i_var3 = 0x28;
                } else {
                    i_var3 = 500;
                }
                if DAT_006d4a02 == 2 {
                    i_var3 = i_var3.wrapping_mul(3);
                }
            }
            let p = (&raw mut DAT_00709310)
                .cast::<i32>()
                .offset(param_4 as isize);
            p.write_unaligned(p.read_unaligned().wrapping_sub(i_var3));
        }
    }
}
