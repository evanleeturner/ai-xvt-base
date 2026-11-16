// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a10514: i32;
    static mut DAT_00a604d0: i32;
    static mut DAT_00a604d4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042dda0(param_1: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042dac0(param_1: *mut i8, param_2: u32) -> u8 {
    // SAFETY: the globals, the callee and the table are the game's own memory;
    // param_1 points at the caller's string, and the object's first dword plus
    // 0x44 is called as the original does.
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
        let mut i_var5: i32 = -1;
        let mut i_var3: i32 = -1;
        let base = &raw mut DAT_00a604d0;
        let mut i_var4: i32 = 0;
        while i_var4 < 8 {
            let p = base.offset((i_var4 as isize) * 3);
            let prev = p.offset(-1).read_unaligned();
            let cur = p.read_unaligned();
            if prev == i_var1 && i_var5 < cur {
                i_var3 = i_var4;
                i_var5 = cur;
            }
            i_var4 += 1;
        }
        if i_var3 == -1 {
            return 0;
        }
        let obj = (&raw mut DAT_00a604d4)
            .offset((i_var3 as isize) * 3)
            .read_unaligned();
        let first = (obj as u32 as usize as *const i32).read_unaligned();
        let addr = first.wrapping_add(0x44) as u32 as usize as *const core::ffi::c_void;
        let f: unsafe extern "stdcall" fn(i32, u32) -> i32 = core::mem::transmute(addr);
        let r = f(obj, param_2);
        (r == 0) as u8
    }
}
