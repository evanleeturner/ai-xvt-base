// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527028: u32;
    static mut DAT_0052702c: u32;
    static mut DAT_00527024: u32;
    static mut DAT_005599a8: [u32; 0];
    static mut DAT_009994f8: u32;
    static mut DAT_0099f6e0: u32;
    static mut DAT_00999be8: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FID_conflict__memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: u32);
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00470060() -> u32 {
    // SAFETY: all globals and callees are the game's own memory and
    // functions; the array indices mirror the original's arithmetic.
    unsafe {
        let mut i_var3: i32 = 0;
        let mut u_var1: u32 = DAT_00527028;
        let mut u_var5: u32 = DAT_00527028;
        let base = (&raw mut DAT_005599a8).cast::<u32>();
        while DAT_0052702c > 0 {
            let src = base.add(u_var5.wrapping_mul(0x84) as usize);
            let v = src.read();
            let mut u_var4 = u_var1;
            if v == 0 || DAT_009994f8 == src.read() {
                u_var4 = u_var1.wrapping_add(1);
                let dst = base.add(u_var1.wrapping_mul(0x84) as usize);
                FID_conflict__memcpy(
                    dst.cast::<core::ffi::c_void>(),
                    src.cast::<core::ffi::c_void>(),
                    0x210,
                );
                if u_var4 > 0x3ff {
                    u_var4 = 0;
                }
                i_var3 = i_var3.wrapping_add(1);
            }
            u_var5 = u_var5.wrapping_add(1);
            if u_var5 > 0x3ff {
                u_var5 = 0;
            }
            u_var1 = u_var4;
            DAT_0052702c = DAT_0052702c.wrapping_sub(1);
        }
        u_var5 = 0;
        DAT_00527024 = u_var1;
        DAT_0052702c = i_var3 as u32;
        if DAT_0099f6e0 != 0 {
            let mut p = (&raw mut DAT_00999be8).cast::<i32>();
            loop {
                if p.read() != DAT_009994f8 as i32 {
                    p.add(1).write(p.add(2).read());
                    p.add(3).write(p.add(4).read());
                    p.add(5).write(p.add(6).read());
                    let d = timeGetTime();
                    p.add(0x89).write(d as i32);
                }
                p = p.add(0x8e);
                u_var5 = u_var5.wrapping_add(1);
                if !(u_var5 < DAT_0099f6e0) {
                    break;
                }
            }
        }
        1
    }
}
