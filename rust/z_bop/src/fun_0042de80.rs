// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0fe80: i32;
    static mut DAT_00a0aba0: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_00a0a7f0: i32;
    static mut DAT_009a20a4: i32;
    static mut DAT_00a0aa10: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042de80() {
    // SAFETY: the globals are the game's own memory; the writes cover the
    // same ranges, in the same order, as the original.
    unsafe {
        let mut p = (&raw mut DAT_00a0fe80).cast::<u32>();
        let mut i: i32 = 0x1a3;
        while i != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i = i.wrapping_sub(1);
        }
        let mut u: u32 = 0;
        loop {
            let mut q = (&raw mut DAT_00a0aba0)
                .cast::<u8>()
                .byte_add(u as usize)
                .cast::<u32>();
            u = u.wrapping_add(0x61);
            let mut n: i32 = 0x18;
            while n != 0 {
                q.write_unaligned(0);
                q = q.add(1);
                n = n.wrapping_sub(1);
            }
            q.cast::<u8>().write(0);
            if u >= 0x246 {
                break;
            }
        }
        if DAT_009a1fe0 != 0 {
            let mut r = (&raw mut DAT_00a0a7f0).cast::<u32>();
            let mut m: i32 = DAT_009a1fe0;
            while m != 0 {
                r.write_unaligned(0);
                r = r.add(1);
                m = m.wrapping_sub(1);
            }
        }
        DAT_009a20a4 = 0;
        DAT_00a0aa10 = 0;
    }
}
