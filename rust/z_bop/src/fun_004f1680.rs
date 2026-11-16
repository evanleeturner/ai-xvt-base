// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6150: u32;
    static mut DAT_00aa615e: u32;
    static mut DAT_00aa62aa: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f1680() -> u32 {
    // SAFETY: the globals are the game's own table; every pointer stays
    // within the 0x2a-byte records the original walks, read and written
    // in the same order.
    unsafe {
        let base_6150 = (&raw mut DAT_00aa6150) as usize;
        let base_615e = (&raw mut DAT_00aa615e) as usize;
        let limit = ((&raw mut DAT_00aa62aa) as usize).wrapping_add(3);

        let mut i_var4: i32 = 0;
        let mut p5: usize = base_615e;
        loop {
            let mut u2: u32 = 0;
            let go_body = if (p5 as *const u32).read_unaligned() != 0 {
                true
            } else {
                u2 = i_var4.wrapping_add(1) as u32;
                7 < u2
            };
            if !go_body {
                let mut p1: usize = base_615e.wrapping_add((u2 as usize).wrapping_mul(0x2a));
                loop {
                    if (p1 as *const u32).read_unaligned() == 0 {
                        p1 = p1.wrapping_add(0x2a);
                        u2 = u2.wrapping_add(1);
                        if limit < p1 {
                            break;
                        }
                        continue;
                    }
                    let p8 = base_6150.wrapping_add((u2 as usize).wrapping_mul(0x2a));
                    let mut src = p8;
                    let mut dst = p5.wrapping_sub(0xe);
                    let mut n = 10;
                    while n != 0 {
                        (dst as *mut u32).write_unaligned((src as *const u32).read_unaligned());
                        src = src.wrapping_add(4);
                        dst = dst.wrapping_add(4);
                        n -= 1;
                    }
                    (dst as *mut u16).write_unaligned((src as *const u16).read_unaligned());
                    let mut z = p8;
                    let mut n = 10;
                    while n != 0 {
                        (z as *mut u32).write_unaligned(0);
                        z = z.wrapping_add(4);
                        n -= 1;
                    }
                    (z as *mut u16).write_unaligned(0);
                    break;
                }
            }
            p5 = p5.wrapping_add(0x2a);
            i_var4 = i_var4.wrapping_add(1);
            if limit < p5 {
                return 1;
            }
        }
    }
}
