// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1834: u16;
    static mut DAT_00521f2e: i8;
    static mut DAT_00524170: i8;
    static mut DAT_009d8c82: i8;
    static mut DAT_009a20c0: i8;
    static mut DAT_009d8c94: i8;
    static mut DAT_009d695b: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00456190(param_1: u16);
    fn FUN_004587c0(param_1: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00456040() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the indexed reads and writes use the same offsets as the original.
    unsafe {
        let idx = DAT_009a1834 as u32;
        let a = (&raw mut DAT_009d8c82)
            .cast::<u8>()
            .byte_add(idx.wrapping_mul(0x566) as usize)
            .read();
        let b = (&raw mut DAT_00524170)
            .cast::<u8>()
            .byte_add(a as usize)
            .read();
        let c = (&raw mut DAT_00521f2e)
            .cast::<u8>()
            .byte_add((b as u32).wrapping_mul(0x1a) as usize)
            .read();
        if (c & 0x80) == 0 {
            FUN_004587c0(0xffff);
        } else {
            FUN_00456190(0xffff);
        }

        let u_var3 = DAT_009a1834 as u32;
        let pc = (&raw mut DAT_009a20c0).byte_add(u_var3.wrapping_mul(0x126) as usize + 1);
        let c_var2 = pc.read();
        if c_var2 != 0 {
            let flag = (&raw mut DAT_009d8c94)
                .byte_add(u_var3.wrapping_mul(0x566) as usize)
                .read();
            if flag == 0 {
                pc.write(c_var2.wrapping_sub(1));
            } else if DAT_009d695b != 2 {
                pc.write(c_var2.wrapping_sub(1));
                return;
            }
        }
    }
}
