// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a90b1c: u32;
    static mut _DAT_00a90b10: u32;
    static mut _DAT_00a90b14: u32;
    static mut _DAT_00a90b18: u32;
    static mut DAT_00528c5c: u32;
    static mut _DAT_00a90b70: u32;
    static mut _DAT_00a90b90: u32;
    static mut _DAT_00a90bb0: u32;
    static mut _DAT_00a90bd0: u32;
    static mut DAT_00a90b60: u32;
    static mut DAT_00a90b30: u32;
    static mut DAT_00528c94: u32;
    static mut DAT_00528c6c: u32;
    static mut _DAT_00a90b40: u32;
    static mut _DAT_00a90b54: u32;
    static mut DAT_00a90a70: u32;
    static mut DAT_00a90a74: u32;
    static mut _DAT_00a90ae8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __ftol() -> i64;
    fn FUN_004b1d10(param_1: *mut core::ffi::c_void, param_2: u32);
    fn FUN_004b1de0(param_1: *mut core::ffi::c_void, param_2: u32);
    fn FUN_004b1a90(param_1: *mut core::ffi::c_void, param_2: u32);
    fn FUN_004b2680(param_1: *mut core::ffi::c_void, param_2: u32, param_3: u32, param_4: u32);
    fn FUN_004b3160(param_1: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004b1c10();
    fn FUN_004b1c90();
    fn FUN_004b1d70();
    fn FUN_004b2020();
    fn FUN_004b1c50();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b3590() {
    // SAFETY: all globals and callees are the game's own memory and code; the
    // pointers formed from game globals are read where the original reads them.
    unsafe {
        if DAT_00a90b1c != 0
            && ((_DAT_00a90b10 & 0x7fffffff) != 0
                || (_DAT_00a90b14 & 0x7fffffff) != 0
                || (_DAT_00a90b18 & 0x7fffffff) != 0)
        {
            let l_var2 = __ftol();
            let l_var3 = __ftol();
            let l_var4 = __ftol();
            let l_var5 = __ftol();
            let b_var1 = l_var5 as u8;
            if b_var1 != 0 {
                let flag = ((DAT_00528c5c.wrapping_add(0x18)) as *const i32).read_unaligned();
                if flag != 0 {
                    _DAT_00a90b70 = (((b_var1 as u32) << 0x10 | (l_var3 as u8 as u32)) << 8)
                        | ((l_var2 as u8 as u32) << 0x10)
                        | (l_var4 as u8 as u32);
                    _DAT_00a90b90 = _DAT_00a90b70;
                    _DAT_00a90bb0 = _DAT_00a90b70;
                    _DAT_00a90bd0 = _DAT_00a90b70;
                    FUN_004b1c10();
                    FUN_004b1c90();
                    FUN_004b1d10((&raw mut DAT_00a90b60).cast(), 4);
                    FUN_004b1d70();
                    FUN_004b1de0((&raw mut DAT_00a90b30).cast(), 2);
                    FUN_004b2020();
                    FUN_004b1c50();
                    return;
                }
                let base = DAT_00528c6c;
                let rb = |off: u32| -> u32 {
                    ((base.wrapping_add(off)) as *const u8).read() as u32 & 0x1f
                };
                let g = (((l_var3 as u8 as u32) >> rb(0x24)) as u8 as u32) << rb(0x18);
                let r = (((l_var2 as u8 as u32) >> rb(0x20)) as u8 as u32) << rb(0x14);
                let a = ((b_var1 as u32) >> rb(0x34)) << rb(0x30);
                let b = (((l_var4 as u8 as u32) >> rb(0x28)) as u8 as u32) << rb(0x1c);
                let color = ((g | r | a | b) as u16) as u32;
                FUN_004b1a90(DAT_00528c94 as *mut core::ffi::c_void, color);
                FUN_004b1c10();
                FUN_004b1c90();
                FUN_004b2680(DAT_00528c94 as *mut core::ffi::c_void, 0xa90a70, 0, 1);
                _DAT_00a90b70 = 0xffffffff;
                _DAT_00a90b90 = 0xffffffff;
                _DAT_00a90bb0 = 0xffffffff;
                _DAT_00a90bd0 = 0xffffffff;
                _DAT_00a90b40 = (&raw mut DAT_00a90a70) as u32;
                _DAT_00a90b54 = (&raw mut DAT_00a90a70) as u32;
                FUN_004b1d10((&raw mut DAT_00a90b60).cast(), 4);
                FUN_004b1d70();
                FUN_004b1de0((&raw mut DAT_00a90b30).cast(), 2);
                FUN_004b2020();
                FUN_004b1c50();
                let obj1 = DAT_00a90a74;
                let f1 = (obj1 as *const u32).read_unaligned().wrapping_add(8);
                let f1: unsafe extern "stdcall" fn(*mut core::ffi::c_void) =
                    core::mem::transmute(f1);
                f1(obj1 as *mut core::ffi::c_void);
                let obj0 = DAT_00a90a70;
                let f0 = (obj0 as *const u32).read_unaligned().wrapping_add(8);
                let f0: unsafe extern "stdcall" fn(*mut core::ffi::c_void) =
                    core::mem::transmute(f0);
                f0(obj0 as *mut core::ffi::c_void);
                _DAT_00a90ae8 = 0;
                FUN_004b3160(0xa90a70);
            }
        }
    }
}
