// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523570: [i8; 0];
    static mut DAT_0081b740: [i8; 0];
    static mut DAT_0081bcf0: [i8; 0];
    static mut DAT_005320d0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dfc80(param_1: *const i8) -> *mut u32;
    fn FUN_004dfea0(param_1: *mut u32);
    fn FUN_004be160(param_1: *mut i8, param_2: *mut i8) -> *mut core::ffi::c_void;
    fn FUN_004be320(param_1: *mut core::ffi::c_void, param_2: *mut core::ffi::c_void, param_3: u32);
    fn FUN_004be1f0(param_1: *mut core::ffi::c_void);
    fn __strcmpi(param_1: *const i8, param_2: *const i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004c0640(param_1: *mut i8) -> u32 {
    // SAFETY: the callees and globals are the game's own; the list returned
    // by FUN_004dfc80 holds a head pointer and a count, and each node holds
    // a string pointer and a next pointer, read where the original does.
    unsafe {
        let pu_var1 = FUN_004dfc80((&raw mut DAT_005320d0).cast::<i8>());
        if pu_var1.is_null() {
            return 0;
        }
        if param_1.is_null() {
            FUN_004dfea0(pu_var1);
            return 0;
        }
        if param_1.read() == 0 {
            FUN_004dfea0(pu_var1);
            return 0;
        }
        let mut pu_var4 = pu_var1.read_unaligned() as usize as *mut u32;
        let mut i_var5: i32 = 0;
        let mut local_4: u32 = 0;
        if 0 < (pu_var1.add(1).read_unaligned() as i32) {
            loop {
                let mut p_var2 = FUN_004be160(
                    pu_var4.read_unaligned() as usize as *mut i8,
                    (&raw mut DAT_00523570).cast::<i8>(),
                );
                if !p_var2.is_null() {
                    FUN_004be320(p_var2, (&raw mut DAT_0081b740).cast(), 0xe);
                    FUN_004be1f0(p_var2);
                    let i_var3 = __strcmpi((&raw mut DAT_0081b740).cast::<i8>(), param_1);
                    if i_var3 == 0 {
                        p_var2 = FUN_004be160(
                            pu_var4.read_unaligned() as usize as *mut i8,
                            (&raw mut DAT_00523570).cast::<i8>(),
                        );
                        FUN_004be320(p_var2, (&raw mut DAT_0081bcf0).cast(), 0x3df3a);
                        local_4 = 1;
                        FUN_004be1f0(p_var2);
                        break;
                    }
                }
                pu_var4 = pu_var4.add(1).read_unaligned() as usize as *mut u32;
                i_var5 = i_var5.wrapping_add(1);
                if !(i_var5 < (pu_var1.add(1).read_unaligned() as i32)) {
                    break;
                }
            }
        }
        FUN_004dfea0(pu_var1);
        local_4
    }
}
