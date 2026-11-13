// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00aa6cfc: *mut core::ffi::c_void;
    static mut DAT_00aa7b1f: i32;
    static mut DAT_00aa7b1b: i32;
    static mut DAT_00aa7b17: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d48e0() -> *mut core::ffi::c_void;
    fn FUN_004d4e20();
    fn FUN_004d4970();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d4ec0(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dca20() -> u32 {
    // SAFETY: the globals and callees are the game's own; the source and
    // destination buffers are the game's surfaces, copied as the original does.
    unsafe {
        let c_var1 = DAT_00aa7b0d;
        let mut pu_var2 = FUN_004d48e0() as *mut u32;
        FUN_004d4e20();
        let mut i_var3: i32 = 0x1e0;
        let mut pu_var5 = DAT_00aa6cfc as *mut u32;
        loop {
            let mut pu_var6 = pu_var5;
            let mut pu_var7 = pu_var2;
            let mut i_var4: i32 =
                ((DAT_00aa7b1f & -8i32).wrapping_mul(5) & 0xfffffff).wrapping_shl(2);
            while i_var4 != 0 {
                pu_var7.write_unaligned(pu_var6.read_unaligned());
                pu_var6 = pu_var6.add(1);
                pu_var7 = pu_var7.add(1);
                i_var4 = i_var4.wrapping_sub(1);
            }
            pu_var5 = pu_var5.byte_offset(DAT_00aa7b1b as isize);
            pu_var2 = pu_var2.byte_offset(DAT_00aa7b17 as isize);
            i_var3 = i_var3.wrapping_sub(1);
            if i_var3 == 0 {
                break;
            }
        }
        FUN_004d4ec0(0);
        if c_var1 == 0 {
            FUN_004d4970();
        }
        1
    }
}
