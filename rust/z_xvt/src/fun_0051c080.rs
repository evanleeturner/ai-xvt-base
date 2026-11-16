// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00742460: [u16; 0x10000];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051c080(param_1: *mut u16, param_2: u32, param_3: i32) {
    // SAFETY: the table is the game's own memory; param_1 points at the
    // caller's buffer of at least param_3 halfwords, written in order.
    unsafe {
        let u_var1: u16 = (&raw const DAT_00742460)
            .cast::<u16>()
            .add((param_2 & 0xffff) as usize)
            .read();
        let mut p = param_1;
        let mut n = param_3;
        if n != 0 {
            loop {
                p.write_unaligned(u_var1);
                p = p.add(1);
                n = n.wrapping_sub(1);
                if n == 0 {
                    break;
                }
            }
        }
    }
}
