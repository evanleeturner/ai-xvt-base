// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a10514: i32;
    static mut DAT_00a604d0: i32;
    static mut DAT_00a604d4: i32;
    static mut DAT_00a60530: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042dda0(param_1: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042d940(param_1: *mut i8, mut param_2: i32) -> u8 {
    // SAFETY: the globals, the callee and the object table are the game's own
    // memory; param_1 points at the caller's string, and the table entries are
    // read and called exactly as the original does.
    unsafe {
        if DAT_00a10514 == 0 {
            return 0;
        }
        if param_1.read() == 0 {
            return 0;
        }
        let i_var1 = FUN_0042dda0(param_1);
        if i_var1 == -1 {
            return 0;
        }
        let mut i_var3: i32 = -1;
        let mut i_var5: i32 = 0;
        let mut i_var4: i32 = -1;
        let mut pi_var2: *mut i32 = &raw mut DAT_00a604d0;
        let end: *mut i32 = &raw mut DAT_00a60530;
        loop {
            if pi_var2.sub(1).read_unaligned() == i_var1 && i_var3 < pi_var2.read_unaligned() {
                i_var3 = pi_var2.read_unaligned();
                i_var4 = i_var5;
            }
            pi_var2 = pi_var2.add(3);
            i_var5 = i_var5.wrapping_add(1);
            if !(pi_var2 < end) {
                break;
            }
        }
        if i_var4 == -1 {
            return 0;
        }
        if 0x7f < param_2 {
            param_2 = 0x7f;
        }
        if param_2 < 0 {
            param_2 = 0;
        }
        let obj = (&raw mut DAT_00a604d4)
            .offset((i_var4 as isize) * 3)
            .read_unaligned();
        let fptr = (obj as usize as *const u8)
            .byte_add(0x40)
            .cast::<u32>()
            .read_unaligned();
        let f: unsafe extern "stdcall" fn(*mut i32, i32) -> i32 =
            core::mem::transmute(fptr as usize as *const core::ffi::c_void);
        let arg = (param_2.wrapping_mul(5).wrapping_sub(0x13b)).wrapping_mul(400) / 0x3f;
        let r = f(obj as usize as *mut i32, arg);
        (r == 0) as u8
    }
}
