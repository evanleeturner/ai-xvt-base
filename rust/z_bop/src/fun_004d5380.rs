// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6cfc: u32;
    static mut DAT_00aa7b17: u32;
    static mut DAT_00aa7b1f: u32;
    static mut DAT_00aa7b23: i8;
    static mut DAT_0051a854: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    #[link_name = "FID_conflict___wfopen"]
    fn FID_conflict___wfopen(filename: *const i8, mode: *const i8) -> *mut core::ffi::c_void;
    fn _fclose(stream: *mut core::ffi::c_void) -> i32;
    fn FUN_004d48e0() -> u32;
    fn FUN_004b7b10(
        param_1: *const i8,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
        param_7: u32,
        param_8: u32,
    ) -> u32;
    fn FUN_004d4970();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d5380() -> u32 {
    let mut local_40: [i8; 64] = [0; 64];
    let mut i_var2: i32 = 0;
    // SAFETY: the callees and globals are the game's own; local_40 is a
    // 64-byte buffer, passed as the original does.
    unsafe {
        loop {
            _sprintf(
                local_40.as_mut_ptr(),
                b"frontscreen%d.bmp\0".as_ptr().cast::<i8>(),
                i_var2,
            );
            let file =
                FID_conflict___wfopen(local_40.as_ptr(), (&raw const DAT_0051a854).cast::<i8>());
            if file.is_null() {
                break;
            }
            i_var2 = i_var2.wrapping_add(1);
            _fclose(file);
        }
        DAT_00aa6cfc = FUN_004d48e0();
        let a = (&raw const DAT_00aa7b17).read_unaligned();
        let b = (&raw const DAT_00aa7b1f).read_unaligned();
        let c = DAT_00aa7b23 as u32;
        let u_var1 = FUN_004b7b10(
            local_40.as_ptr(),
            DAT_00aa6cfc,
            0x280,
            0x1e0,
            a,
            b,
            c,
            0xaa7b9e,
        );
        FUN_004d4970();
        u_var1
    }
}
