// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a10514: i32;
    static mut DAT_00a604d0: i32;
    static mut DAT_00a60530: i32;
    static mut DAT_00a604d4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042dda0(param_1: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042d7c0(param_1: *mut i8, param_2: i32) -> i32 {
    let mut param_2 = param_2;
    // SAFETY: the globals and the callee are the game's own memory; param_1
    // points at the caller's string, and the table and the object it names
    // are read where the original reads them.
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
        let mut pi_var2: *mut i32 = (&raw mut DAT_00a604d0).cast::<i32>();
        let end: *mut i32 = (&raw mut DAT_00a60530).cast::<i32>();
        loop {
            if pi_var2.wrapping_sub(1).read_unaligned() == i_var1
                && i_var3 < pi_var2.read_unaligned()
            {
                i_var3 = pi_var2.read_unaligned();
                i_var4 = i_var5;
            }
            pi_var2 = pi_var2.wrapping_add(3);
            i_var5 = i_var5.wrapping_add(1);
            if !(pi_var2 < end) {
                break;
            }
        }
        if i_var4 == -1 {
            return 0;
        }
        if param_2 < 0x80 {
            if param_2 < 0 {
                param_2 = 0;
            }
        } else {
            param_2 = 0x7f;
        }
        let p_obj_val: i32 = (&raw mut DAT_00a604d4)
            .cast::<i32>()
            .wrapping_offset((i_var4.wrapping_mul(3)) as isize)
            .read_unaligned();
        let p_obj = p_obj_val as u32 as usize as *mut i32;
        let addr = p_obj.read_unaligned().wrapping_add(0x3c);
        let vfunc: unsafe extern "stdcall" fn(*mut i32, i32) -> i32 =
            core::mem::transmute(addr as u32 as usize as *const core::ffi::c_void);
        let arg = (param_2.wrapping_mul(5).wrapping_sub(0x27b)).wrapping_mul(400) / 0x7f;
        let r = vfunc(p_obj, arg);
        (r == 0) as i32
    }
}
