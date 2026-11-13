// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [u8; 0];
    static mut DAT_009a7ed0: [i16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: u32) -> u32;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_004ae1a0(param_1: i32, param_2: *mut i32) -> i32;
    fn __ftol() -> i64;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004aed40(param_1: i32, mut param_2: i32) -> u32 {
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: the globals and callees are the game's own; the pointers read
    // below are the game's own structures, accessed where the original does.
    unsafe {
        let flag = (&raw mut DAT_00521f19)
            .cast::<u8>()
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if flag & 1 == 0 {
            return 0;
        }
        let w = (&raw mut DAT_009a7ed0)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read() as u16;
        let arg = ((param_1.wrapping_mul(5) as u32) >> 16) << 16 | (w as u32);
        let p_var1 = FUN_004ac720(arg) as *mut i32;
        if p_var1.read() != p_var1 as i32 {
            FUN_00471ff0(p_var1);
        }
        let arr = p_var1.byte_add(0xA).cast::<i32>().read_unaligned() as *const i32;
        let first = arr.read_unaligned();
        let v = (first.wrapping_add(4) as *const i32).read_unaligned();
        if v == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let count = p_var1.byte_add(0x6).cast::<i32>().read_unaligned();
        if count <= param_2 {
            param_2 = count.wrapping_sub(1);
        }
        let arr = p_var1.byte_add(0xA).cast::<i32>().read_unaligned() as *const i32;
        let elem = arr.offset(param_2 as isize).read_unaligned();
        let i_var2 = FUN_004ae1a0(elem, p_var1);
        let u_var3: u32;
        if i_var2 == 0 {
            u_var3 = 0;
        } else if ((i_var2.wrapping_add(0x38)) as *const i32).read_unaligned() == 0 {
            u_var3 = __ftol() as u32;
        } else {
            u_var3 = __ftol() as u32;
        }
        FUN_004ac740();
        u_var3
    }
}
