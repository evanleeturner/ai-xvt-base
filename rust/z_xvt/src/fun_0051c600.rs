// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536704: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f3c0(param_1: u32, param_2: u32) -> i16;
    fn FUN_0048f4f0(param_1: u32) -> *mut core::ffi::c_void;
    fn FUN_0048f510(param_1: u32);
    fn FUN_0051d220(param_1: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051c600(param_1: *mut u32) -> u32 {
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // 0x10 dwords to copy, and the allocated block is at least 0x4a bytes.
    unsafe {
        if DAT_00536704 == 0 {
            return 0;
        }
        let s_var1 = FUN_0048f3c0(0x4a, 2);
        if s_var1 == 0 {
            return 0;
        }
        let puVar2 = FUN_0048f4f0(s_var1 as u16 as u32) as *mut u32;
        let mut src = param_1;
        let mut dst = puVar2;
        let mut i_var3: i32 = 0x10;
        while i_var3 != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            i_var3 -= 1;
        }
        let p_bytes = puVar2 as *mut u8;
        p_bytes.byte_add(0x40).cast::<u16>().write_unaligned(0);
        p_bytes.byte_add(0x42).cast::<u32>().write_unaligned(0);
        p_bytes.byte_add(0x46).write(0);
        p_bytes.byte_add(0x47).write(0);
        p_bytes.byte_add(0x48).cast::<u16>().write_unaligned(0);
        FUN_0048f510(puVar2 as u32);
        FUN_0051d220(s_var1);
        1
    }
}
