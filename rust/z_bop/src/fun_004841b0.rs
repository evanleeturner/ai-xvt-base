// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08104: i32;
    static mut DAT_009e968a: u8;
    static mut DAT_009e967e: u8;
    static mut DAT_009e967c: u8;
    static mut DAT_009d8c8b: u8;
    static mut DAT_009d8c88: u8;
    static mut DAT_009d9156: u8;
    static mut DAT_009d8c94: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004841b0(param_1: u32) -> u32 {
    // SAFETY: all globals and the memory they and the records point at are the
    // game's own; reads happen at the same offsets and in the same order as the original.
    unsafe {
        let i_var2: i32 = (param_1 as i32).wrapping_mul(0x5bd);
        let mut u_var3: u16 = (&raw mut DAT_009e968a)
            .byte_add(i_var2 as usize)
            .cast::<u16>()
            .read_unaligned();
        if u_var3 == 0xffff {
            return param_1 & 0xffff0000;
        }
        if DAT_009a1fe0 <= u_var3 as i32 {
            return 0;
        }
        let i_var4: i32 = (u_var3 as i32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8);
        let tmp_i: i32 = ((i_var4.wrapping_add(0x1b)) as usize as *const i32).read_unaligned();
        if tmp_i != -1 {
            return (DAT_009a1fe8 as u32) & 0xffff0000;
        }
        let tmp_u: u32 = ((i_var4.wrapping_add(0x1f)) as usize as *const u32).read_unaligned();
        let tmp_i2: i32 = (tmp_u.wrapping_add(0xa9) as usize as *const i32).read_unaligned();
        DAT_00a08104 = tmp_i2;
        if *((DAT_00a08104.wrapping_add(7)) as usize as *const i8) != 0 {
            return tmp_u & 0xffff0000;
        }
        u_var3 = ((DAT_00a08104 as u32) >> 0x10) as u16;
        let tmp_s: i16 =
            ((DAT_00a08104.wrapping_add(0x1e5)) as usize as *const i16).read_unaligned();
        if tmp_s == 0 {
            return (u_var3 as u32) << 0x10;
        }
        let rec_b: u8 = *((i_var4.wrapping_add(0x16)) as usize as *const u8);
        let u_var6: u32 = rec_b as u32;
        let tmp_us: u16 = (&raw mut DAT_009e967e)
            .byte_add(i_var2 as usize)
            .cast::<u16>()
            .read_unaligned();
        let u_var5: u32 = tmp_us as u32;
        if u_var6 == u_var5 {
            return ((u_var3 as u32) << 0x10) | 1;
        }
        let off6: usize = u_var6.wrapping_mul(0x566) as usize;
        let off5: usize = u_var5.wrapping_mul(0x566) as usize;
        let b_var1: u8 = (&raw mut DAT_009d8c8b).byte_add(off6).read();
        u_var3 = (rec_b / 0x5f) as u16;
        if b_var1 == 0 {
            return (u_var3 as u32) << 0x10;
        }
        let tmp_us2: u16 = (&raw mut DAT_009e967c)
            .byte_add(i_var2 as usize)
            .cast::<u16>()
            .read_unaligned();
        if (&raw mut DAT_009d8c88).byte_add(off6).read() as u32 == tmp_us2 as u32 {
            return ((u_var3 as u32) << 0x10) | 1;
        }
        let a6: u8 = (&raw mut DAT_009d9156).byte_add(off6).read();
        if a6 != 0 && (&raw mut DAT_009d9156).byte_add(off5).read() == a6 {
            return ((u_var3 as u32) << 0x10) | 1;
        }
        if (b_var1 as u32).wrapping_sub((&raw mut DAT_009d8c94).byte_add(off5).read() as u32) == 8 {
            return 1;
        }
        ((b_var1 as u32).wrapping_sub(tmp_us2 as u32) == 1) as u32
    }
}
