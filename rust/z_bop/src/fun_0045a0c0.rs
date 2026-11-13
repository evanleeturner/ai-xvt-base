// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07ce8: i32;
    static mut DAT_009d6820: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_0099f9a4: i32;
    static mut DAT_009a8da0: i32;
    static mut DAT_00a07bd0: i32;
    static mut DAT_009a1090: [i32; 1];
    static mut DAT_009a8e18: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0045a0c0() {
    // SAFETY: all globals and the memory they point into belong to the game;
    // reads and writes happen at the same offsets and in the same order as
    // the original.
    unsafe {
        let mut i_var1: i32 = 0;
        if 0 < DAT_00a07ce8 {
            let mut i_var3: i32 = 0;
            let mut i_var2: i32 = 0;
            loop {
                let i_var4: i32 = DAT_009d6820.wrapping_add(i_var3);
                i_var2 = i_var2.wrapping_add(0x23);
                i_var3 = i_var3.wrapping_add(0xb1);
                i_var1 = i_var1.wrapping_add(1);
                let addr = DAT_009a1fe8.wrapping_sub(4).wrapping_add(i_var2);
                (addr as *mut i32).write_unaligned(i_var4);
                if !(i_var1 < DAT_00a07ce8) {
                    break;
                }
            }
        }
        let mut i_var2: i32 = 0;
        i_var1 = 0;
        if 0 < DAT_00a07ce8 {
            let mut pi_var5: *mut i32 = (&raw mut DAT_0099f9a4).cast::<i32>();
            let arr: *mut i32 = (&raw mut DAT_009a1090).cast::<i32>();
            loop {
                let prev = pi_var5.offset(-1).read_unaligned();
                let cur = pi_var5.read_unaligned();
                if prev == -1 {
                    if cur == -1 {
                        let next = pi_var5.offset(1).read_unaligned();
                        if next != -1 {
                            let rhs = DAT_009a8da0.wrapping_add(next.wrapping_mul(0x4c));
                            let addr = DAT_009d6820.wrapping_add(0xad).wrapping_add(i_var2);
                            (addr as *mut i32).write_unaligned(rhs);
                        }
                    } else {
                        let rhs2 = cur.wrapping_mul(0x462).wrapping_add(DAT_00a07bd0);
                        let addr = DAT_009d6820.wrapping_add(0xa9).wrapping_add(i_var2);
                        (addr as *mut i32).write_unaligned(rhs2);
                        let entry = arr.offset(cur as isize).read_unaligned();
                        if entry != -1 {
                            let addr2 = DAT_009d6820.wrapping_add(0xa9).wrapping_add(i_var2);
                            let inner_val = (addr2 as *const i32).read_unaligned();
                            let entry2 = arr.offset(cur as isize).read_unaligned();
                            let rhs3 = entry2.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
                            (inner_val.wrapping_add(0x45e) as *mut i32).write_unaligned(rhs3);
                        }
                    }
                } else {
                    let rhs4 = DAT_009a8e18.wrapping_add(prev.wrapping_mul(10));
                    let addr = DAT_009d6820.wrapping_add(0xa5).wrapping_add(i_var2);
                    (addr as *mut i32).write_unaligned(rhs4);
                }
                pi_var5 = pi_var5.add(3);
                i_var2 = i_var2.wrapping_add(0xb1);
                i_var1 = i_var1.wrapping_add(1);
                if !(i_var1 < DAT_00a07ce8) {
                    break;
                }
            }
        }
    }
}
