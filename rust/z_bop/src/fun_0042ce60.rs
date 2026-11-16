// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a10514: *mut u32;
    static mut DAT_00a10518: u32;
    static mut DAT_00a1065c: u32;
    static mut DAT_00a5fbe4: u32;
    static mut DAT_00a604cc: u32;
    static mut DAT_00a6052c: u32;
    static mut DAT_00a606dc: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0042d020();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042ce60() -> u32 {
    // SAFETY: all globals and callees are the game's own memory; the object
    // pointer and its vtable are read and called as the original does.
    unsafe {
        if DAT_00a10514.is_null() {
            return 1;
        }
        FUN_0042d020();
        let obj = DAT_00a10514;
        let vtable = obj.read_unaligned() as usize;
        let slot = ((vtable.wrapping_add(8)) as *const u32).read_unaligned() as usize;
        let f: unsafe extern "stdcall" fn(*mut u32) =
            core::mem::transmute(slot as *const core::ffi::c_void);
        f(obj);
        DAT_00a10514 = core::ptr::null_mut();

        let end1 = &raw mut DAT_00a5fbe4 as usize;
        let mut p = &raw mut DAT_00a1065c as usize;
        loop {
            (p as *mut u32).write_unaligned(0);
            (p.wrapping_sub(0x50) as *mut u8).write(0);
            p = p.wrapping_add(0x145);
            if !(p < end1) {
                break;
            }
        }

        let end2 = &raw mut DAT_00a6052c as usize;
        let mut q = &raw mut DAT_00a604cc as usize;
        loop {
            (q as *mut u32).write_unaligned(0xffffffff);
            (q.wrapping_add(4) as *mut u32).write_unaligned(0);
            (q.wrapping_add(8) as *mut u32).write_unaligned(0);
            q = q.wrapping_add(12);
            if !(q < end2) {
                break;
            }
        }

        DAT_00a10518 = 0;
        DAT_00a606dc = 0;
        1
    }
}
