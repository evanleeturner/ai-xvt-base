// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7fb7: i32;
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00aa7fc3: i32;
    static mut DAT_00aa6cfc: i32;
    static mut unaff_ESI: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004df1b0(param_1: *mut i8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970();
    fn FUN_004d48e0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004deca0(param_1: *mut i8) -> i32 {
    // SAFETY: the globals, callees and the table at DAT_00aa7fc3 are the
    // game's own memory; param_1 is the caller's string, read as the
    // original does.
    unsafe {
        if DAT_00aa7fb7 == 0 {
            return 0;
        }
        if param_1.read() == 0 {
            return 0;
        }
        let i_var2 = FUN_004df1b0(param_1);
        let c_var1 = DAT_00aa7b0d;
        if i_var2 == -1 {
            return 0;
        }
        let mut i_var6: i32 = -1;
        let mut i_var4: i32 = 0;
        let mut i_var5: i32 = -1;
        let base = DAT_00aa7fc3 as usize as *const u8;
        let mut p = base.byte_add(4);
        loop {
            let key = p.byte_sub(4).cast::<i32>().read_unaligned();
            let val = p.cast::<i32>().read_unaligned();
            if key == i_var2 && i_var6 < val {
                i_var5 = i_var4;
                i_var6 = val;
            }
            p = p.byte_add(12);
            i_var4 = i_var4.wrapping_add(1);
            if i_var4 >= 0xc {
                break;
            }
        }
        if i_var5 == -1 {
            return 0;
        }
        FUN_004d4970();
        let off = 8usize.wrapping_add((i_var5 as usize).wrapping_mul(0xc));
        let obj = (DAT_00aa7fc3 as usize).wrapping_add(off) as *const u8;
        let obj = obj.cast::<u32>().read_unaligned() as usize as *mut i32;
        let vtable = obj.read_unaligned() as usize as *const u8;
        let fp = vtable.byte_add(0x18).cast::<u32>().read_unaligned() as usize;
        let f: unsafe extern "stdcall" fn(*mut i32, *mut i8) -> i32 =
            core::mem::transmute(fp as *const core::ffi::c_void);
        let mut local_4: [i8; 4] = [0; 4];
        let r = f(obj, local_4.as_mut_ptr());
        if r != 0 {
            return 0;
        }
        if c_var1 != 0 {
            DAT_00aa6cfc = FUN_004d48e0();
        }
        (unaff_ESI.wrapping_mul(0x7f) / 2000).wrapping_add(0x7f)
    }
}
