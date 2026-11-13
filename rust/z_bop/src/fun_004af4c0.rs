// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [u8; 0];
    static mut DAT_009a7ed0: [u16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> *mut core::ffi::c_void;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_004af150(param_1: *mut u32, param_2: i32, param_3: i32) -> i32;
    fn __ftol() -> i64;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004af4c0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: *mut u32,
    param_5: *mut u32,
    param_6: *mut i32,
    param_7: *mut u32,
) {
    let mut param_2 = param_2;
    // SAFETY: the globals, callees and pointers are the game's own memory;
    // every read and write happens where the original does it.
    unsafe {
        let flags = (&raw mut DAT_00521f19)
            .cast::<u8>()
            .byte_add(param_1.wrapping_mul(0x1a) as usize)
            .read();
        if (flags & 1) != 0 {
            let hi = ((param_1.wrapping_mul(5) as u32) >> 16) as u16 as i16 as i32;
            let lo = (&raw mut DAT_009a7ed0)
                .cast::<u16>()
                .add(param_1 as usize)
                .read_unaligned() as i32;
            let concat_val = (hi << 16) | lo;
            let pi_var1 = FUN_004ac720(concat_val) as *mut i32;
            if pi_var1.read_unaligned() as usize as *mut i32 != pi_var1 {
                FUN_00471ff0(pi_var1);
            }
            let list = pi_var1.byte_add(10).cast::<i32>().read_unaligned();
            let first = (list as usize as *const i32).read_unaligned();
            if ((first as usize + 4) as *const i32).read_unaligned() == 0x14 {
                param_2 = param_2.wrapping_add(1);
            }
            let count = pi_var1.byte_add(6).cast::<i32>().read_unaligned();
            if count <= param_2 {
                param_2 = count.wrapping_sub(1);
            }
            let list = pi_var1.byte_add(10).cast::<i32>().read_unaligned();
            let elem = (list as usize as *const i32)
                .offset(param_2 as isize)
                .read_unaligned();
            let i_var2 = FUN_004af150(elem as usize as *mut u32, pi_var1 as usize as i32, param_3);
            if i_var2 == 0 {
                param_4.write_unaligned(0);
                param_5.write_unaligned(0);
                param_6.write_unaligned(0);
                param_7.write_unaligned(0);
            } else {
                let p = ((i_var2 as usize).wrapping_add(0x14) as *const i32).read_unaligned();
                param_4.write_unaligned((p as usize as *const u32).read_unaligned());
                let l3 = __ftol();
                param_5.write_unaligned(l3 as u32);
                let l3 = __ftol();
                param_6.write_unaligned((l3 as i32).wrapping_neg());
                let l3 = __ftol();
                param_7.write_unaligned(l3 as u32);
            }
            FUN_004ac740();
        }
    }
}
