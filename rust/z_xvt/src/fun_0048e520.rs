// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn __strcmpi(param_1: *mut i8, param_2: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048e520(param_1: *mut u32, param_2: *mut i8) -> *mut u32 {
    if param_1.is_null() {
        return core::ptr::null_mut();
    }
    // SAFETY: param_1 points at the game's node structure; its fields, the
    // child array and the callees are the game's own memory.
    unsafe {
        let name = param_1.read_unaligned() as *mut i8;
        if !name.is_null() {
            let r = __strcmpi(name, param_2);
            if r == 0 {
                return param_1;
            }
        }
        let mut off: i32 = 0;
        let mut idx: i32 = 0;
        if 0 < (param_1.byte_add(8).read_unaligned() as i32) {
            loop {
                let children = param_1.byte_add(12).read_unaligned() as usize;
                let child = ((children as *const u8).byte_add(off as usize) as *const u32)
                    .read_unaligned() as *mut u32;
                let r = FUN_0048e520(child, param_2);
                if !r.is_null() {
                    return r;
                }
                off = off.wrapping_add(4);
                idx = idx.wrapping_add(1);
                if !(idx < (param_1.byte_add(8).read_unaligned() as i32)) {
                    break;
                }
            }
        }
        core::ptr::null_mut()
    }
}
