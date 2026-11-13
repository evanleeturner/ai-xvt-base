// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8d4a: u16;
    static mut DAT_009a8d4e: u16;
    static mut DAT_009a7b48: i16;
    static mut DAT_009a7b40: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00421ae0(param_1: i32, param_2: *mut u16) {
    // SAFETY: the globals are the game's own memory; param_1 and param_2
    // point at the caller's data, read at the same offsets as the original.
    unsafe {
        let u_var2: u16 = DAT_009a8d4e;
        let u_var1: u16 = DAT_009a8d4a;
        if (DAT_009a8d4a as i16) < 0 {
            DAT_009a8d4a = DAT_009a8d4a.wrapping_neg();
        }
        if (DAT_009a8d4e as i16) < 0 {
            DAT_009a8d4e = DAT_009a8d4e.wrapping_neg();
        }
        let p0: u32 = param_2.read_unaligned() as u32;
        let p204: u32 = param_2.add(0x204).read_unaligned() as u32;
        let p205: u32 = param_2.add(0x205).read_unaligned() as u32;
        let w2: u32 = ((param_1.wrapping_add(2)) as *const u16).read_unaligned() as u32;
        let w4: u16 = ((param_1.wrapping_add(4)) as *const u16).read_unaligned();
        let w6: u32 = ((param_1.wrapping_add(6)) as *const u16).read_unaligned() as u32;
        let w8: u16 = ((param_1.wrapping_add(8)) as *const u16).read_unaligned();

        let u_var6: u32 = ((p0
            .wrapping_mul((DAT_009a8d4a as i16 as i32) as u32)
            .wrapping_add(0x80) as i32)
            >> 8) as u32
            & 0xffff;
        let mut i_var3: i32 = w6.wrapping_mul(u_var6) as i32;
        if ((w8 ^ u_var1) & 0x8000) != 0 {
            i_var3 = i_var3.wrapping_neg();
        }
        let t: i32 = (p205
            .wrapping_mul((DAT_009a8d4e as i16 as i32) as u32)
            .wrapping_add(0x80) as i32)
            >> 8;
        let u_var5: u32 =
            ((((t as u32).wrapping_mul(p0)).wrapping_add(0x80) as i32) >> 8) as u32 & 0xffff;
        let mut i_var4: i32 = w2.wrapping_mul(u_var5) as i32;
        if ((w4 ^ u_var2) & 0x8000) != 0 {
            i_var4 = i_var4.wrapping_neg();
        }
        DAT_009a7b48 = ((i_var3.wrapping_add(0x8000).wrapping_add(i_var4) as u32) >> 16) as i16;

        i_var3 = w2.wrapping_mul(u_var6) as i32;
        if ((w4 ^ u_var1) & 0x8000) != 0 {
            i_var3 = i_var3.wrapping_neg();
        }
        i_var4 = w6.wrapping_mul(u_var5) as i32;
        if ((w8 ^ u_var2) & 0x8000) == 0 {
            i_var4 = i_var4.wrapping_neg();
        }
        i_var4 = i_var3.wrapping_add(0x8000).wrapping_add(i_var4);
        DAT_009a7b40 = ((i_var4 as u32) >> 16) as i16;
        if i_var4 < 0 {
            DAT_009a7b40 = DAT_009a7b40.wrapping_neg();
        }
        DAT_009a7b40 = ((p204
            .wrapping_mul((DAT_009a7b40 as i32) as u32)
            .wrapping_add(0x80))
            >> 8) as i16;
        if i_var4 < 0 {
            DAT_009a7b40 = DAT_009a7b40.wrapping_neg();
        }
    }
}
