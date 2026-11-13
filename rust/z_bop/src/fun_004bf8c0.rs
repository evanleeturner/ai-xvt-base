// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051a854: i8;
    static mut DAT_00b69d20: i8;
    static mut s___plt_0052b35c: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ccf50();
    fn FUN_004dfa10(param_1: *mut i8) -> *mut u32;
    fn FUN_004ccf70();
    fn FUN_004dfc30(param_1: *mut u32);
    fn FUN_004cc4a0(param_1: *mut i8, param_2: *mut i8) -> *mut core::ffi::c_void;
    fn FUN_004cc6c0(param_1: *mut core::ffi::c_void, param_2: *mut core::ffi::c_void, param_3: u32);
    fn FUN_004cc590(param_1: *mut core::ffi::c_void);
    fn __strcmpi(param_1: *mut i8, param_2: *mut i8) -> i32;
    fn FUN_004cb0c0(param_1: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004bf8c0(param_1: *mut i8) -> u32 {
    // SAFETY: the callees and globals are the game's own; param_1 and the
    // list structure returned by FUN_004dfa10 are read where the original does.
    unsafe {
        FUN_004ccf50();
        let puVar1 = FUN_004dfa10(&raw mut s___plt_0052b35c);
        FUN_004ccf70();
        if puVar1.is_null() {
            return 0;
        }
        if param_1.is_null() {
            FUN_004dfc30(puVar1);
            return 0;
        }
        if param_1.read() == 0 {
            FUN_004dfc30(puVar1);
            return 0;
        }
        let mut puVar5 = puVar1.read_unaligned() as *mut u32;
        let mut iVar4: i32 = 0;
        let mut local_4: u32 = 0;
        if 0 < (puVar1.byte_add(4).read_unaligned() as i32) {
            loop {
                let pFVar2 =
                    FUN_004cc4a0(puVar5.read_unaligned() as *mut i8, &raw mut DAT_0051a854);
                if !pFVar2.is_null() {
                    FUN_004cc6c0(pFVar2, (&raw mut DAT_00b69d20).cast(), 0xe);
                    FUN_004cc590(pFVar2);
                    let iVar3 = __strcmpi(&raw mut DAT_00b69d20, param_1);
                    if iVar3 == 0 {
                        iVar4 = FUN_004cb0c0(puVar5.read_unaligned() as *mut i8);
                        if iVar4 != 0 {
                            local_4 = 1;
                        }
                        break;
                    }
                }
                puVar5 = puVar5.byte_add(4).read_unaligned() as *mut u32;
                iVar4 = iVar4.wrapping_add(1);
                if !(iVar4 < (puVar1.byte_add(4).read_unaligned() as i32)) {
                    break;
                }
            }
        }
        FUN_004dfc30(puVar1);
        local_4
    }
}
