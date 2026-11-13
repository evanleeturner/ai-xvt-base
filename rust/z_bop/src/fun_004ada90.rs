// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ec5fe: u16;
    static mut DAT_009a1fe8: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ada90(param_1: i32, param_2: i32) {
    // SAFETY: the globals and the object pointed to are the game's own
    // memory, read and written at the offsets the original uses.
    unsafe {
        let divisor: u64 = 0xec;
        let prod: u32 = u32::from(DAT_009ec5fe).wrapping_mul(param_2 as u32);
        let div1_num: u64 = prod as u64;
        if div1_num / divisor > 0xFFFF_FFFF {
            RaiseException(0xC000_0094, 0, 0, core::ptr::null());
        }
        let u_var4: u32 = (div1_num / divisor) as u32;

        let i_var5: i32 = param_1.wrapping_mul(0x23);
        let p_obj: i32 =
            ((DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var5)) as *const i32).read_unaligned();

        let field_7b: u16 = ((p_obj.wrapping_add(0x7b)) as *const u16).read_unaligned();

        let rem: u32 = (u_var4 as i32)
            .wrapping_mul(-0xec)
            .wrapping_add(u32::from(DAT_009ec5fe).wrapping_mul(param_2 as u32) as i32)
            as u32;
        let div2_num: u64 = (rem as u64).wrapping_mul(0x10000);
        if div2_num / divisor > 0xFFFF_FFFF {
            RaiseException(0xC000_0094, 0, 0, core::ptr::null());
        }
        let scaled_rem: u16 = (div2_num / divisor) as u16;

        let new_7b: u16 = field_7b.wrapping_sub(scaled_rem);
        ((p_obj.wrapping_add(0x7b)) as *mut u16).write_unaligned(new_7b);

        let p79 = p_obj.wrapping_add(0x79);
        if field_7b < new_7b {
            let f = (p79 as *const u16).read_unaligned();
            (p79 as *mut u16).write_unaligned(f.wrapping_sub(1));
        }

        let f = (p79 as *const u16).read_unaligned();
        (p79 as *mut u16).write_unaligned(f.wrapping_sub(u_var4 as u16));

        let f = (p79 as *const u16).read_unaligned();
        if 0x8000 < f {
            (p79 as *mut u16).write_unaligned(0);
        }
    }
}
