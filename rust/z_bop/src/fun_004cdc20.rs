// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00acebc5: *mut *mut i32;
    static mut DAT_00acec25: i32;
    static mut DAT_00acec4d: i32;
    static mut DAT_00acebfd: i32;
    static mut DAT_00acec45: i32;
    static mut DAT_00aa6cfc: i32;
    static mut LAB_004cdcf0: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970() -> i32;
    fn FUN_004d48e0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004cdc20() -> i32 {
    let mut local_500: [i32; 0x140] = [0; 0x140];
    // SAFETY: all globals and callees are the game's own memory; the copy and
    // clear loops stay within the game's buffers at the original's counts, and
    // local indices stay below 0x140.
    unsafe {
        let c_var1 = DAT_00aa7b0d;
        if DAT_00acebc5.is_null() {
            return 0;
        }
        FUN_004d4970();
        let mut pi_var4: *mut i32 = &raw mut DAT_00acec25;
        let mut i = 0usize;
        while i < 0x140 {
            local_500[i] = pi_var4.read_unaligned();
            pi_var4 = pi_var4.add(1);
            i += 1;
        }
        let mut pu_var5: *mut i32 = &raw mut DAT_00acec4d;
        let mut n = 0x136;
        while n != 0 {
            pu_var5.write_unaligned(0);
            pu_var5 = pu_var5.add(1);
            n -= 1;
        }
        DAT_00acebfd = 1;
        let mut i_var3: i32 = 0;
        let f: unsafe extern "stdcall" fn(*mut *mut i32, i32, *mut i32, i32, i32) -> i32 =
            core::mem::transmute(
                (*DAT_00acebc5).add(12).read_unaligned() as usize as *const core::ffi::c_void
            );
        f(DAT_00acebc5, 0, &raw mut LAB_004cdcf0, 0, 0);
        if DAT_00acebfd > 0 {
            pi_var4 = &raw mut DAT_00acec45;
            loop {
                let mut i_var2: usize = 0;
                let mut idx: usize = 3;
                loop {
                    if local_500[idx] == pi_var4.read_unaligned() {
                        pi_var4.add(1).write_unaligned(local_500[4 + i_var2 * 10]);
                        break;
                    }
                    idx += 10;
                    i_var2 += 1;
                    if idx >= 0x140 {
                        break;
                    }
                }
                pi_var4 = pi_var4.add(10);
                i_var3 = i_var3.wrapping_add(1);
                if i_var3 >= DAT_00acebfd {
                    break;
                }
            }
        }
        if c_var1 != 0 {
            DAT_00aa6cfc = FUN_004d48e0();
        }
        1
    }
}
