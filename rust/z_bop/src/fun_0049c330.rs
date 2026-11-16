// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523434: i32;
    static mut DAT_00a07bd8: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049e720(
        param_1: *mut i8,
        param_2: *mut core::ffi::c_void,
        param_3: i32,
        param_4: i32,
    ) -> i32;
    fn _fgetc(stream: *mut core::ffi::c_void) -> i32;
    fn FUN_0049e9c0(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049c330(param_1: *mut i8, param_2: *mut u8, param_3: i32) -> i32 {
    let mut param_2 = param_2;
    // SAFETY: the callees and globals are the game's own; param_2 and param_3
    // point at caller buffers, and the FILE pointer is the game's stream,
    // accessed exactly as the original does.
    unsafe {
        let mut i_var3 = FUN_0049e720(
            param_1,
            (&raw mut DAT_00523434).cast::<core::ffi::c_void>(),
            1,
            0,
        );
        let file = DAT_00a07bd8;
        if i_var3 != 0 {
            let mut s_var4: i16 = 0;
            let pi_var1 = file.cast::<u8>().byte_add(0xc).cast::<i32>();
            let mut u_var2 = pi_var1.read_unaligned() as u32;
            while (u_var2 & 0x10) == 0 {
                let slot = param_3.wrapping_add((s_var4 as i32).wrapping_mul(4)) as u32 as usize
                    as *mut *mut u8;
                slot.write_unaligned(param_2);
                i_var3 = _fgetc(file);
                u_var2 = pi_var1.read_unaligned() as u32;
                while (u_var2 & 0x10) == 0 && (i_var3 as i16 as i32) != 0xff {
                    param_2.write(i_var3 as u8);
                    param_2 = param_2.add(1);
                    i_var3 = _fgetc(file);
                    u_var2 = pi_var1.read_unaligned() as u32;
                }
                s_var4 = s_var4.wrapping_add(1);
                param_2.write(0xff);
                param_2 = param_2.add(1);
                u_var2 = pi_var1.read_unaligned() as u32;
            }
            FUN_0049e9c0(0);
            return s_var4 as i32;
        }
        0
    }
}
