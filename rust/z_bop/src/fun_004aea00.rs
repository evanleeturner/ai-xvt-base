// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [u8; 0];
    static mut DAT_009a7ed0: [i16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> *mut i32;
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
pub unsafe extern "C" fn FUN_004aea00(param_1: i32, param_2: i32) -> i32 {
    let mut param_2 = param_2;
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: the globals, the callees and the object pointers are the game's
    // own memory, read and written where the original does.
    unsafe {
        let flag = (&raw mut DAT_00521f19)
            .cast::<u8>()
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if (flag & 1) == 0 {
            return 0;
        }
        let w = (&raw mut DAT_009a7ed0)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read() as u16 as u32;
        let hi = ((param_1.wrapping_mul(5) >> 16) & 0xFFFF) as u32;
        let key = ((hi << 16) | w) as i32;
        let piv1 = FUN_004ac720(key);
        if piv1.read() != piv1 as usize as i32 {
            FUN_00471ff0(piv1);
        }
        let p10 = piv1.byte_add(10).cast::<*mut i32>().read_unaligned();
        let inner = p10.read_unaligned();
        if ((inner as u32 as usize + 4) as *const i32).read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let val6 = piv1.byte_add(6).cast::<i32>().read_unaligned();
        if val6 <= param_2 {
            param_2 = val6.wrapping_sub(1);
        }
        let mut u_var3: i32 = 0;
        let elem = p10.offset(param_2 as isize).read_unaligned();
        let i_var2 = FUN_004ae1a0(elem, piv1);
        if i_var2 != 0 {
            let l = __ftol();
            u_var3 = l as i32;
        }
        FUN_004ac740();
        u_var3
    }
}
