// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_0052686c: i8;
    static mut DAT_0052b157: i8;
    static mut DAT_0052b159: i8;
    static mut DAT_0052b15b: i8;
    static mut DAT_0052b15d: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004428e0(param_1: i32) -> u32 {
    // SAFETY: the globals and the records they index are the game's own
    // memory; every read is at the offset the original uses.
    unsafe {
        let i_var2 = param_1.wrapping_mul(0x23).wrapping_add(DAT_006d4988);
        let tmp_int = ((i_var2.wrapping_add(0x1f)) as u32 as *const i32).read_unaligned();
        if tmp_int == 0 {
            let b = ((i_var2.wrapping_add(3)) as u32 as *const u8).read();
            let p = (&raw mut DAT_0052686c)
                .cast::<u8>()
                .byte_add((b as usize).wrapping_mul(0x16));
            return u32::from(p.cast::<u16>().read_unaligned());
        }
        let i_var1 = ((tmp_int.wrapping_add(0xa7)) as u32 as *const i32).read_unaligned();
        if i_var1 != 0 {
            let b2 = ((i_var1.wrapping_add(4)) as u32 as *const u8).read();
            let off = (b2 as usize).wrapping_mul(0xef);
            let s1 = (&raw mut DAT_0052b159)
                .cast::<u8>()
                .byte_add(off)
                .cast::<i16>()
                .read_unaligned();
            let s2 = (&raw mut DAT_0052b15b)
                .cast::<u8>()
                .byte_add(off)
                .cast::<i16>()
                .read_unaligned();
            let s3 = (&raw mut DAT_0052b15d)
                .cast::<u8>()
                .byte_add(off)
                .cast::<i16>()
                .read_unaligned();
            let sum = (s1 as i32).wrapping_add(s2 as i32).wrapping_add(s3 as i32);
            let div_result = sum / 3;
            let b3 = (&raw mut DAT_0052b157).cast::<u8>().byte_add(off).read();
            return div_result.wrapping_shl((b3 & 0x1f) as u32) as u32;
        }
        let b = ((i_var2.wrapping_add(3)) as u32 as *const u8).read();
        let p = (&raw mut DAT_0052686c)
            .cast::<u8>()
            .byte_add((b as usize).wrapping_mul(0x16));
        u32::from(p.cast::<u16>().read_unaligned())
    }
}
