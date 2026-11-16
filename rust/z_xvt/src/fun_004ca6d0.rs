// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007806fa: *mut *mut u32;
    static mut DAT_00780716: u32;
    static mut DAT_0078071a: u32;
    static mut DAT_0078071e: u32;
    static mut DAT_00780722: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ca780(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ca6d0(param_1: u32, param_2: *mut u32) -> u32 {
    let mut param_2 = param_2;
    let mut local_50: [u32; 0x14] = [0; 0x14];
    // SAFETY: the callee, globals and COM object are the game's own; param_2
    // points at four u32 values supplied by the caller or by FUN_004ca780.
    unsafe {
        if param_2.is_null() {
            param_2 = FUN_004ca780(param_1) as usize as *mut u32;
            if param_2.is_null() {
                return 0;
            }
        }
        local_50[0] = 0x50;
        local_50[1] = 0x40;
        local_50[2] = param_2.read_unaligned();
        local_50[3] = param_2.add(1).read_unaligned();
        local_50[4] = param_2.add(2).read_unaligned();
        local_50[5] = param_2.add(3).read_unaligned();
        let obj = DAT_007806fa;
        let slot = (*obj).add(0x60) as *const *const core::ffi::c_void;
        let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void, *mut u32, i32) -> i32 =
            core::mem::transmute(slot.read_unaligned());
        let r = f(obj as *mut core::ffi::c_void, local_50.as_mut_ptr(), 1);
        if r != 0 {
            return 0;
        }
        DAT_00780716 = param_2.read_unaligned();
        DAT_0078071a = param_2.add(1).read_unaligned();
        DAT_0078071e = param_2.add(2).read_unaligned();
        DAT_00780722 = param_2.add(3).read_unaligned();
        1
    }
}
