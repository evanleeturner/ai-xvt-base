// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a7ed0: [u16; 0];
    static mut DAT_00521f19: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> i32;
    fn FUN_00471ff0(param_1: i32);
    fn FUN_004ae1a0(param_1: i32, param_2: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ae200(param_1: i32, mut param_2: i32) -> i32 {
    // SAFETY: the globals and callees are the game's own; the pointers
    // read here are the game's records, accessed at the original offsets.
    unsafe {
        let w = (&raw mut DAT_009a7ed0)
            .cast::<u16>()
            .offset(param_1 as isize)
            .read();
        if w == 0 {
            return 0;
        }
        if param_2 < 0 {
            return 0;
        }
        let flag = (&raw mut DAT_00521f19)
            .cast::<u8>()
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if flag & 1 == 0 {
            return 0;
        }
        let p = FUN_004ac720(w as i32);
        FUN_004ac740();
        let pp = p as u32 as usize as *const u8;
        if pp.cast::<i32>().read_unaligned() != p {
            FUN_00471ff0(p);
        }
        let ptr_at_10 = pp.byte_add(10).cast::<u32>().read_unaligned();
        let val_at_6 = pp.byte_add(6).cast::<i32>().read_unaligned();
        let base = (ptr_at_10 as usize as *const u32).read_unaligned() as usize as *const u8;
        if base.byte_add(16).cast::<i32>().read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        if val_at_6 <= param_2 {
            param_2 = val_at_6.wrapping_sub(1);
        }
        let elem = base
            .byte_offset((param_2 as isize).wrapping_mul(4))
            .cast::<i32>()
            .read_unaligned();
        FUN_004ae1a0(elem, p)
    }
}
