// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a38: i32;
    static mut DAT_007091a0: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_007394e4: i32;
    static mut DAT_006db71c: i32;
    static mut DAT_00732940: i32;
    static mut DAT_0073abd0: i32;
    static mut DAT_006db798: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004230f0() {
    // SAFETY: all globals and the addresses computed from them are the game's
    // own memory, read and written exactly where the original does.
    unsafe {
        let mut i_var1: i32 = 0;
        if 0 < DAT_00732a38 {
            let mut i_var3: i32 = 0;
            let mut i_var2: i32 = 0;
            loop {
                let i_var5: i32 = DAT_007091a0.wrapping_add(i_var3);
                i_var2 = i_var2.wrapping_add(0x23);
                i_var3 = i_var3.wrapping_add(0xaf);
                i_var1 = i_var1.wrapping_add(1);
                let addr = DAT_006d4988.wrapping_add(-4).wrapping_add(i_var2);
                (addr as usize as *mut i32).write_unaligned(i_var5);
                if !(i_var1 < DAT_00732a38) {
                    break;
                }
            }
        }
        i_var1 = 0;
        let mut i_var2: i32 = 0;
        if 0 < DAT_00732a38 {
            let mut pi: *mut i32 = &raw mut DAT_007394e4;
            loop {
                let m1: i32 = pi.wrapping_sub(1).read_unaligned();
                if m1 == -1 {
                    let p0: i32 = pi.read_unaligned();
                    if p0 == -1 {
                        let p1: i32 = pi.wrapping_add(1).read_unaligned();
                        if p1 != -1 {
                            let dst = DAT_006db71c.wrapping_add(p1.wrapping_mul(0x4c));
                            let a = DAT_007091a0.wrapping_add(0xab).wrapping_add(i_var1);
                            (a as usize as *mut i32).write_unaligned(dst);
                        }
                    } else {
                        let dst = p0.wrapping_mul(0x436).wrapping_add(DAT_00732940);
                        let a = DAT_007091a0.wrapping_add(0xa7).wrapping_add(i_var1);
                        (a as usize as *mut i32).write_unaligned(dst);
                        let v: i32 = (&raw mut DAT_0073abd0)
                            .wrapping_offset(p0 as isize)
                            .read_unaligned();
                        if v != -1 {
                            let a2 = DAT_007091a0.wrapping_add(0xa7).wrapping_add(i_var1);
                            let t: i32 = (a2 as usize as *const i32).read_unaligned();
                            let dst2 = v.wrapping_mul(0x23).wrapping_add(DAT_006d4988);
                            (t.wrapping_add(0x432) as usize as *mut i32).write_unaligned(dst2);
                        }
                    }
                } else {
                    let dst = DAT_006db798.wrapping_add(m1.wrapping_mul(8));
                    let a = DAT_007091a0.wrapping_add(0xa3).wrapping_add(i_var1);
                    (a as usize as *mut i32).write_unaligned(dst);
                }
                pi = pi.wrapping_add(3);
                i_var1 = i_var1.wrapping_add(0xaf);
                i_var2 = i_var2.wrapping_add(1);
                if !(i_var2 < DAT_00732a38) {
                    break;
                }
            }
        }
    }
}
