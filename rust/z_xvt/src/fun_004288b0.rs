// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536f25: u32;
    static mut DAT_00536f29: u32;
    static mut DAT_00536f31: *mut u8;
    static mut DAT_00536f35: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _malloc(param_1: u32) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004288b0(
    param_1: u8,
    param_2: u8,
    param_3: u32,
    param_4: u8,
    param_5: u8,
    param_6: u8,
) -> *mut u8 {
    // SAFETY: the globals and malloc are the game's own; the pool block is
    // 0x29004 bytes and each record written lies within it as the original does.
    unsafe {
        if DAT_00536f29 == 0 {
            let p = _malloc(0x29004) as *mut u8;
            if p.is_null() {
                return core::ptr::null_mut();
            }
            p.byte_add(0x29000)
                .cast::<*mut u8>()
                .write_unaligned(DAT_00536f35);
            DAT_00536f29 = 0x800;
            DAT_00536f31 = p;
            DAT_00536f35 = p;
        }
        let p2 = DAT_00536f31;
        DAT_00536f25 = DAT_00536f25.wrapping_add(1);
        DAT_00536f29 = DAT_00536f29.wrapping_sub(1);
        DAT_00536f31 = DAT_00536f31.byte_add(0x52);
        p2.byte_add(0x2e).cast::<u32>().write_unaligned(param_3);
        let mut p4 = p2.byte_add(0x32);
        let mut i: i32 = 8;
        while i != 0 {
            p4.cast::<u32>().write_unaligned(0);
            p4 = p4.byte_add(4);
            i -= 1;
        }
        *p2 = param_1;
        *p2.byte_add(1) = param_2;
        *p2.byte_add(2) = 0;
        *p2.byte_add(3) = param_4;
        *p2.byte_add(4) = param_5;
        *p2.byte_add(5) = param_6;
        p2.byte_add(0xe).cast::<u32>().write_unaligned(0);
        p2.byte_add(0x12).cast::<u32>().write_unaligned(0);
        p2.byte_add(10).cast::<u32>().write_unaligned(0);
        p2.byte_add(0x16).cast::<u32>().write_unaligned(0);
        p2.byte_add(0x1a).cast::<u32>().write_unaligned(0);
        p2.byte_add(0x1e).cast::<u32>().write_unaligned(0);
        p2.byte_add(0x22).cast::<u32>().write_unaligned(0);
        p2.byte_add(0x26).cast::<u32>().write_unaligned(0);
        p2.byte_add(0x2a).cast::<u32>().write_unaligned(0);
        p2
    }
}
