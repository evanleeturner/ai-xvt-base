// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a10514: i32;
    static mut DAT_00a606dc: i32;
    static mut DAT_00a604d0: i32;
    static mut DAT_00a60530: i32;
    static mut DAT_00a604d4: i32;
    static mut DAT_00a604cc: i32;
    static mut DAT_00a606d8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042dda0(param_1: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042d600(param_1: *mut i8) -> u8 {
    // SAFETY: the globals and the callee are the game's own memory; param_1
    // is the caller's string, and the table entries and the object they
    // point at are read and written where the original does.
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
        let mut i_var5: i32 = DAT_00a606dc.wrapping_add(1);
        let mut i_var4: i32 = 0;
        let mut pi_var2: *mut i32 = &raw mut DAT_00a604d0;
        let end: *mut i32 = &raw mut DAT_00a60530;
        loop {
            if pi_var2.byte_sub(4).read_unaligned() == i_var1 {
                let v = pi_var2.read_unaligned();
                if v < i_var5 {
                    i_var3 = i_var4;
                    i_var5 = v;
                }
            }
            pi_var2 = pi_var2.byte_add(12);
            i_var4 = i_var4.wrapping_add(1);
            if !(pi_var2 < end) {
                break;
            }
        }
        if i_var3 == -1 {
            return 0;
        }
        let off = (i_var3 as isize).wrapping_mul(12);
        let slot = (&raw mut DAT_00a604d4).byte_offset(off);
        let obj = slot.read_unaligned() as usize as *mut i32;
        if obj.is_null() {
            return 0;
        }
        let vt = obj.read_unaligned();
        let f1 = vt.wrapping_add(0x48) as usize as *const core::ffi::c_void;
        let f1: unsafe extern "C" fn(*mut i32) -> i32 = core::mem::transmute(f1);
        let r = f1(obj);
        let f2 = obj.read_unaligned().wrapping_add(8) as usize as *const core::ffi::c_void;
        let f2: unsafe extern "C" fn(*mut i32) = core::mem::transmute(f2);
        f2(obj);
        slot.write_unaligned(0);
        (&raw mut DAT_00a604cc).byte_offset(off).write_unaligned(-1);
        DAT_00a606d8 = DAT_00a606d8.wrapping_sub(1);
        (-1 < r) as u8
    }
}
