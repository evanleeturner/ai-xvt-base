// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_006d4980: i32;
    static mut _DAT_00732a3c: i32;
    static mut unaff_ESI: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040ed40(param_1: *mut u8, param_2: u16, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040f050(param_1: u16) {
    // SAFETY: the globals and the record table they describe are the game's
    // own memory; reads and writes happen at the offsets the original uses.
    unsafe {
        let base = DAT_006d4988;
        let mut ptr_0x1f: i32 = ((base
            .wrapping_add(0x1f)
            .wrapping_add((param_1 as u32).wrapping_mul(0x23) as i32))
            as *const i32)
            .read_unaligned();
        if ptr_0x1f != 0 {
            (ptr_0x1f.wrapping_add(0x73) as *mut i32).write_unaligned(0);
            (ptr_0x1f.wrapping_add(0x12) as *mut u8).write(0);
            return;
        }
        if _DAT_00732a3c < DAT_006d4980 {
            let mut i_var3 = _DAT_00732a3c;
            while i_var3 < DAT_006d4980 {
                let i_var1 = DAT_006d4988.wrapping_add(i_var3.wrapping_mul(0x23));
                let val_0x03 = (i_var1.wrapping_add(3) as *const i8).read();
                let val_0x1b = (i_var1.wrapping_add(0x1b) as *const i32).read_unaligned();
                ptr_0x1f = (i_var1.wrapping_add(0x1f) as *const i32).read_unaligned();
                if val_0x03 != 0 && val_0x1b != -1 && ptr_0x1f != 0 {
                    FUN_0040ed40(
                        ptr_0x1f.wrapping_add(0x12) as *mut u8,
                        i_var3 as u16,
                        (((unaff_ESI as u32) >> 16) << 16) | (param_1 as u32),
                    );
                }
                i_var3 += 1;
            }
        }
    }
}
