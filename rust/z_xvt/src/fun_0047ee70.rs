// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732948: *mut core::ffi::c_void;
    static mut DAT_0070ab20: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "fclose"]
    fn _fclose(param_1: *mut core::ffi::c_void) -> i32;
    fn FID_conflict__remove(param_1: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0047ee70(param_1: i16) -> i32 {
    // SAFETY: the globals and callees are the game's own; DAT_00732948 points
    // at a live FILE whose flags word sits at offset 0xc.
    unsafe {
        let mut eax: i32;
        let mut cx: u16 = 0;

        let flags = DAT_00732948
            .cast::<u8>()
            .byte_add(0xc)
            .cast::<i32>()
            .read_unaligned();
        if (flags & 0x20) == 0 {
            eax = DAT_00732948 as usize as i32;
            cx = 1;
        } else {
            eax = _fclose(DAT_00732948);
            if eax != -1 {
                eax = DAT_00732948 as usize as i32;
                cx = 1;
            }
        }

        let mut high16: u16 = ((eax as u32) >> 16) as u16;

        if param_1 != 0 && cx != 0 {
            eax = FID_conflict__remove((&raw mut DAT_0070ab20).cast::<i8>());
            high16 = ((eax as u32) >> 16) as u16;
        }

        (((high16 as u32) << 16) | (cx as u32)) as i32
    }
}
