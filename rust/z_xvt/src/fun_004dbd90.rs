// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759645: i8;
    static mut DAT_00758830: *mut core::ffi::c_void;
    static mut DAT_00759657: i32;
    static mut DAT_00759653: i32;
    static mut DAT_0075964f: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4d00() -> *mut core::ffi::c_void;
    fn FUN_004d52b0();
    fn FUN_004d4d90();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d5350(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dbd90() -> i32 {
    // SAFETY: the globals and callees are the game's own; the pointers are
    // the game's surface buffers, copied dword by dword as the original does.
    unsafe {
        let c_var1 = DAT_00759645;
        let mut pu_var2 = FUN_004d4d00() as *mut i32;
        FUN_004d52b0();
        let mut i_var5: i32 = 0x1e0;
        let mut pu_var3 = DAT_00758830 as *mut i32;
        loop {
            let mut pu_var6 = pu_var3;
            let mut pu_var7 = pu_var2;
            let mut i_var4: i32 =
                ((((DAT_00759657 & 0xfffffff8u32 as i32).wrapping_mul(5)) & 0xfffffff) << 2);
            while i_var4 != 0 {
                pu_var7.write_unaligned(pu_var6.read_unaligned());
                pu_var6 = pu_var6.wrapping_add(1);
                pu_var7 = pu_var7.wrapping_add(1);
                i_var4 = i_var4.wrapping_sub(1);
            }
            pu_var3 = pu_var3.wrapping_byte_offset(DAT_00759653 as isize);
            pu_var2 = pu_var2.wrapping_byte_offset(DAT_0075964f as isize);
            i_var5 = i_var5.wrapping_sub(1);
            if i_var5 == 0 {
                break;
            }
        }
        FUN_004d5350(0);
        if c_var1 == 0 {
            FUN_004d4d90();
        }
        1
    }
}
