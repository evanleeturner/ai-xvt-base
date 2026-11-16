// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d8e27: i8;
    static mut DAT_009d8e29: i8;
    static mut DAT_009d6968: i32;
    static mut DAT_009ed22c: u8;
    static mut DAT_009ed22d: u8;
    static mut DAT_009ed228__3_1_: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00435a10(param_1: u8, param_2: u8, param_3: u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00435930(param_1: u16, param_2: u32, param_3: i32, param_4: u8) {
    // SAFETY: all globals and the callee belong to the game; the pointers walk
    // its own record table exactly as the original does.
    unsafe {
        let i_var1 = ((param_2 & 0xffff) as i32).wrapping_mul(0x566);
        let mut pc5: *mut i8 = (&raw mut DAT_009d8e29)
            .byte_offset(i_var1 as isize)
            .byte_offset(param_4 as isize);
        let mut pc3: *mut i8 = (&raw mut DAT_009d8e27).byte_offset(i_var1 as isize);
        let mut local_4: i32 = 0;
        let mut i_var4: i32 = 8;
        loop {
            if pc3.byte_offset(-2).read() == 2 {
                if (pc3.byte_offset(-1).read() as u8 as u16) == param_1 {
                    let c = pc3.read();
                    if c == 0x12 || (c == 0x13 && param_3 == 1) {
                        let u_var2 = FUN_00435a10(DAT_009ed228__3_1_, DAT_009ed22c, DAT_009ed22d);
                        let lim = (pc3.byte_offset(0xc).read() as u8 as u32).wrapping_mul(5);
                        if lim == 0 || u_var2 <= lim {
                            local_4 = pc3.byte_offset(1).read() as i32;
                        }
                        local_4 = local_4.wrapping_mul(0xfa);
                        if pc5.read() != 0 {
                            let p = (&raw mut DAT_009d6968).add(param_4 as usize);
                            p.write_unaligned(p.read_unaligned().wrapping_add(local_4));
                        }
                    }
                }
            }
            pc5 = pc5.byte_offset(0x4e);
            pc3 = pc3.byte_offset(0x4e);
            i_var4 = i_var4.wrapping_sub(1);
            if i_var4 == 0 {
                break;
            }
        }
    }
}
