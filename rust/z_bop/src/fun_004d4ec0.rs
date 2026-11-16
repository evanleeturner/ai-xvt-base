// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b2e: u32;
    static mut DAT_00aa7b32: u32;
    static mut DAT_00aa6cfc: u32;
    static mut DAT_00aa7b92: u32;
    static mut DAT_00aa7b1b: u32;
    static mut DAT_00aa7b13: u32;
    static mut DAT_00aa7b17: u32;
    static mut DAT_00aa7b0d: u32;
    static mut DAT_00aa7ac5: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d48e0() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d4ec0(param_1: i32) -> u32 {
    // SAFETY: the globals and the callee are the game's own memory; the
    // pointers copied between are the game's buffers, and the call goes
    // through the address the original computes.
    unsafe {
        if DAT_00aa7b2e == 0 {
            return 0;
        }
        if DAT_00aa7b32 == 0 {
            return 0;
        }
        if DAT_00aa6cfc != 0 && DAT_00aa7b92 != 0 && param_1 != 0 {
            let mut src = DAT_00aa6cfc as *const u32;
            let mut dst = DAT_00aa7b92 as *mut u32;
            let mut count = (DAT_00aa7b1b.wrapping_mul(0xf) & 0x7ffffff) << 3;
            while count != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.add(1);
                dst = dst.add(1);
                count -= 1;
            }
        }
        let addr = (DAT_00aa7b32 as *const u32)
            .read_unaligned()
            .wrapping_add(0x80);
        let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void, i32) =
            core::mem::transmute(addr as usize as *const core::ffi::c_void);
        f(DAT_00aa7b32 as usize as *mut core::ffi::c_void, 0);
        DAT_00aa7b13 = DAT_00aa7b17;
        if DAT_00aa7b0d != 0 {
            DAT_00aa6cfc = DAT_00aa7ac5;
            return 1;
        }
        DAT_00aa6cfc = FUN_004d48e0();
        1
    }
}
