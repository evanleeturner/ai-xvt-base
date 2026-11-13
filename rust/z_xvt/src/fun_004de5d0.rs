// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759aec: i32;
    static mut DAT_00759645: i8;
    static mut DAT_00759b04: i32;
    static mut DAT_00759af8: *mut u8;
    static mut DAT_00759b00: i32;
    static mut DAT_00758830: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dee40(param_1: *mut i8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4d90();
    fn FUN_004d4d00() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004de5d0(param_1: *mut i8) -> u8 {
    // SAFETY: globals and callees are the game's own; param_1 and the table
    // and object pointers are read and written exactly as the original does.
    unsafe {
        if DAT_00759aec == 0 {
            return 0;
        }
        if param_1.read() == 0 {
            return 0;
        }
        let i_var2 = FUN_004dee40(param_1);
        let c_var1 = DAT_00759645;
        if i_var2 == -1 {
            return 0;
        }
        let mut i_var4: i32 = -1;
        let mut i_var6: i32 = DAT_00759b04.wrapping_add(1);
        let mut i_var5: i32 = 0;
        let base = DAT_00759af8;
        loop {
            let p = base.byte_add((i_var5 as usize).wrapping_mul(12));
            let a = p.cast::<i32>().read_unaligned();
            let b = p.byte_add(4).cast::<i32>().read_unaligned();
            if a == i_var2 && b < i_var6 {
                i_var4 = i_var5;
                i_var6 = b;
            }
            i_var5 = i_var5.wrapping_add(1);
            if i_var5 >= 0xc {
                break;
            }
        }
        if i_var4 == -1 {
            return 0;
        }
        let off = (i_var4 as usize).wrapping_mul(12);
        let obj = DAT_00759af8
            .byte_add(8)
            .byte_add(off)
            .cast::<u32>()
            .read_unaligned();
        if obj == 0 {
            return 0;
        }
        FUN_004d4d90();
        let vt = (obj as *const u32).read_unaligned();
        let f1: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32 =
            core::mem::transmute(((vt.wrapping_add(0x48)) as *const u32).read_unaligned() as usize);
        let i_var2 = f1(obj as *mut core::ffi::c_void);
        let vt = (obj as *const u32).read_unaligned();
        let f2: unsafe extern "C" fn(*mut core::ffi::c_void) =
            core::mem::transmute(((vt.wrapping_add(8)) as *const u32).read_unaligned() as usize);
        f2(obj as *mut core::ffi::c_void);
        DAT_00759af8
            .byte_add(8)
            .byte_add(off)
            .cast::<i32>()
            .write_unaligned(0);
        DAT_00759af8.byte_add(off).cast::<i32>().write_unaligned(-1);
        DAT_00759b00 = DAT_00759b00.wrapping_sub(1);
        if c_var1 != 0 {
            DAT_00758830 = FUN_004d4d00();
        }
        (-1 < i_var2) as u8
    }
}
