// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [u8; 1];
    static mut DAT_009a7ed0: [i16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> i32;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_004add10(param_1: i32) -> i32;
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004af000(param_1: i32, mut param_2: i32) -> u32 {
    // SAFETY: the globals and callees are the game's own memory; the pointers
    // returned by the game's functions are read at the offsets the original uses.
    unsafe {
        let flags = (&raw mut DAT_00521f19)
            .cast::<u8>()
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if flags & 1 == 0 {
            return 0;
        }
        let s = (&raw mut DAT_009a7ed0)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read();
        let arg = ((((param_1.wrapping_mul(5)) as u32) >> 16) << 16) | (s as u16 as u32);
        let p1 = FUN_004ac720(arg as i32) as usize as *mut i32;
        if p1.read_unaligned() != p1 as usize as i32 {
            FUN_00471ff0(p1);
        }
        let arr = p1.byte_add(10).read_unaligned() as usize as *mut i32;
        if arr.byte_add(4).read_unaligned() != 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let cnt = p1.byte_add(6).read_unaligned();
        if cnt <= param_2 {
            param_2 = cnt.wrapping_sub(1);
        }
        let mut u_var3: u32 = 0;
        let elem = arr.offset(param_2 as isize).read_unaligned();
        let i_var2 = FUN_004add10(elem);
        if i_var2 != 0 {
            u_var3 = (i_var2.wrapping_add(0x14) as usize as *const u32).read_unaligned();
        }
        FUN_004ac740();
        u_var3
    }
}
