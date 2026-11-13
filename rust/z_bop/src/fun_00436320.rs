// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ec47c: u32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_00521f1c: u32;
    static mut DAT_009ecc34: u32;
    static mut DAT_009a8e34: u32;
    static mut DAT_00a07ce8: u32;
    static mut DAT_009d1308: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00436310();
    fn FUN_00436470(param_1: u32, param_2: u32, param_3: u32) -> i32;
    fn FUN_0044ff10(param_1: u32, param_2: u32) -> i32;
    fn FUN_004362a0(param_1: u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00436320() {
    // SAFETY: the globals and callees are the game's own; the table pointers
    // are read at the offsets the original uses.
    unsafe {
        let mut u_var5: u32 = 0;
        FUN_00436310();
        if DAT_009ec47c != 0 {
            let mut i_var6: i32 = 0;
            loop {
                let base = DAT_009a1fe8.wrapping_add(i_var6 as u32);
                let u_var3: u32 = u32::from((base.wrapping_add(3) as *const u8).read());
                if u_var3 != 0 {
                    let type_val: u8 = (base.wrapping_add(2) as *const u8).read();
                    let tmp: u32 = (&raw mut DAT_00521f1c)
                        .cast::<u8>()
                        .byte_add(u_var3.wrapping_mul(0x1a) as usize)
                        .cast::<u32>()
                        .read_unaligned();
                    let i_var4: i32 = match type_val {
                        0..=5 => FUN_00436470(u_var5, tmp, DAT_009ecc34),
                        6 | 7 | 0xb | 0xd => FUN_0044ff10(u_var5, tmp),
                        _ => 0,
                    };
                    if i_var4 != 0 {
                        FUN_004362a0(u_var5, DAT_009a8e34);
                    }
                }
                i_var6 = i_var6.wrapping_add(0x23);
                u_var5 = u_var5.wrapping_add(1);
                if !(u_var5 < DAT_009ec47c) {
                    break;
                }
            }
        }
        if DAT_00a07ce8 < DAT_009d1308.wrapping_add(DAT_00a07ce8) {
            let mut i_var6: i32 = (DAT_00a07ce8 as i32).wrapping_mul(0x23);
            u_var5 = DAT_00a07ce8;
            loop {
                let base = DAT_009a1fe8.wrapping_add(i_var6 as u32);
                let u_var3: u32 = u32::from((base.wrapping_add(3) as *const u8).read());
                if u_var3 != 0 {
                    let b_var1: u8 = (base.wrapping_add(2) as *const u8).read();
                    if b_var1 > 7 && b_var1 < 10 {
                        let tmp: u32 = (&raw mut DAT_00521f1c)
                            .cast::<u8>()
                            .byte_add(u_var3.wrapping_mul(0x1a) as usize)
                            .cast::<u32>()
                            .read_unaligned();
                        if FUN_00436470(u_var5, tmp, DAT_009ecc34) != 0 {
                            FUN_004362a0(u_var5, DAT_009a8e34);
                        }
                    }
                }
                i_var6 = i_var6.wrapping_add(0x23);
                u_var5 = u_var5.wrapping_add(1);
                if !(u_var5 < DAT_009d1308.wrapping_add(DAT_00a07ce8)) {
                    break;
                }
            }
        }
    }
}
