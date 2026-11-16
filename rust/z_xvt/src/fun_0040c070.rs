// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e795: u8;
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: *mut u8;
    static mut DAT_00523ab0: [u8; 0];
    static mut DAT_00709b08: [u8; 0];
    static mut DAT_0071f2db: u8;
    static mut DAT_0071f2dc: u8;
    static mut DAT_0071f2dd: u8;
    static mut DAT_0073b430: [i32; 0];
    static mut DAT_0070ad48: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004784d0() -> u16;
    fn FUN_004a18c0(a: u8, b: u8, c: u8) -> i32;
    fn FUN_0040c4c0(a: u32, b: i8, c: i8, d: i8, e: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040c070(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: u16,
) -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are offset and read exactly as the original does.
    unsafe {
        if DAT_0085e795 == 0 {
            return 0;
        }
        let base = (&raw mut DAT_0071b730)
            .cast::<u8>()
            .add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize);
        let i_tmp = base.cast::<i32>().read_unaligned();
        if i_tmp == -1 {
            return 0;
        }
        if param_4 != 0xffff {
            let u_var1 = FUN_004784d0();
            if param_4 <= u_var1 {
                return 0;
            }
        }
        let mut s_var4: i16 = -1;
        let mut u_stack_8: u32 = 0;
        if param_1 == 1 {
            if param_3 == -1 || DAT_006d4980 <= param_3 {
                return 0;
            }
            let ps = DAT_006d4988.add((param_3 as u32).wrapping_mul(0x23) as usize);
            let mut us_tmp = base.add(0xc).cast::<u16>().read_unaligned();
            let idx1 =
                (ps.add(0x16).read() as u32).wrapping_add((us_tmp as u32).wrapping_mul(0x30));
            let m = (&raw mut DAT_00709b08)
                .cast::<u8>()
                .add(idx1 as usize)
                .read();
            u_stack_8 = (&raw mut DAT_00523ab0).cast::<u8>().add(m as usize).read() as u32;
            if u_stack_8 == 0xff {
                return 0;
            }
            s_var4 = ps.cast::<i16>().read_unaligned();
            let i_var2 = FUN_004a18c0(DAT_0071f2db, DAT_0071f2dc, DAT_0071f2dd);
            if param_2 != 0x1b && param_2 != 0x1c {
                let slot = (&raw mut DAT_0073b430)
                    .cast::<i32>()
                    .offset(param_3 as isize);
                let v = slot.read();
                if v != 0 && (i_var2 as u32).wrapping_sub(v as u32) < 0xb {
                    return 0;
                }
                slot.write(i_var2);
            }
            us_tmp = base.add(0xc).cast::<u16>().read_unaligned();
            if u_stack_8 == 4 || u_stack_8 == 5 {
                let b = DAT_006d4988
                    .add((0x16u32).wrapping_add((param_3 as u32).wrapping_mul(0x23)) as usize)
                    .read();
                let o = (&raw mut DAT_0070ad48)
                    .cast::<u8>()
                    .add((b as u32).wrapping_mul(0x566) as usize)
                    .read();
                if (o as u16) != us_tmp {
                    u_stack_8 = 0xbu32.wrapping_sub((u_stack_8 == 4) as u32);
                }
            }
        }
        let c_var5: i8;
        if param_1 == 1 {
            FUN_0040c4c0(u_stack_8.wrapping_add(0x2b5), 2, 1, 0, s_var4);
            c_var5 = 1;
        } else {
            if param_1 < 2 {
                return 1;
            }
            if 6 < param_1 {
                return 1;
            }
            c_var5 = 0;
        }
        FUN_0040c4c0(
            (param_2 as u32).wrapping_add(0x2b5),
            2,
            param_1 as i8,
            c_var5,
            s_var4,
        );
        1
    }
}
