// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528c8c: i32;
    static mut DAT_00528c90: i32;
    static mut _DAT_00528c88: i32;
    static mut DAT_00528c5c: i32;
    static mut DAT_00528c84: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b3070() {
    // SAFETY: all globals and the linked list nodes are the game's own memory;
    // pointers are read and written exactly where the original does, and the
    // vtable slot called is the one the original calls.
    unsafe {
        let mut p2 = DAT_00528c8c as *mut i32;
        while !p2.is_null() {
            let p1 = p2.add(1).read_unaligned() as *mut i32;
            if !p1.is_null() {
                let vt = p1.read_unaligned() as *const i32;
                let f = vt.add(8).read_unaligned() as usize as *const core::ffi::c_void;
                let f: unsafe extern "C" fn(*mut i32) = core::mem::transmute(f);
                f(p1);
                p2.add(1).write_unaligned(0);
            }
            let p1 = p2.read_unaligned() as *mut i32;
            if !p1.is_null() {
                let vt = p1.read_unaligned() as *const i32;
                let f = vt.add(8).read_unaligned() as usize as *const core::ffi::c_void;
                let f: unsafe extern "C" fn(*mut i32) = core::mem::transmute(f);
                f(p1);
                p2.write_unaligned(0);
            }
            p2.add(0x1e).write_unaligned(0);
            p2.add(0x23).write_unaligned(0);
            let next = p2.add(0x25).read_unaligned() as *mut i32;
            p2.add(0x25).write_unaligned(0);
            p2.add(0x24).write_unaligned(0);
            p2 = next;
        }
        DAT_00528c8c = 0;
        DAT_00528c90 = 0;
        _DAT_00528c88 = 0;
        let base = DAT_00528c5c as usize as *mut u8;
        let v = base.byte_add(0x148).cast::<i32>().read_unaligned();
        base.byte_add(0x14c).cast::<i32>().write_unaligned(v);
        DAT_00528c84 = 1;
    }
}
