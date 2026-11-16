// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d8ccb: u8;
    static mut DAT_009d8c83: u8;
    static mut DAT_009d8c87: u8;
    static mut DAT_009d8c30: u8;
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9683: u8;
    static mut DAT_009e9670: u8;
    static mut DAT_009e9a63: u8;
    static mut DAT_009e9a5f: u8;
    static mut DAT_009e9a5b: u8;
    static mut DAT_009e967a: u8;
    static mut DAT_0051c56a: u8;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_009a8c00: u32;
    static mut DAT_00a07be0: u16;
    static mut DAT_00a08292: u16;
    static mut DAT_00a07be6: i16;
    static mut DAT_00a07be4: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00458e20(param_1: u32, param_2: u32);
    fn FUN_0046a8d0(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_00451bf0(param_1: i32, param_2: u32);
    fn FUN_00450650(param_1: i32, param_2: i32);
}

#[inline(always)]
unsafe fn at(p: *mut u8, off: i32) -> *mut u8 {
    // SAFETY: byte offset within the game's own memory.
    unsafe { p.byte_offset(off as isize) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00451940(param_1: u32, param_2: u32) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are read where the original reads them.
    unsafe {
        let u_var7: u32 = param_1 & 0xffff;
        let i_var1: i32 = u_var7.wrapping_mul(0x566) as i32;
        let mut u_var5: u16 = DAT_00a07cec;
        if at(&raw mut DAT_009d8ccb, i_var1).read() == 0 {
            FUN_00458e20(0x8000, u_var7);
        } else {
            while (u_var5 as i32) < DAT_009a1fe0 {
                let i_var8 = (u_var5 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_009a1fe8 as u32);
                let p_entry = i_var8 as usize as *mut u8;
                if p_entry.byte_add(3).read() != 0
                    && (p_entry.byte_add(0x16).read() as u32) == u_var7
                {
                    let mut p_ptr: u32 = p_entry.byte_add(0x1f).cast::<u32>().read_unaligned();
                    p_ptr = (p_ptr.wrapping_add(0xa9) as usize as *const u32).read_unaligned();
                    let p_inner = p_ptr as usize as *const u8;
                    if (p_inner.byte_add(5).read() as i8) == -1 {
                        FUN_00458e20(u_var5 as u32, u_var7);
                        break;
                    }
                }
                u_var5 = u_var5.wrapping_add(1);
            }
        }
        let u_off: u32 = (DAT_009ecc34 as u32).wrapping_mul(0x5bd);
        let off = u_off as i32;
        let i_var3: i32;
        let i_var4: i32;
        let i_var8: i32;
        if at(&raw mut DAT_009e9683, off).read() == 0 {
            let i_tmp = at(&raw mut DAT_009e9670, off)
                .cast::<i32>()
                .read_unaligned();
            let base = (i_tmp as u32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_009a1fe8 as u32) as usize;
            i_var3 = ((base + 0xc) as *const i32).read_unaligned();
            i_var4 = ((base + 8) as *const i32).read_unaligned();
            i_var8 = ((base + 4) as *const i32).read_unaligned();
        } else {
            i_var3 = at(&raw mut DAT_009e9a63, off)
                .cast::<i32>()
                .read_unaligned();
            i_var4 = at(&raw mut DAT_009e9a5f, off)
                .cast::<i32>()
                .read_unaligned();
            i_var8 = at(&raw mut DAT_009e9a5b, off)
                .cast::<i32>()
                .read_unaligned();
        }
        FUN_0046a8d0(
            DAT_00a07cdc.wrapping_sub(i_var8),
            DAT_00a07cd8.wrapping_sub(i_var4),
            DAT_00a07cd4.wrapping_sub(i_var3),
        );
        DAT_009a8c00 = DAT_009a8c00.wrapping_mul(0xa1);
        let mut s_var6: i16 = (((DAT_009a8c00 >> 0x10) as u64 + 0x32) / 100) as i16;
        if s_var6 == 0 {
            s_var6 = 1;
        }
        let b_var2: u8 = at(&raw mut DAT_009d8c83, i_var1).read();
        DAT_00a07be0 = b_var2 as u16;
        DAT_00a08292 = at(&raw mut DAT_009d8c87, i_var1).read() as u16;
        let off2 = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32;
        let us_tmp: u16 = at(&raw mut DAT_009e967a, off2)
            .cast::<u16>()
            .read_unaligned();
        let off3 = (param_2 & 0xffff).wrapping_mul(0x101) as i32;
        let u_tmp: u32 = at(&raw mut DAT_0051c56a, off3)
            .cast::<u32>()
            .read_unaligned();
        let addr30 = (&raw mut DAT_009d8c30) as usize as u32;
        if us_tmp == DAT_00a08292 {
            if DAT_00a07be0 != 1 {
                FUN_00451bf0(1, u_tmp);
                FUN_00451bf0(2, addr30.wrapping_add(i_var1 as u32));
                DAT_00a07be6 = s_var6;
                FUN_00450650(0xec, DAT_009ecc34);
                return;
            }
            FUN_00451bf0(0, u_tmp);
            FUN_00451bf0(1, addr30.wrapping_add(i_var1 as u32));
            DAT_00a07be4 = s_var6;
            FUN_00450650(0xeb, DAT_009ecc34);
            return;
        }
        FUN_00451bf0(1, u_tmp);
        if b_var2 != 1 {
            DAT_00a07be4 = s_var6;
            FUN_00450650(0x73, DAT_009ecc34);
            return;
        }
        DAT_00a07be4 = s_var6;
        FUN_00450650(0x72, DAT_009ecc34);
    }
}
