// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f440: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004df760(a: u8, b: u8, c: u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d3030(param_1: *mut u8, param_2: i32) -> u32 {
    // SAFETY: DAT_0064f440 is the game's own table of i16 entries, indexed
    // by param_2 as the original does; param_1 points at the caller's three
    // bytes.
    unsafe {
        let slot = (&raw mut DAT_0064f440)
            .cast::<u8>()
            .byte_offset(param_2.wrapping_mul(2) as isize)
            .cast::<i16>();
        let s_var1 = slot.read_unaligned();
        let mut u_var2 = s_var1 as i32 as u32;
        if 0xff < s_var1 {
            if param_2 == 0 {
                return u_var2 & 0xffff_ff00;
            }
            let a = param_1.byte_add(2).read();
            let b = param_1.byte_add(1).read();
            let c = param_1.read();
            u_var2 = FUN_004df760(a, b, c);
            slot.write_unaligned(u_var2 as i16);
        }
        u_var2
    }
}
