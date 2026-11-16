// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7fb7: i32;
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00aa7fcf: i32;
    static mut DAT_00aa7fc3: i32;
    static mut DAT_00aa7fcb: i32;
    static mut DAT_00aa6cfc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004df1b0(param_1: *mut i8) -> i32;
    fn FUN_004d48e0() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004de9a0(param_1: *mut i8) -> i32 {
    // SAFETY: globals and callees are the game's own; param_1 and the table
    // at DAT_00aa7fc3 are game memory accessed exactly as the original does.
    unsafe {
        if DAT_00aa7fb7 == 0 {
            return 0;
        }
        if param_1.read() == 0 {
            return 0;
        }
        let i_var2 = FUN_004df1b0(param_1);
        let c_var1 = DAT_00aa7b0d;
        if i_var2 == -1 {
            return 0;
        }
        let mut i_var6: i32 = -1;
        let mut i_var5: i32 = DAT_00aa7fcf.wrapping_add(1);
        let mut i_var4: i32 = 0;
        let mut p = DAT_00aa7fc3.wrapping_add(4) as u32;
        loop {
            let a = (p.wrapping_sub(4) as usize as *const i32).read_unaligned();
            let b = (p as usize as *const i32).read_unaligned();
            if a == i_var2 && b < i_var5 {
                i_var5 = b;
                i_var6 = i_var4;
            }
            p = p.wrapping_add(12);
            i_var4 = i_var4.wrapping_add(1);
            if i_var4 >= 0xc {
                break;
            }
        }
        if i_var6 == -1 {
            return 0;
        }
        let slot = (DAT_00aa7fc3 as u32)
            .wrapping_add(8)
            .wrapping_add((i_var6 as u32).wrapping_mul(0xc));
        let pi_var3 = (slot as usize as *const i32).read_unaligned() as u32;
        if pi_var3 == 0 {
            return 0;
        }
        FUN_004d4970();
        let vt = (pi_var3 as usize as *const u32).read_unaligned();
        let f1: unsafe extern "C" fn(u32) -> i32 =
            core::mem::transmute(vt.wrapping_add(0x120) as usize as *const core::ffi::c_void);
        let r = f1(pi_var3);
        let vt2 = (pi_var3 as usize as *const u32).read_unaligned();
        let f2: unsafe extern "C" fn(u32) =
            core::mem::transmute(vt2.wrapping_add(0x20) as usize as *const core::ffi::c_void);
        f2(pi_var3);
        let base = DAT_00aa7fc3 as u32;
        let off = (i_var6 as u32).wrapping_mul(0xc);
        (base.wrapping_add(8).wrapping_add(off) as usize as *mut i32).write_unaligned(0);
        (base.wrapping_add(off) as usize as *mut i32).write_unaligned(-1);
        DAT_00aa7fcb = DAT_00aa7fcb.wrapping_sub(1);
        if c_var1 != 0 {
            DAT_00aa6cfc = FUN_004d48e0();
        }
        (r > -1) as i32
    }
}
