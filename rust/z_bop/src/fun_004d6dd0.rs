// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc6c0(param_1: i32, param_2: *mut core::ffi::c_void, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d6dd0(param_1: i32, mut param_2: *mut u32, param_3: i32) {
    let mut buf: [u8; 4] = [0; 4];
    let mut p = param_2;
    let mut param_3 = param_3;
    // SAFETY: param_2 points at the caller's 0x100-dword buffer; the callee is
    // the game's own and reads into our local 4-byte buffer.
    unsafe {
        let mut i: i32 = 0x100;
        while i != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i = i.wrapping_sub(1);
        }
        if 0 < param_3 {
            loop {
                FUN_004cc6c0(param_1, (&raw mut buf).cast::<core::ffi::c_void>(), 4);
                param_3 = param_3.wrapping_sub(1);
                let b = param_2.cast::<u8>();
                b.write(buf[2]);
                b.add(1).write(buf[1]);
                b.add(2).write(buf[0]);
                b.add(3).write(0);
                param_2 = param_2.add(1);
                if param_3 == 0 {
                    break;
                }
            }
        }
    }
}
