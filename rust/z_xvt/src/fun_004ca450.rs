// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759645: i8;
    static mut DAT_007806fa: *mut i32;
    static mut DAT_0078075a: i32;
    static mut DAT_00780782: i32;
    static mut DAT_00780732: i32;
    static mut DAT_0078077a: i32;
    static mut DAT_00758830: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4d90();
    fn FUN_004d4d00() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ca450() -> i32 {
    // SAFETY: all globals, callees and the COM-style object at DAT_007806fa
    // are the game's own memory; the local buffer holds the 0x140 words
    // copied and is indexed within bounds.
    unsafe {
        let c_var1 = DAT_00759645;
        if DAT_007806fa.is_null() {
            return 0;
        }
        FUN_004d4d90();
        let mut buf = [0i32; 320];
        let mut src = &raw mut DAT_0078075a;
        for i in 0..0x140usize {
            buf[i] = src.read_unaligned();
            src = src.add(1);
        }
        let mut dst = &raw mut DAT_00780782;
        for _ in 0..0x136 {
            dst.write_unaligned(0);
            dst = dst.add(1);
        }
        let mut i_var3: i32 = 0;
        let obj = DAT_007806fa;
        let vtable = obj.read_unaligned() as u32 as usize;
        let fp = ((vtable + 0x30) as *const u32).read_unaligned();
        let f: unsafe extern "stdcall" fn(*mut i32, i32, *mut i32, i32, i32) =
            core::mem::transmute(fp as usize as *const core::ffi::c_void);
        f(obj, 0, 0x004ca520usize as *mut i32, 0, 0);
        if 0 < DAT_00780732 {
            let mut pi4 = &raw mut DAT_0078077a;
            loop {
                let mut i_var2: usize = 0;
                let mut idx: usize = 3;
                loop {
                    if buf[idx] == pi4.read_unaligned() {
                        pi4.add(1).write_unaligned(buf[4 + i_var2 * 10]);
                        break;
                    }
                    idx += 10;
                    i_var2 += 1;
                    if idx >= 320 {
                        break;
                    }
                }
                pi4 = pi4.add(10);
                i_var3 = i_var3.wrapping_add(1);
                if i_var3 >= DAT_00780732 {
                    break;
                }
            }
        }
        if c_var1 != 0 {
            DAT_00758830 = FUN_004d4d00();
        }
        1
    }
}
