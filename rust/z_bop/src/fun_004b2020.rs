// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00664c80: *mut i8;
    static mut DAT_00528c98: *mut *mut c_void;
    static mut DAT_00528cc0: i32;
    static mut DAT_00664548: *mut *mut c_void;
    static mut DAT_00664974: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b1600(param_1: i32, param_2: *mut i32, param_3: i32) -> *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004079e0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b2020() -> i32 {
    let mut local_34: [i32; 12] = [0; 12];
    // SAFETY: the globals, the vtables they point to and the callees are the
    // game's own; the output buffer is a valid, writable cursor, and the
    // vtable slots are read and called where the original does.
    unsafe {
        let p = DAT_00664c80;
        p.write(0xb);
        p.byte_add(1).write(0);
        p.byte_add(2).cast::<i16>().write_unaligned(0);
        DAT_00664c80 = DAT_00664c80.wrapping_byte_add(4);

        let vt = DAT_00528c98.read_unaligned() as *const u8;
        let f14 = vt.byte_add(0x14).cast::<*const c_void>().read_unaligned();
        let f14: unsafe extern "C" fn(*mut *mut c_void) -> i32 = core::mem::transmute(f14);
        let mut i_var1 = f14(DAT_00528c98);
        if i_var1 != 0 {
            FUN_004b1600(i_var1, &raw mut DAT_00528cc0, 0x79);
            FUN_004079e0();
            return 0;
        }

        let mut i: i32 = 0;
        while i < 12 {
            local_34[i as usize] = 0;
            i += 1;
        }

        let vt = DAT_00528c98.read_unaligned() as *const u8;
        let f18 = vt.byte_add(0x18).cast::<*const c_void>().read_unaligned();
        let f18: unsafe extern "C" fn(*mut *mut c_void, *mut i32) = core::mem::transmute(f18);
        f18(DAT_00528c98, local_34.as_mut_ptr());

        let vt2 = DAT_00664548.read_unaligned() as *const u8;
        let f20 = vt2.byte_add(0x20).cast::<*const c_void>().read_unaligned();
        let f20: unsafe extern "C" fn(*mut *mut c_void, *mut *mut c_void, *mut c_void, i32) -> i32 =
            core::mem::transmute(f20);
        i_var1 = f20(DAT_00664548, DAT_00528c98, DAT_00664974, 2);
        if i_var1 != 0 {
            FUN_004b1600(i_var1, &raw mut DAT_00528cc0, 0x79);
            FUN_004079e0();
            return 0;
        }
        1
    }
}
